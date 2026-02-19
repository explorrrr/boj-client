# boj-mcp-server

`boj-mcp-server` is an MCP server binary that exposes BOJ API access through `boj-client`.
The repository also provides an `npx` launcher package: `@explorrrr/boj-mcp-server`.

## Tools

- `boj_get_data_code`
- `boj_get_data_layer`
- `boj_get_metadata`

## Run

```bash
cargo run -p boj-mcp-server
```

## Run with npx

```bash
npx -y @explorrrr/boj-mcp-server --help
```

## MCP client config example (npx)

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

## Launcher-specific environment variables

- `BOJ_MCP_SERVER_PATH`: force an existing local binary path
- `BOJ_MCP_CACHE_DIR`: override download cache location
- `BOJ_MCP_RELEASE_BASE_URL`: override release download base URL

## Runtime options

- `--base-url` / `BOJ_BASE_URL` (default: `https://www.stat-search.boj.or.jp`)
- `--timeout-ms` / `BOJ_TIMEOUT_MS` (default: `10000`)
- `--retry-max` / `BOJ_RETRY_MAX` (default: `2`)
- `--retry-backoff-ms` / `BOJ_RETRY_BACKOFF_MS` (default: `200`)

## Behavior notes

- Responses are single-page. Use `next_position` for subsequent calls.
- `raw` is omitted unless `include_raw=true`.

## Release workflow

Maintainers publish release assets through `.github/workflows/mcp-release.yml` using `workflow_dispatch`.
The workflow validates that these versions match:

- input `version`
- `mcp-server/Cargo.toml`
- `npm/boj-mcp-server/package.json`

When `publish_npm=true`, the workflow also publishes `@explorrrr/boj-mcp-server`.
