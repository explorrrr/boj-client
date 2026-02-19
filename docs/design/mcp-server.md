# BOJ MCP Server Design Notes

- Created: 2026-02-19
- Target repository: `boj-client`
- Scope: `mcp-server/` workspace member (`boj-mcp-server` binary)

## 1. Goal

Expose `boj-client` as an MCP server over `stdio` so MCP clients can call BOJ endpoints through stable tool contracts.

## 2. Transport and Runtime

- Transport: MCP `stdio`
- Runtime: `tokio` (multi-thread)
- MCP SDK: `rmcp` (`tool_router` + `tool_handler`)

## 3. Tool Surface

The server exposes exactly three tools:

- `boj_get_data_code`
- `boj_get_data_layer`
- `boj_get_metadata`

### 3.1 Input policy

- Input mirrors existing `boj-client::query::*` semantics.
- `include_raw` defaults to `false`.
- `frequency` is required only for `boj_get_data_layer`.

### 3.2 Output policy

- Response is structured JSON.
- Paging is single-call only; when provided by BOJ, `next_position` is returned as-is.
- Raw decoded body is omitted unless `include_raw=true`.

## 4. Error Contract

`boj-client::error::BojError` is mapped to MCP `ErrorData`:

- `ValidationError` -> `invalid_params` with `error_type="VALIDATION_ERROR"`
- `TransportError` -> `internal_error` with `error_type="TRANSPORT_ERROR"`
- `DecodeError` -> `internal_error` with `error_type="DECODE_ERROR"`
- `ApiError` -> `invalid_params` only for `status=400`, otherwise `internal_error`, with `error_type="API_ERROR"`

The error `data` payload keeps diagnostic context (`status`, `message_id`, `message` when available).

## 5. Retry Policy

Server-side retries are intentionally minimal:

- Retry only `TransportError` and `ApiError` with status `500` or `503`.
- Backoff is exponential (`initial_backoff_ms * 2^n`).
- Retry count is configurable (`retry_max`), defaulting to `2`.

## 6. Configuration

Runtime config (CLI args and env vars):

- `--base-url` / `BOJ_BASE_URL` (default: `https://www.stat-search.boj.or.jp`)
- `--timeout-ms` / `BOJ_TIMEOUT_MS` (default: `10000`)
- `--retry-max` / `BOJ_RETRY_MAX` (default: `2`)
- `--retry-backoff-ms` / `BOJ_RETRY_BACKOFF_MS` (default: `200`)

## 7. Distribution

Distribution is split into two layers:

- Rust binary release assets (`boj-mcp-server-*`)
- npm launcher package (`@explorrrr/boj-mcp-server`) for `npx`

`mcp-release.yml` runs via `workflow_dispatch` and validates version consistency across:

- workflow input `version`
- `mcp-server/Cargo.toml`
- `npm/boj-mcp-server/package.json`

The npm publish job in `mcp-release.yml` uses GitHub Actions OIDC trusted publishing and does not use long-lived npm tokens. The publish job must grant `id-token: write`.

Supported release targets:

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

Release assets follow fixed names:

- `boj-mcp-server-x86_64-unknown-linux-gnu.tar.gz`
- `boj-mcp-server-x86_64-apple-darwin.tar.gz`
- `boj-mcp-server-aarch64-apple-darwin.tar.gz`
- `boj-mcp-server-x86_64-pc-windows-msvc.tar.gz`
- `SHA256SUMS`

The npm launcher downloads the correct asset for the current platform on first run, validates it with `SHA256SUMS`, caches it locally, and then executes the binary over `stdio`.

## 8. Compatibility

- Existing library public API remains unchanged.
- MCP tool names are stable once released.
- Future additions should be additive and avoid breaking required parameters.
