import { exec, getVersion } from "./utils.js";

const version = getVersion();
exec(`git tag v${version}`);
exec("git push --tags");

console.log(`Tagged v${version} and pushed.`);
