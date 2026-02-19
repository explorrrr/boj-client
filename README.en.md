[日本語](./README.md) | English

# boj-client

[![CI](https://github.com/explorrrr/boj-client/actions/workflows/ci.yml/badge.svg)](https://github.com/explorrrr/boj-client/actions/workflows/ci.yml)
[![Nightly Contract Test](https://github.com/explorrrr/boj-client/actions/workflows/nightly-contract.yml/badge.svg)](https://github.com/explorrrr/boj-client/actions/workflows/nightly-contract.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Support

[![Buy Me a Coffee](https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png)](https://buymeacoffee.com/explorrrr)

If this OSS helps you, consider supporting ongoing development.

An unofficial Rust API client for the Bank of Japan Time-Series Statistics Data Search API functions (launched on February 18, 2026).

## What this crate is

`boj-client` is designed to make it easier to integrate the BOJ time-series statistics API into Rust applications. It provides a cleaner way to build requests and handle responses based on the official API specification.

## Features

- Simplifies request building for the BOJ API
- Organizes API response handling for Rust usage
- Includes companion documentation converted from the official API manual

## Installation

The crate is available on `crates.io`. Add this to your `Cargo.toml`:

```toml
[dependencies]
boj-client = "0.1.0"
```

## Official references

- BOJ announcement (2026-02-18): [Launch of API functions in the Time-Series Statistics Data Search Site](https://www.boj.or.jp/statistics/outline/notice_2026/not260218a.htm)
- API manual (PDF): [api_manual.pdf](https://www.stat-search.boj.or.jp/info/api_manual.pdf)
- API usage notice (PDF): [api_notice.pdf](https://www.stat-search.boj.or.jp/info/api_notice.pdf)

## Positioning (Unofficial client)

- `boj-client` is an unofficial (non-endorsed by BOJ) client for the BOJ Time-Series Statistics Data Search API functions.
- For usage conditions, limitations, and disclaimer/guarantee scope, prioritize the official BOJ documents above.

## Important Notice (Usage & Disclaimer)

- Usage, interpretation of specifications, and operational decisions for this software must follow BOJ official documents and BOJ's latest published decisions.
- If this README or implementation differs from BOJ publications/decisions, BOJ publications/decisions take precedence.
- If BOJ requests actions regarding this software, the author will comply without prior notice. This may include changes to specifications, provided contents, or publication status.
- To the fullest extent permitted by applicable law, the author of this software shall not be liable for any damages arising from the use or inability to use this software.

## Quick links

- [Contributing](./CONTRIBUTING.en.md)
- [Contributing (Japanese)](./CONTRIBUTING.md)
- [API manual](./docs/api-manual/README.md)
- [API client design](./docs/design/api-client-versioning.md)
- [MCP server design](./docs/design/mcp-server.md)
- [License](./LICENSE)
- [Buy Me a Coffee](https://buymeacoffee.com/explorrrr)

## Quick start

Response types are separated by API:

- `get_data_code` -> `CodeResponse`
- `get_data_layer` -> `LayerResponse`
- `get_metadata` -> `MetadataResponse`

```rust
use boj_client::client::BojClient;
use boj_client::query::{CodeQuery, Format, Language};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BojClient::new();
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
        .with_format(Format::Json)
        .with_lang(Language::En)
        .with_start_date("202401")?
        .with_end_date("202401")?;

    let _response = client.get_data_code(&query)?;
    Ok(())
}
```

## MCP Server (stdio / npx)

This repository includes both the Rust MCP server binary `boj-mcp-server` and the `npx` launcher package `@explorrrr/boj-mcp-server`.

### Available tools

- `boj_get_data_code`
- `boj_get_data_layer`
- `boj_get_metadata`

### Run with npx

```bash
npx -y @explorrrr/boj-mcp-server --help
```

### Run locally (Rust development)

```bash
cargo run -p boj-mcp-server
```

### MCP client config example (npx)

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

### Launcher environment variables

- `BOJ_MCP_SERVER_PATH`: use an existing local binary directly (skip download)
- `BOJ_MCP_CACHE_DIR`: cache directory for downloaded binaries
- `BOJ_MCP_RELEASE_BASE_URL`: release download base URL (default: `https://github.com/explorrrr/boj-client/releases/download`)

### Supported targets

- `linux-x64`
- `darwin-x64`
- `darwin-arm64`
- `win32-x64`

### Maintainer release flow

`mcp-release.yml` is executed via `workflow_dispatch`. The `version` input must match both `mcp-server/Cargo.toml` and `npm/boj-mcp-server/package.json`. npm publication runs only when `publish_npm=true`.

`raw` is returned only when `include_raw=true`.  
`get_data_code` and `get_data_layer` are single-page responses; use `next_position` to request the next page.

## Public API Notes (0.1.0)

- Client: `boj_client::client::BojClient`
- Queries and options: `boj_client::query::*`
- Response types: `boj_client::model::*`
- `decode` / `transport` are internal implementation modules and are not public

The project is still under active development. For behavior details, prioritize the API specification docs.

- API manual: [`docs/api-manual/README.md`](./docs/api-manual/README.md)
- API client design: [`docs/design/api-client-versioning.md`](./docs/design/api-client-versioning.md)

`docs/api-manual` contains BOJ official API specification references (source-aligned). `docs/design` contains this crate's client-side design policies (public interfaces, versioning, and compatibility operations). `docs/design/api-client-versioning.md` is a policy proposal for handling potential future BOJ API version changes and does not define BOJ's official versioning specification. If interpretation differs, prioritize `docs/api-manual` first and update `docs/design` as needed.

## Coverage and limitations

- Target API: Bank of Japan Time-Series Statistics Data Search API functions (launched on February 18, 2026; see BOJ announcement above)
- This crate is an unofficial client (not endorsed by BOJ)
- This repository prioritizes conformance to the official specification
- When the specification changes, updates follow the official manual

## Development status

- Status: In active development
- Compatibility and API design may still change before stable releases

### README translation policy (loose synchronization)

- Single-language updates are allowed
- If only one language is updated, add the following note to the updated README

> Note: Japanese version will be synchronized later.

- Remove the note once synchronization is complete
- It is acceptable for `Last updated` dates to differ between languages

## License

MIT License. See [`LICENSE`](./LICENSE) for details.

Last updated: 2026-02-19
