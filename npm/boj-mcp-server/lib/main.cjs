"use strict";

const { spawn } = require("node:child_process");
const path = require("node:path");

const packageJson = require("../package.json");
const { ensureBinary } = require("./install.cjs");

function resolveExplicitBinaryPath(binaryPath) {
  if (!binaryPath) {
    return null;
  }
  if (path.isAbsolute(binaryPath)) {
    return binaryPath;
  }
  return path.resolve(process.cwd(), binaryPath);
}

function runBinary(binaryPath, args, { env = process.env, spawnImpl = spawn } = {}) {
  return new Promise((resolve, reject) => {
    const child = spawnImpl(binaryPath, args, {
      stdio: "inherit",
      env,
    });

    child.once("error", reject);
    child.once("exit", (code, signal) => {
      if (signal) {
        reject(new Error(`boj-mcp-server terminated by signal: ${signal}`));
        return;
      }
      if (typeof code === "number") {
        resolve(code);
        return;
      }
      resolve(1);
    });
  });
}

async function main(args, options = {}) {
  const env = options.env || process.env;
  const platform = options.platform || process.platform;
  const arch = options.arch || process.arch;
  const version = options.version || packageJson.version;

  const explicitPath = resolveExplicitBinaryPath(env.BOJ_MCP_SERVER_PATH);
  const binaryPath =
    explicitPath ||
    (await ensureBinary({
      version,
      releaseBaseUrl: env.BOJ_MCP_RELEASE_BASE_URL,
      cacheDir: env.BOJ_MCP_CACHE_DIR,
      platform,
      arch,
    }));

  return runBinary(binaryPath, args, {
    env,
    spawnImpl: options.spawnImpl,
  });
}

module.exports = {
  main,
  resolveExplicitBinaryPath,
  runBinary,
};
