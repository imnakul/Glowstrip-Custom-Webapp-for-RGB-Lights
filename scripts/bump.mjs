// Sets one version everywhere: web app, service worker cache, desktop app.
// Usage: node scripts/bump.mjs 1.10.0
import { readFileSync, writeFileSync } from "node:fs";
const v = process.argv[2];
if (!/^\d+\.\d+\.\d+$/.test(v || "")) { console.error("Usage: node scripts/bump.mjs X.Y.Z"); process.exit(1); }
const edit = (f, re, rep) => { const s = readFileSync(f, "utf8"); if (!re.test(s)) throw new Error("pattern not found in " + f); writeFileSync(f, s.replace(re, rep)); };
edit("index.html", /const VERSION = "[^"]+";/, `const VERSION = "${v}";`);
edit("sw.js", /const CACHE = "glowstrip-v[^"]+";/, `const CACHE = "glowstrip-v${v}";`);
edit("desktop/src-tauri/tauri.conf.json", /"version": "[^"]+"/, `"version": "${v}"`);
edit("desktop/src-tauri/Cargo.toml", /^version = "[^"]+"/m, `version = "${v}"`);
edit("desktop/package.json", /"version": "[^"]+"/, `"version": "${v}"`);
console.log("Version set to", v, "(remember RELEASED and NOTES in index.html)");
