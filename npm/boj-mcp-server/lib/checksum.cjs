"use strict";

const { createHash } = require("node:crypto");
const { createReadStream } = require("node:fs");

function parseSha256Sums(content) {
  const checksums = new Map();

  for (const rawLine of String(content).split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line) {
      continue;
    }

    const match = /^([a-fA-F0-9]{64})\s+\*?(.+)$/.exec(line);
    if (!match) {
      continue;
    }

    checksums.set(match[2].trim(), match[1].toLowerCase());
  }

  return checksums;
}

function calculateFileSha256(filePath) {
  return new Promise((resolve, reject) => {
    const hash = createHash("sha256");
    const stream = createReadStream(filePath);

    stream.on("error", reject);
    stream.on("data", (chunk) => hash.update(chunk));
    stream.on("end", () => resolve(hash.digest("hex")));
  });
}

async function verifyFileSha256(filePath, expectedHash) {
  const actual = await calculateFileSha256(filePath);
  return actual === String(expectedHash).toLowerCase();
}

module.exports = {
  parseSha256Sums,
  calculateFileSha256,
  verifyFileSha256,
};
