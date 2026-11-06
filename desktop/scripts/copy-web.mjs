// Copies the web app (repo root) into desktop/dist for the Tauri build.
import { cpSync, mkdirSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, "..", "..");
const dist = join(here, "..", "dist");
rmSync(dist, { recursive: true, force: true });
mkdirSync(dist, { recursive: true });
for (const f of ["index.html", "manifest.webmanifest"]) cpSync(join(root, f), join(dist, f));
cpSync(join(root, "icons"), join(dist, "icons"), { recursive: true });
console.log("Copied web app into", dist);
