#!/usr/bin/env node

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const parser = fileURLToPath(new URL("./parse-npm-trust-list.mjs", import.meta.url));
const exact = JSON.stringify({ id: "publisher-123", type: "github", file: "npm-publish.yml", repository: "misofm/engine", permissions: ["createPackage"] });

function run(input) {
  return spawnSync(process.execPath, [parser], { input, encoding: "utf8" });
}
function accepts(name, input, output) {
  const result = run(input);
  assert.equal(result.status, 0, `${name}: ${result.stderr}`);
  assert.equal(result.stdout, `${output}\n`, name);
}
function rejects(name, input) {
  const result = run(input);
  assert.notEqual(result.status, 0, `${name}: accepted unexpectedly`);
}

accepts("zero-byte absence", "", "absent");
accepts("exact object", exact, "present");
accepts("surrounding JSON whitespace", ` \n${exact}\t`, "present");
rejects("whitespace-only absence", " \n\t");
rejects("array", `[${exact}]`);
rejects("scalar", '"publisher-123"');
rejects("malformed JSON", "{");
rejects("multiple documents", `${exact}\n${exact}`);
rejects("duplicate key", '{"id":"first","id":"second","type":"github","file":"npm-publish.yml","repository":"misofm/engine","permissions":["createPackage"]}');
rejects("escaped duplicate key", '{"id":"first","\\u0069d":"second","type":"github","file":"npm-publish.yml","repository":"misofm/engine","permissions":["createPackage"]}');
rejects("missing field", JSON.stringify({ id: "publisher-123", type: "github", file: "npm-publish.yml", repository: "misofm/engine" }));
rejects("extra field", JSON.stringify({ id: "publisher-123", type: "github", file: "npm-publish.yml", repository: "misofm/engine", permissions: ["createPackage"], extra: true }));
rejects("empty id", JSON.stringify({ id: "", type: "github", file: "npm-publish.yml", repository: "misofm/engine", permissions: ["createPackage"] }));
rejects("wrong provider", exact.replace('"github"', '"gitlab"'));
rejects("wrong file", exact.replace('"npm-publish.yml"', '"release.yml"'));
rejects("wrong repository", exact.replace('"misofm/engine"', '"misofm/other"'));
rejects("wrong permission", exact.replace('"createPackage"', '"publish"'));
rejects("extra permission", exact.replace('["createPackage"]', '["createPackage","other"]'));

process.stdout.write("npm trust list parser fixtures and mutations: ok\n");
