#![allow(dead_code)]

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone)]
pub struct StubResponse {
    status_code: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl StubResponse {
    pub fn with_content_type(status_code: u16, body: Vec<u8>, content_type: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), content_type.to_string());
        Self {
            status_code,
            headers,
            body,
        }
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct CapturedRequest {
    pub method: String,
    pub target: String,
    pub headers: HashMap<String, String>,
}

pub struct StubServer {
    base_url: String,
    captured_request: Arc<Mutex<Option<CapturedRequest>>>,
    handle: Option<JoinHandle<Result<(), String>>>,
}

impl StubServer {
    pub fn serve_once(response: StubResponse) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub server");
        let address = listener.local_addr().expect("stub local address");
        let captured_request = Arc::new(Mutex::new(None));
        let captured_request_clone = Arc::clone(&captured_request);

        let handle = thread::spawn(move || {
            let (stream, _) = listener.accept().map_err(|error| error.to_string())?;
            handle_request(stream, response, captured_request_clone)
        });

        Self {
            base_url: format!("http://{address}"),
            captured_request,
            handle: Some(handle),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn finish(mut self) -> Option<CapturedRequest> {
        let handle = self.handle.take().expect("stub server handle");
        let result = handle.join().expect("stub server thread panic");
        result.expect("stub server failed");
        self.captured_request
            .lock()
            .expect("captured request mutex")
            .clone()
    }
}

fn handle_request(
    mut stream: TcpStream,
    response: StubResponse,
    captured_request: Arc<Mutex<Option<CapturedRequest>>>,
) -> Result<(), String> {
    let raw = read_http_request(&mut stream)?;
    let captured = parse_request(&raw)?;
    *captured_request
        .lock()
        .map_err(|_| "captured request mutex poisoned".to_string())? = Some(captured);

    let mut response_head = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nConnection: close\r\n",
        response.status_code,
        reason_phrase(response.status_code),
        response.body.len()
    );
    for (name, value) in response.headers {
        response_head.push_str(&format!("{name}: {value}\r\n"));
    }
    response_head.push_str("\r\n");

    stream
        .write_all(response_head.as_bytes())
        .map_err(|error| error.to_string())?;
    stream
        .write_all(&response.body)
        .map_err(|error| error.to_string())?;
    stream.flush().map_err(|error| error.to_string())?;

    Ok(())
}

fn read_http_request(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        let read = stream.read(&mut chunk).map_err(|error| error.to_string())?;
        if read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..read]);
        if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
        if buffer.len() > 64 * 1024 {
            return Err("request head is too large".to_string());
        }
    }
    Ok(buffer)
}

fn parse_request(raw: &[u8]) -> Result<CapturedRequest, String> {
    let text = String::from_utf8_lossy(raw);
    let mut lines = text.split("\r\n");
    let request_line = lines
        .next()
        .ok_or_else(|| "missing request line".to_string())?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts
        .next()
        .ok_or_else(|| "missing request method".to_string())?
        .to_string();
    let target = request_parts
        .next()
        .ok_or_else(|| "missing request target".to_string())?
        .to_string();

    let mut headers = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }

    Ok(CapturedRequest {
        method,
        target,
        headers,
    })
}

fn reason_phrase(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        400 => "Bad Request",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Stub Response",
    }
}

pub fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(path).expect("fixture file")
}

pub fn fixture_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("fixture file")
}
