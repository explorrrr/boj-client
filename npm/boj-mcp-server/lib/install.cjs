"use strict";

const fs = require("node:fs/promises");
const { createWriteStream } = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { Readable } = require("node:stream");
const { pipeline } = require("node:stream/promises");

const tar = require("tar");

const { parseSha256Sums, verifyFileSha256 } = require("./checksum.cjs");
const { acquireLock } = require("./lock.cjs");
const { resolveTarget } = require("./platform.cjs");
const { buildReleaseUrls } = require("./release.cjs");

async function fileExists(filePath) {
  try {
    await fs.access(filePath);
    return true;
  } catch (error) {
    if (error && error.code === "ENOENT") {
      return false;
    }
    throw error;
  }
}

function defaultCacheDir() {
  if (process.platform === "win32") {
    if (process.env.LOCALAPPDATA) {
      return path.join(process.env.LOCALAPPDATA, "boj-mcp-server");
    }
    return path.join(os.tmpdir(), "boj-mcp-server");
  }

  if (process.env.XDG_CACHE_HOME) {
    return path.join(process.env.XDG_CACHE_HOME, "boj-mcp-server");
  }

  if (process.env.HOME) {
    return path.join(process.env.HOME, ".cache", "boj-mcp-server");
  }

  return path.join(os.tmpdir(), "boj-mcp-server");
}

async function fetchText(url) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`failed to download ${url}: HTTP ${response.status}`);
  }
  return response.text();
}

async function downloadFile(url, destinationPath) {
  const response = await fetch(url);
  if (!response.ok || !response.body) {
    throw new Error(`failed to download ${url}: HTTP ${response.status}`);
  }

  await pipeline(Readable.fromWeb(response.body), createWriteStream(destinationPath));
}

async function ensureBinary({ version, releaseBaseUrl, cacheDir, platform = process.platform, arch = process.arch }) {
  if (!version) {
    throw new Error("version is required for binary install");
  }

  const { targetTriple, binaryName } = resolveTarget(platform, arch);
  const release = buildReleaseUrls({ version, targetTriple, baseUrl: releaseBaseUrl });

  const rootCacheDir = cacheDir || defaultCacheDir();
  const targetDir = path.join(rootCacheDir, version, targetTriple);
  const binaryPath = path.join(targetDir, binaryName);

  if (await fileExists(binaryPath)) {
    return binaryPath;
  }

  await fs.mkdir(path.dirname(targetDir), { recursive: true });
  const lockPath = `${targetDir}.lock`;
  const releaseLock = await acquireLock(lockPath);

  let tempDir = null;
  try {
    if (await fileExists(binaryPath)) {
      return binaryPath;
    }

    tempDir = `${targetDir}.tmp-${process.pid}-${Date.now()}`;
    await fs.rm(tempDir, { recursive: true, force: true });
    await fs.mkdir(tempDir, { recursive: true });

    const checksumsText = await fetchText(release.checksumUrl);
    const checksums = parseSha256Sums(checksumsText);
    const expectedHash = checksums.get(release.assetName);
    if (!expectedHash) {
      throw new Error(`checksum entry not found for ${release.assetName} in ${release.checksumUrl}`);
    }

    const archivePath = path.join(tempDir, release.assetName);
    await downloadFile(release.assetUrl, archivePath);

    const checksumOk = await verifyFileSha256(archivePath, expectedHash);
    if (!checksumOk) {
      throw new Error(`checksum mismatch for ${release.assetName}`);
    }

    await tar.x({ file: archivePath, cwd: tempDir, strict: true });

    const extractedBinaryPath = path.join(tempDir, binaryName);
    if (!(await fileExists(extractedBinaryPath))) {
      throw new Error(`archive did not contain expected binary: ${binaryName}`);
    }

    if (platform !== "win32") {
      await fs.chmod(extractedBinaryPath, 0o755);
    }

    await fs.rm(archivePath, { force: true });
    await fs.rm(targetDir, { recursive: true, force: true });
    await fs.rename(tempDir, targetDir);

    return binaryPath;
  } finally {
    if (tempDir) {
      await fs.rm(tempDir, { recursive: true, force: true });
    }
    await releaseLock();
  }
}

module.exports = {
  defaultCacheDir,
  ensureBinary,
  fetchText,
  downloadFile,
};
