"use strict";

const { assetNameForTarget } = require("./platform.cjs");

const DEFAULT_RELEASE_BASE_URL = "https://github.com/explorrrr/boj-client/releases/download";

function normalizeBaseUrl(baseUrl) {
  return String(baseUrl || DEFAULT_RELEASE_BASE_URL).replace(/\/+$/, "");
}

function releaseTag(version) {
  return `mcp-server-v${version}`;
}

function buildReleaseUrls({ version, targetTriple, baseUrl }) {
  const normalizedBaseUrl = normalizeBaseUrl(baseUrl);
  const tag = releaseTag(version);
  const assetName = assetNameForTarget(targetTriple);
  const releasePrefix = `${normalizedBaseUrl}/${tag}`;

  return {
    tag,
    assetName,
    assetUrl: `${releasePrefix}/${assetName}`,
    checksumUrl: `${releasePrefix}/SHA256SUMS`,
  };
}

module.exports = {
  DEFAULT_RELEASE_BASE_URL,
  normalizeBaseUrl,
  releaseTag,
  buildReleaseUrls,
};
