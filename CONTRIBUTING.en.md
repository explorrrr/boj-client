# Contributing to boj-client

This document defines the contribution workflow for `boj-client`.

## 1. Purpose and Scope

- In scope: Feature additions, bug fixes, test additions, and documentation updates by external contributors
- Out of scope: Maintainer-authorized release execution itself (workflow_dispatch permissions are required)

## 2. Setup

- Use Rust stable
- After fetching dependencies, ensure the following commands run successfully

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps
cargo test --doc --workspace
```

## 3. Repository Layout

- `src/`: Library implementation
- `tests/`: Integration tests
- `tests/fixtures/`: Fixtures for decoder and behavior verification
- `docs/api-manual/`: Primary reference for official specifications (source-aligned Markdown)
- `docs/design/`: Client design policies

As a library crate, `Cargo.lock` is not tracked. Files included in crates.io packages must be managed with `[package].include` in `Cargo.toml`, not `.gitignore`. Keep published artifacts minimal: `src/` and metadata files (`Cargo.toml`, `README*`, `LICENSE`).

## 4. Branch Strategy

Direct commits and direct pushes to `master` are prohibited. All changes must be merged via pull requests.

- Feature additions / spec extensions: `feature/<topic>`
- Bug fixes: `hotfix/<topic>`
- Operational improvements, development workflow improvements, docs-only changes, CI/tooling maintenance, and dependency updates (without external behavior changes): `chore/<topic>`

Choose the branch type by the primary objective of the change.

- For mixed changes, decide by the primary objective
- If bug fixes are included, prioritize `hotfix/*`
- If user-facing feature additions or spec extensions are primary, use `feature/*`
- Otherwise (operations, developer experience, docs), use `chore/*`
- Docs-only changes are always `chore/*`

Branch name examples:

- `feature/add-layer-query-validation`
- `hotfix/fix-csv-json-error-fallback`
- `chore/improve-contributing-workflow`

## 5. Change Flow

1. Fetch the latest `master`
2. Create a branch based on the objective (`feature/*` / `hotfix/*` / `chore/*`)
3. Update implementation, tests, and documentation
4. Pass required checks
5. Open a PR and receive review

## 6. Specification and Implementation Rules

- Prioritize `docs/api-manual/` for API specification interpretation
- Keep client design decisions consistent with `docs/design/`
- When incorporating specification changes, update implementation, tests, and documentation together as needed

### 6.1 rustdoc Writing Policy (Public API)

- Use English as the default language for public API `rustdoc`
- Keep documentation concise and user-oriented so usage and behavior are easy to understand
- For proper nouns and official API terms, Japanese/original-language notation may be added when needed
- Missing documentation on public APIs is treated as a build error (`#![deny(missing_docs)]`)
- Broken links and bare URLs are treated as build errors (`rustdoc::broken_intra_doc_links`, `rustdoc::bare_urls`)

Recommended template (public types and public functions):

- One-line summary (what the type/function provides)
- Add constraints as needed (for example, date format or value range)
- For public functions returning `Result`, `# Errors` is required
- Add `# Examples` when usage examples are meaningful
- For examples involving network I/O, use `no_run` so doctest performs compile-only validation

## 7. Test Addition Rules

- Query-building or URL changes: update `tests/url_building.rs`
- Decoder changes: update `tests/json_decoder.rs` or `tests/csv_decoder.rs`
- Client behavior changes: update `tests/client_behavior.rs`
- Snapshot-target changes: update `tests/snapshot_regression.rs`
- If fixtures are added or updated: update the reference section in `tests/fixtures/README.md`

## 8. Required Checks Before Submission

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps
cargo test --doc --workspace
```

## 9. Optional Checks

- Nightly contract test (when you want to verify connectivity with the live API)

```bash
BOJ_CONTRACT_TEST=1 cargo test --test contract_nightly -- --ignored
```

- When snapshot output formatting updates are required

```bash
cargo insta test --force-update-snapshots
```

## 10. Documentation Synchronization Rules

- If updating only one of `README.md` or `README.en.md`, explicitly state the deferral reason in the PR
- README translations follow a loose synchronization policy
- Resolve deferred status after synchronization is completed

## 11. PR Rules

- Fill in `.github/pull_request_template.md`
- In `Summary / 概要` and `What Changed / 変更内容`, describe the objective and the actual changes
- In `Compatibility & Release Impact / 互換性とリリース影響`, specify API impact and expected SemVer level; if the change is breaking, include migration notes
- In `Test Evidence / テスト実施内容`, list executed checks and added/updated tests
- In `Spec & Docs Sync / 仕様・ドキュメント同期` and `README Synchronization / README同期`, clearly state whether documentation updates were made
- In `Reviewer Focus / レビュアー注目点` and `Risks & Rollback / リスクとロールバック`, describe focus areas and risk mitigation/rollback strategy
- `Related issue` is optional (it can be left blank if no issue exists)
- Opening an issue for large changes is optional, but recommended for context sharing

## 12. Definition of Done

The work is considered complete when all of the following are satisfied:

- Work is done on the appropriate branch type (`feature/*` / `hotfix/*` / `chore/*`)
- Required checks pass
- Necessary tests are added or updated
- Specification references and implementation are consistent
- PR template is filled out

## 13. Maintainer Release Flow (Reference)

This is the publication flow under protected-branch operation (PR required). The old path where a workflow directly pushes release changes to `master` is not used.

1. Bump versions in a PR and merge into `master`
2. Run `release-publish.yml` via `workflow_dispatch`
   - Input `version`: must match the root `Cargo.toml` version
   - Input `dry_run=true`: run release gate only
   - Input `dry_run=false`: publish to crates.io after the gate passes
3. If MCP publication is required, run `mcp-release.yml`
   - Input `version`: must match both `mcp-server/Cargo.toml` and `npm/boj-mcp-server/package.json`
   - `publish_npm=false`: publish GitHub Release assets only
   - `publish_npm=true`: publish npm package after assets upload (summary still shows assets status if npm publish fails)
