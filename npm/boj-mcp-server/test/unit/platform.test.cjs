"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");

const { assetNameForTarget, resolveTarget } = require("../../lib/platform.cjs");

test("resolveTarget maps all supported platforms", () => {
  assert.deepEqual(resolveTarget("linux", "x64"), {
    targetTriple: "x86_64-unknown-linux-gnu",
    binaryName: "boj-mcp-server",
  });

  assert.deepEqual(resolveTarget("darwin", "x64"), {
    targetTriple: "x86_64-apple-darwin",
    binaryName: "boj-mcp-server",
  });

  assert.deepEqual(resolveTarget("darwin", "arm64"), {
    targetTriple: "aarch64-apple-darwin",
    binaryName: "boj-mcp-server",
  });

  assert.deepEqual(resolveTarget("win32", "x64"), {
    targetTriple: "x86_64-pc-windows-msvc",
    binaryName: "boj-mcp-server.exe",
  });
});

test("resolveTarget throws for unsupported combinations", () => {
  assert.throws(() => resolveTarget("linux", "arm64"), /unsupported platform/);
});

test("assetNameForTarget builds release asset name", () => {
  assert.equal(assetNameForTarget("x86_64-unknown-linux-gnu"), "boj-mcp-server-x86_64-unknown-linux-gnu.tar.gz");
});
