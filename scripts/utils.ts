import { execSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = resolve(fileURLToPath(import.meta.url), "../..");

export function exec(cmd: string): string {
  return execSync(cmd, { cwd: ROOT, encoding: "utf-8" }).trim();
}

export function getVersion(): string {
  const cargo = readFileSync(resolve(ROOT, "Cargo.toml"), "utf-8");
  const m = cargo.match(/^version\s*=\s*"([^"]+)"/m);
  if (!m) throw new Error("version not found in Cargo.toml");
  console.log(`Version recieved: ${m[1]}`);
  return m[1];
}


