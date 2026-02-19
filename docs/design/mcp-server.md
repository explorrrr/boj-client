# BOJ MCP Server Design Notes

- Created: 2026-02-19
- Updated: 2026-02-19
- Target repository: `boj-client`
- Scope: `mcp-server/` workspace member (`boj-mcp-server` binary)

## 1. Goal

Expose `boj-client` as an MCP server over `stdio` so MCP clients can call BOJ endpoints through stable tool contracts and discover usage patterns through MCP-native interfaces.

## 2. Transport and Runtime

- Transport: MCP `stdio`
- Runtime: `tokio` (multi-thread)
- MCP SDK: `rmcp`
- Internal layout:
  - `src/lib.rs`: server handlers and contracts
  - `src/main.rs`: process entrypoint only

## 3. Protocol and Capabilities

- Server protocol target: `2025-11-25`
- Enabled capabilities:
  - `tools`
  - `prompts`
  - `resources`
  - `completions`

Notes:

- rmcp currently negotiates protocol version against client-provided version.
- When peer supports `2025-11-25`, negotiated version is `2025-11-25`.

## 4. Tool Surface

The server exposes six tools:

- `boj_get_data_code`
- `boj_get_data_layer`
- `boj_get_metadata`
- `boj_list_databases`
- `boj_get_parameter_catalog`
- `boj_get_message_catalog`

All tools publish:

- explicit `outputSchema`
- `annotations` (`read_only`, `idempotent`, non-destructive hints)
- `execution.taskSupport = forbidden`

### 4.1 Input policy

- Input mirrors existing `boj-client::query::*` semantics.
- `include_raw` defaults to `false`.
- `frequency` is required only for `boj_get_data_layer`.
- Discovery tools are read-only and return static catalog data from `boj_client::catalog`.
- `boj_get_parameter_catalog` accepts endpoint scope (`all` / `get_data_code` / `get_data_layer` / `get_metadata`).
- `boj_get_message_catalog` accepts optional `status` filter.
- `format` and `lang` accept uppercase aliases (`JSON`/`CSV`, `JP`/`EN`) and are normalized internally.

### 4.2 Output policy

- Success payloads are structured JSON.
- Paging is single-call only; when provided by BOJ, `next_position` is returned as-is.
- Raw decoded body is omitted unless `include_raw=true`.
- Discovery responses include source metadata (`source_document`, `source_date`) for traceability.
- `boj_get_parameter_catalog` includes both BOJ notation (`format_codes`, `language_codes`) and MCP input-oriented lowercase notation (`mcp_format_codes`, `mcp_language_codes`).

## 5. Discovery Interfaces

### 5.1 Prompts

- `boj_discovery_flow`
- `boj_fetch_data_code_flow`
- `boj_latest_value_flow`

Purpose:

- Provide reusable procedural guidance as MCP prompt contracts.

### 5.2 Resources

Static resources:

- `boj://guide/call-order`
- `boj://guide/input-normalization`
- `boj://catalog/databases`

Resource templates:

- `boj://catalog/parameters/{endpoint}`
- `boj://catalog/messages/{status}`

### 5.3 Completion

`completion/complete` provides prefix completion for:

- `db`
- `format`
- `lang`
- `endpoint`
- `status`
- `frequency`

## 6. Error Contract

Tool execution failures are returned as `CallToolResult` with `isError=true` and structured payload:

- `error_type`
- `message`
- `retryable`
- optional `status`
- optional `message_id`

Classification:

- `ValidationError` -> `VALIDATION_ERROR`, non-retryable
- `TransportError` -> `TRANSPORT_ERROR`, retryable
- `DecodeError` -> `DECODE_ERROR`, non-retryable
- `ApiError` -> `API_ERROR`, retryable only for `500`/`503`

JSON-RPC errors are reserved for protocol-level failures.

## 7. Retry Policy

Server-side retries are intentionally minimal:

- Retry only `TransportError` and `ApiError` with status `500` or `503`.
- Backoff is exponential (`initial_backoff_ms * 2^n`).
- Retry count is configurable (`retry_max`), defaulting to `2`.

## 8. Configuration

Runtime config (CLI args and env vars):

- `--base-url` / `BOJ_BASE_URL` (default: `https://www.stat-search.boj.or.jp`)
- `--timeout-ms` / `BOJ_TIMEOUT_MS` (default: `10000`)
- `--retry-max` / `BOJ_RETRY_MAX` (default: `2`)
- `--retry-backoff-ms` / `BOJ_RETRY_BACKOFF_MS` (default: `200`)

## 9. Distribution

Distribution is split into two layers:

- Rust binary release assets (`boj-mcp-server-*`)
- npm launcher package (`@explorrrr/boj-mcp-server`) for `npx`

`mcp-release.yml` runs via `workflow_dispatch` and validates version consistency across:

- workflow input `version`
- `mcp-server/Cargo.toml`
- `npm/boj-mcp-server/package.json`

The npm publish job in `mcp-release.yml` uses GitHub Actions OIDC trusted publishing and does not use long-lived npm tokens. The publish job must grant `id-token: write`.

## 10. Compatibility

- Existing library public API remains unchanged.
- MCP tool names are stable once released.
- New MCP interfaces (`prompts`, `resources`, `completion`) are additive.
- Error semantics changed to `isError=true` tool results; this is a breaking contract change for clients that only handled JSON-RPC tool errors.
