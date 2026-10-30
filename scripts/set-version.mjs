// Setel versi app dari tag git (mis. v1.2.3) sebelum build rilis.
// Dipakai workflow release.yml: node scripts/set-version.mjs "$GITHUB_REF_NAME"
import { readFileSync, writeFileSync } from "node:fs";

const tag = process.argv[2] ?? "";
const version = tag.replace(/^v/, "");
// Installer MSI Windows hanya menerima versi angka x.y.z (tanpa -beta dsb.)
if (!/^\d+\.\d+\.\d+$/.test(version)) {
  console.error(`Tag "${tag}" harus berbentuk vX.Y.Z, mis. v1.0.0`);
  process.exit(1);
}

const updateJson = (path, update) => {
  const data = JSON.parse(readFileSync(path, "utf8"));
  update(data);
  writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`);
};
updateJson("src-tauri/tauri.conf.json", (d) => (d.version = version));
updateJson("package.json", (d) => (d.version = version));
// Ditampilkan di sidebar app
writeFileSync("VERSION", `v${version}`);
console.log(`Versi diset ke ${version}`);
