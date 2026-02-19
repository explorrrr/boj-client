#!/usr/bin/env node

const { main } = require("../lib/main.cjs");

main(process.argv.slice(2))
  .then((exitCode) => {
    process.exit(typeof exitCode === "number" ? exitCode : 0);
  })
  .catch((error) => {
    const message = error && error.message ? error.message : String(error);
    console.error(`[boj-mcp-server] ${message}`);
    process.exit(1);
  });
