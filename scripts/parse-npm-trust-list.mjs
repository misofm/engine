#!/usr/bin/env node

import fs from "node:fs";

const source = fs.readFileSync(0, "utf8");

if (source.length === 0) {
  process.stdout.write("absent\n");
  process.exit(0);
}

class InvalidTrustList extends Error {}

function reject(message) {
  throw new InvalidTrustList(`npm trust list is not one exact trusted-publisher object: ${message}`);
}

// JSON.parse intentionally does not report duplicate object keys. Scan the complete
// document first so a malicious duplicate cannot be hidden by JSON's last-key-wins
// behavior. Decoding property names makes escaped spellings collide too.
function rejectDuplicateKeys(text) {
  let index = 0;
  const whitespace = () => {
    while (/[\u0020\u000a\u000d\u0009]/.test(text[index] ?? "")) index += 1;
  };
  const string = () => {
    const start = index;
    if (text[index] !== '"') reject("invalid JSON string");
    index += 1;
    while (index < text.length) {
      if (text[index] === "\\") {
        index += 2;
      } else if (text[index] === '"') {
        index += 1;
        try {
          return JSON.parse(text.slice(start, index));
        } catch {
          reject("invalid JSON string");
        }
      } else {
        index += 1;
      }
    }
    reject("unterminated JSON string");
  };
  const value = () => {
    whitespace();
    if (text[index] === "{") {
      index += 1;
      whitespace();
      const keys = new Set();
      if (text[index] === "}") { index += 1; return; }
      while (true) {
        whitespace();
        const key = string();
        if (keys.has(key)) reject(`duplicate key ${JSON.stringify(key)}`);
        keys.add(key);
        whitespace();
        if (text[index] !== ":") reject("object key lacks a colon");
        index += 1;
        value();
        whitespace();
        if (text[index] === "}") { index += 1; return; }
        if (text[index] !== ",") reject("object properties lack a comma");
        index += 1;
      }
    }
    if (text[index] === "[") {
      index += 1;
      whitespace();
      if (text[index] === "]") { index += 1; return; }
      while (true) {
        value();
        whitespace();
        if (text[index] === "]") { index += 1; return; }
        if (text[index] !== ",") reject("array values lack a comma");
        index += 1;
      }
    }
    if (text[index] === '"') { string(); return; }
    for (const literal of ["true", "false", "null"]) {
      if (text.startsWith(literal, index)) { index += literal.length; return; }
    }
    const number = text.slice(index).match(/^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/);
    if (number) { index += number[0].length; return; }
    reject("invalid JSON value");
  };

  value();
  whitespace();
  if (index !== text.length) reject("multiple JSON documents or trailing data");
}

try {
  rejectDuplicateKeys(source);
  const entry = JSON.parse(source);
  if (entry === null || Array.isArray(entry) || typeof entry !== "object") reject("document is not an object");
  const expectedKeys = ["file", "id", "permissions", "repository", "type"];
  const actualKeys = Object.keys(entry).sort();
  if (actualKeys.length !== expectedKeys.length || actualKeys.some((key, index) => key !== expectedKeys[index])) reject("object keys differ");
  if (typeof entry.id !== "string" || entry.id.length === 0) reject("id is not a nonempty string");
  if (entry.type !== "github" || entry.file !== "npm-publish.yml" || entry.repository !== "misofm/engine") reject("publisher identity differs");
  if (!Array.isArray(entry.permissions) || entry.permissions.length !== 1 || entry.permissions[0] !== "createPackage") reject("permissions differ");
  process.stdout.write("present\n");
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exit(1);
}
