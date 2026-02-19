# @explorrrr/boj-mcp-server

`npx` launcher for the `boj-mcp-server` Rust MCP binary.

## Usage

Run directly with `npx`:

```bash
npx -y @explorrrr/boj-mcp-server --help
```

MCP client config example:

```json
{
  "mcpServers": {
    "boj": {
      "command": "npx",
      "args": ["-y", "@explorrrr/boj-mcp-server"]
    }
  }
}
```

## Runtime environment variables

- `BOJ_MCP_SERVER_PATH`: absolute/relative path to an existing `boj-mcp-server` binary. If set, no download is performed.
- `BOJ_MCP_CACHE_DIR`: cache directory for downloaded release binaries.
- `BOJ_MCP_RELEASE_BASE_URL`: release download base URL. Default: `https://github.com/explorrrr/boj-client/releases/download`

The launcher also passes through server runtime options such as:

- `BOJ_BASE_URL`
- `BOJ_TIMEOUT_MS`
- `BOJ_RETRY_MAX`
- `BOJ_RETRY_BACKOFF_MS`

## Supported targets

- `linux-x64`
- `darwin-x64`
- `darwin-arm64`
- `win32-x64`

## License

MIT
