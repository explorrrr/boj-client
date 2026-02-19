# boj-mcp-server

`boj-mcp-server` is an MCP server binary that exposes BOJ API access through `boj-client`.

## Tools

- `boj_get_data_code`
- `boj_get_data_layer`
- `boj_get_metadata`

## Run

```bash
cargo run -p boj-mcp-server
```

## Runtime options

- `--base-url` / `BOJ_BASE_URL` (default: `https://www.stat-search.boj.or.jp`)
- `--timeout-ms` / `BOJ_TIMEOUT_MS` (default: `10000`)
- `--retry-max` / `BOJ_RETRY_MAX` (default: `2`)
- `--retry-backoff-ms` / `BOJ_RETRY_BACKOFF_MS` (default: `200`)

## Behavior notes

- Responses are single-page. Use `next_position` for subsequent calls.
- `raw` is omitted unless `include_raw=true`.
