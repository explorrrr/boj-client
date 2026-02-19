"use strict";

const SUPPORTED_TARGETS = Object.freeze({
  "linux:x64": {
    targetTriple: "x86_64-unknown-linux-gnu",
    binaryName: "boj-mcp-server",
  },
  "darwin:x64": {
    targetTriple: "x86_64-apple-darwin",
    binaryName: "boj-mcp-server",
  },
  "darwin:arm64": {
    targetTriple: "aarch64-apple-darwin",
    binaryName: "boj-mcp-server",
  },
  "win32:x64": {
    targetTriple: "x86_64-pc-windows-msvc",
    binaryName: "boj-mcp-server.exe",
  },
});

function resolveTarget(platform, arch) {
  const key = `${platform}:${arch}`;
  const resolved = SUPPORTED_TARGETS[key];
  if (!resolved) {
    const supported = Object.keys(SUPPORTED_TARGETS).join(", ");
    throw new Error(`unsupported platform/arch combination: ${key}. supported combinations: ${supported}`);
  }
  return resolved;
}

function assetNameForTarget(targetTriple) {
  return `boj-mcp-server-${targetTriple}.tar.gz`;
}

module.exports = {
  SUPPORTED_TARGETS,
  resolveTarget,
  assetNameForTarget,
};
