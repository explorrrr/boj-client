"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const { EventEmitter } = require("node:events");

const { resolveExplicitBinaryPath, runBinary } = require("../../lib/main.cjs");

test("resolveExplicitBinaryPath keeps absolute path", () => {
  const absolutePath = "/tmp/boj-mcp-server";
  assert.equal(resolveExplicitBinaryPath(absolutePath), absolutePath);
});

test("resolveExplicitBinaryPath resolves relative path", () => {
  const resolved = resolveExplicitBinaryPath("./bin/boj-mcp-server");
  assert.match(resolved, /bin\/boj-mcp-server$/);
});

test("runBinary returns child exit code", async () => {
  const spawnImpl = () => {
    const child = new EventEmitter();
    process.nextTick(() => child.emit("exit", 42, null));
    return child;
  };

  const exitCode = await runBinary("dummy", [], { spawnImpl });
  assert.equal(exitCode, 42);
});

test("runBinary rejects when child emits error", async () => {
  const spawnImpl = () => {
    const child = new EventEmitter();
    process.nextTick(() => child.emit("error", new Error("boom")));
    return child;
  };

  await assert.rejects(() => runBinary("dummy", [], { spawnImpl }), /boom/);
});
