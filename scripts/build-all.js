const { execSync } = require('child_process');
const { rmSync, existsSync, readdirSync, mkdirSync, copyFileSync } = require('fs');
const path = require('path');

const ROOT = path.resolve(__dirname, '..');
const SRC_TAURI = path.join(ROOT, 'src-tauri');

function run(cmd) {
  console.log(`\n> ${cmd}`);
  execSync(cmd, { stdio: 'inherit', cwd: ROOT });
}

console.log('[1/4] Building x64...');
run('npm run build');

console.log('\n[2/4] Building x86...');
run('npm run build:x86');

const srcDir = path.join(SRC_TAURI, 'target', 'i686-pc-windows-msvc', 'release', 'bundle', 'nsis');
const dstDir = path.join(SRC_TAURI, 'target', 'release', 'bundle', 'nsis');

if (!existsSync(srcDir)) {
  console.error(`Error: x86 bundle directory not found at ${srcDir}`);
  process.exit(1);
}

console.log('\n[3/4] Copying x86 installer to bundle directory...');
if (!existsSync(dstDir)) mkdirSync(dstDir, { recursive: true });
const files = readdirSync(srcDir).filter(f => f.endsWith('.exe'));
for (const f of files) {
  copyFileSync(path.join(srcDir, f), path.join(dstDir, f));
  console.log(`  copied ${f}`);
}

console.log('\n[4/4] Cleaning up x86 build artifacts...');
if (existsSync(srcDir)) {
  const x86Target = path.join(SRC_TAURI, 'target', 'i686-pc-windows-msvc');
  rmSync(x86Target, { recursive: true, force: true });
  console.log('  removed src-tauri/target/i686-pc-windows-msvc');
}

console.log('\nDone! Bundles at: src-tauri/target/release/bundle/nsis/');
readdirSync(dstDir).filter(f => f.endsWith('.exe')).forEach(f => console.log(`  - ${f}`));
