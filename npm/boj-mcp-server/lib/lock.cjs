"use strict";

const fs = require("node:fs/promises");

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function isStaleLock(lockPath, staleMs) {
  try {
    const stat = await fs.stat(lockPath);
    return Date.now() - stat.mtimeMs > staleMs;
  } catch (error) {
    if (error && error.code === "ENOENT") {
      return false;
    }
    throw error;
  }
}

async function acquireLock(
  lockPath,
  { staleMs = 5 * 60 * 1000, retryMs = 200, maxWaitMs = 60 * 1000 } = {},
) {
  const startedAt = Date.now();

  while (true) {
    try {
      const handle = await fs.open(lockPath, "wx");
      await handle.writeFile(`${process.pid} ${new Date().toISOString()}\n`, "utf8");

      return async function release() {
        await handle.close();
        await fs.rm(lockPath, { force: true });
      };
    } catch (error) {
      if (!error || error.code !== "EEXIST") {
        throw error;
      }

      if (await isStaleLock(lockPath, staleMs)) {
        await fs.rm(lockPath, { force: true });
        continue;
      }

      if (Date.now() - startedAt > maxWaitMs) {
        throw new Error(`timed out waiting for lock: ${lockPath}`);
      }

      await sleep(retryMs);
    }
  }
}

module.exports = {
  acquireLock,
};
