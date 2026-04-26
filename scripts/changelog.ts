import { exec } from "./utils.js";

exec("git cliff --unreleased --prepend CHANGELOG.md");
exec("git add CHANGELOG.md");
exec('git commit -m "chore: update changelog"');
exec("git push");

console.log("Changelog committed and pushed. Bump versions in Cargo.toml, then run pnpm tag.");
