"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");

const {
  calculateFileSha256,
  parseSha256Sums,
  verifyFileSha256,
} = require("../../lib/checksum.cjs");

test("parseSha256Sums parses hash lines", () => {
  const content = [
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa  boj-mcp-server-a.tar.gz",
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb *boj-mcp-server-b.tar.gz",
    "",
  ].join("\n");

  const parsed = parseSha256Sums(content);
  assert.equal(parsed.get("boj-mcp-server-a.tar.gz"), "a".repeat(64));
  assert.equal(parsed.get("boj-mcp-server-b.tar.gz"), "b".repeat(64));
});

test("calculateFileSha256 and verifyFileSha256 work together", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "boj-mcp-checksum-"));
  const filePath = path.join(tempDir, "sample.txt");

  try {
    await fs.writeFile(filePath, "boj-client", "utf8");
    const digest = await calculateFileSha256(filePath);
    assert.equal(digest.length, 64);
    assert.equal(await verifyFileSha256(filePath, digest), true);
    assert.equal(await verifyFileSha256(filePath, "0".repeat(64)), false);
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});
