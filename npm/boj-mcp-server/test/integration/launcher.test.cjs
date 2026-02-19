"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");
const http = require("node:http");

const tar = require("tar");

const { calculateFileSha256 } = require("../../lib/checksum.cjs");
const { main } = require("../../lib/main.cjs");
const { resolveTarget, assetNameForTarget } = require("../../lib/platform.cjs");
const { releaseTag } = require("../../lib/release.cjs");

async function startStaticServer(rootDir) {
  const requestCounts = new Map();

  const server = http.createServer(async (req, res) => {
    const requestPath = req.url || "/";
    requestCounts.set(requestPath, (requestCounts.get(requestPath) || 0) + 1);

    const decoded = decodeURIComponent(requestPath.split("?")[0]);
    const sanitized = decoded.replace(/^\/+/, "");
    const filePath = path.join(rootDir, sanitized);

    if (!filePath.startsWith(rootDir)) {
      res.statusCode = 400;
      res.end("bad path");
      return;
    }

    try {
      const stat = await fs.stat(filePath);
      if (!stat.isFile()) {
        res.statusCode = 404;
        res.end("not found");
        return;
      }
      const content = await fs.readFile(filePath);
      res.statusCode = 200;
      res.end(content);
    } catch {
      res.statusCode = 404;
      res.end("not found");
    }
  });

  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const address = server.address();
  if (!address || typeof address === "string") {
    throw new Error("failed to start static server");
  }

  return {
    baseUrl: `http://127.0.0.1:${address.port}`,
    getCount(requestPath) {
      return requestCounts.get(requestPath) || 0;
    },
    close() {
      return new Promise((resolve, reject) => {
        server.close((error) => {
          if (error) {
            reject(error);
            return;
          }
          resolve();
        });
      });
    },
  };
}

async function createReleaseFixture({
  rootDir,
  version,
  targetTriple,
  binaryName,
  scriptBody,
  checksumOverride,
}) {
  const tag = releaseTag(version);
  const releaseDir = path.join(rootDir, tag);
  await fs.mkdir(releaseDir, { recursive: true });

  const stagingDir = path.join(rootDir, `staging-${Date.now()}-${Math.random().toString(16).slice(2)}`);
  await fs.mkdir(stagingDir, { recursive: true });

  const stagedBinaryPath = path.join(stagingDir, binaryName);
  await fs.writeFile(stagedBinaryPath, scriptBody, "utf8");
  await fs.chmod(stagedBinaryPath, 0o755);

  const assetName = assetNameForTarget(targetTriple);
  const assetPath = path.join(releaseDir, assetName);

  await tar.c(
    {
      gzip: true,
      file: assetPath,
      cwd: stagingDir,
    },
    [binaryName],
  );

  const digest = await calculateFileSha256(assetPath);
  const checksum = checksumOverride || digest;
  await fs.writeFile(
    path.join(releaseDir, "SHA256SUMS"),
    `${checksum}  ${assetName}\n`,
    "utf8",
  );

  await fs.rm(stagingDir, { recursive: true, force: true });

  return {
    tag,
    assetName,
  };
}

test("main downloads binary, verifies checksum, caches it, and forwards args", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "boj-mcp-launcher-int-"));
  const cacheDir = path.join(tempDir, "cache");
  const releaseRoot = path.join(tempDir, "release");
  const argsOut = path.join(tempDir, "args.txt");
  const version = "9.9.9";

  const { targetTriple, binaryName } = resolveTarget("linux", "x64");

  const scriptBody = [
    "#!/bin/sh",
    "printf '%s\\n' \"$@\" > \"${LAUNCHER_TEST_ARGS_FILE}\"",
    "exit 0",
    "",
  ].join("\n");

  const { assetName, tag } = await createReleaseFixture({
    rootDir: releaseRoot,
    version,
    targetTriple,
    binaryName,
    scriptBody,
  });

  const server = await startStaticServer(releaseRoot);

  try {
    const env = {
      ...process.env,
      BOJ_MCP_CACHE_DIR: cacheDir,
      BOJ_MCP_RELEASE_BASE_URL: server.baseUrl,
      LAUNCHER_TEST_ARGS_FILE: argsOut,
    };

    const args = ["--timeout-ms", "123", "--retry-max", "7"];
    await main(args, {
      env,
      version,
      platform: "linux",
      arch: "x64",
    });

    const firstRunArgs = await fs.readFile(argsOut, "utf8");
    assert.equal(firstRunArgs, "--timeout-ms\n123\n--retry-max\n7\n");

    const checksumPath = `/${tag}/SHA256SUMS`;
    const assetPath = `/${tag}/${assetName}`;
    assert.equal(server.getCount(checksumPath), 1);
    assert.equal(server.getCount(assetPath), 1);

    await main(["--retry-max", "2"], {
      env,
      version,
      platform: "linux",
      arch: "x64",
    });

    const secondRunArgs = await fs.readFile(argsOut, "utf8");
    assert.equal(secondRunArgs, "--retry-max\n2\n");

    assert.equal(server.getCount(checksumPath), 1);
    assert.equal(server.getCount(assetPath), 1);
  } finally {
    await server.close();
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});

test("main fails fast on checksum mismatch", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "boj-mcp-launcher-int-"));
  const cacheDir = path.join(tempDir, "cache");
  const releaseRoot = path.join(tempDir, "release");
  const version = "9.9.8";

  const { targetTriple, binaryName } = resolveTarget("linux", "x64");

  await createReleaseFixture({
    rootDir: releaseRoot,
    version,
    targetTriple,
    binaryName,
    scriptBody: "#!/bin/sh\nexit 0\n",
    checksumOverride: "0".repeat(64),
  });

  const server = await startStaticServer(releaseRoot);

  try {
    const env = {
      ...process.env,
      BOJ_MCP_CACHE_DIR: cacheDir,
      BOJ_MCP_RELEASE_BASE_URL: server.baseUrl,
    };

    await assert.rejects(
      () =>
        main([], {
          env,
          version,
          platform: "linux",
          arch: "x64",
        }),
      /checksum mismatch/,
    );
  } finally {
    await server.close();
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});

test("BOJ_MCP_SERVER_PATH bypasses download", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "boj-mcp-launcher-int-"));
  const binaryPath = path.join(tempDir, "boj-mcp-server");
  const argsOut = path.join(tempDir, "args.txt");

  const scriptBody = [
    "#!/bin/sh",
    "printf '%s\\n' \"$@\" > \"${LAUNCHER_TEST_ARGS_FILE}\"",
    "exit 0",
    "",
  ].join("\n");

  try {
    await fs.writeFile(binaryPath, scriptBody, "utf8");
    await fs.chmod(binaryPath, 0o755);

    const env = {
      ...process.env,
      BOJ_MCP_SERVER_PATH: binaryPath,
      LAUNCHER_TEST_ARGS_FILE: argsOut,
    };

    await main(["--base-url", "https://example.invalid"], {
      env,
      version: "0.0.0",
      platform: "linux",
      arch: "x64",
    });

    const args = await fs.readFile(argsOut, "utf8");
    assert.equal(args, "--base-url\nhttps://example.invalid\n");
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});
