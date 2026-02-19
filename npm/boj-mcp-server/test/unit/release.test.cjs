"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");

const {
  DEFAULT_RELEASE_BASE_URL,
  buildReleaseUrls,
  normalizeBaseUrl,
  releaseTag,
} = require("../../lib/release.cjs");

test("releaseTag prefixes semantic version", () => {
  assert.equal(releaseTag("0.1.0"), "mcp-server-v0.1.0");
});

test("normalizeBaseUrl trims trailing slash", () => {
  assert.equal(normalizeBaseUrl("https://example.com/path/"), "https://example.com/path");
  assert.equal(normalizeBaseUrl(undefined), DEFAULT_RELEASE_BASE_URL);
});

test("buildReleaseUrls creates deterministic URLs", () => {
  const urls = buildReleaseUrls({
    version: "0.1.0",
    targetTriple: "x86_64-unknown-linux-gnu",
    baseUrl: "https://releases.example.com/download/",
  });

  assert.deepEqual(urls, {
    tag: "mcp-server-v0.1.0",
    assetName: "boj-mcp-server-x86_64-unknown-linux-gnu.tar.gz",
    assetUrl:
      "https://releases.example.com/download/mcp-server-v0.1.0/boj-mcp-server-x86_64-unknown-linux-gnu.tar.gz",
    checksumUrl: "https://releases.example.com/download/mcp-server-v0.1.0/SHA256SUMS",
  });
});
