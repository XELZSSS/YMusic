const { execSync } = require('child_process');
const { rmSync, existsSync, readdirSync, mkdirSync, copyFileSync, writeFileSync, readFileSync } = require('fs');
const path = require('path');
const os = require('os');

const ROOT = path.resolve(__dirname, '..');
const SRC_TAURI = path.join(ROOT, 'src-tauri');
const KEY_PATH = process.env.TAURI_SIGNING_PRIVATE_KEY_PATH || path.join(os.homedir(), '.tauri', 'ymusic.key');
const KEY_PASSWORD = process.env.TAURI_SIGNING_PRIVATE_KEY_PASSWORD || '';

const X64_DIR = path.join(SRC_TAURI, 'target', 'release', 'bundle', 'nsis');
const X86_DIR = path.join(SRC_TAURI, 'target', 'i686-pc-windows-msvc', 'release', 'bundle', 'nsis');

function runWithKey(cmd) {
  console.log(`\n> ${cmd}`);
  execSync(cmd, {
    stdio: 'inherit',
    cwd: ROOT,
    env: {
      ...process.env,
      TAURI_SIGNING_PRIVATE_KEY: readFileSync(KEY_PATH, 'utf-8').replace(/\r?\n$/, ''),
      TAURI_SIGNING_PRIVATE_KEY_PASSWORD: KEY_PASSWORD,
    },
  });
}

function cleanDir(dir) {
  if (!existsSync(dir)) return;
  for (const f of readdirSync(dir)) {
    rmSync(path.join(dir, f), { force: true });
    console.log(`  cleaned ${f}`);
  }
}

console.log('[0/5] Cleaning old bundles...');
cleanDir(X64_DIR);
cleanDir(X86_DIR);

console.log('\n[1/5] Building x64...');
try {
  runWithKey('npm run build');
} catch (e) {
  console.error('x64 build failed:', e.message);
  process.exit(1);
}

console.log('\n[2/5] Building x86...');
try {
  runWithKey('npm run build:x86');
} catch (e) {
  console.error('x86 build failed:', e.message);
  process.exit(1);
}

if (!existsSync(X86_DIR)) {
  console.error(`Error: x86 bundle directory not found at ${X86_DIR}`);
  process.exit(1);
}

console.log('\n[3/5] Copying x86 artifacts...');
if (!existsSync(X64_DIR)) mkdirSync(X64_DIR, { recursive: true });
for (const f of readdirSync(X86_DIR)) {
  copyFileSync(path.join(X86_DIR, f), path.join(X64_DIR, f));
  console.log(`  copied ${f}`);
}

console.log('\n[4/5] Generating updater.json...');
const version = JSON.parse(readFileSync(path.join(SRC_TAURI, 'tauri.conf.json'), 'utf-8')).version;
const installers = readdirSync(X64_DIR).filter(f => f.endsWith('.exe'));
const platforms = {};

for (const f of installers) {
  const sigPath = path.join(X64_DIR, f + '.sig');
  if (!existsSync(sigPath)) {
    console.error(`  [ERROR] Signature not found: ${f}.sig`);
    process.exit(1);
  }
  const sig = readFileSync(sigPath, 'utf-8').trim();
  const key = f.includes('x64') ? 'windows-x86_64' : 'windows-i686';
  platforms[key] = {
    signature: sig,
    url: `https://github.com/XELZSSS/YMusic/releases/download/v${version}/${encodeURIComponent(f)}`,
  };
  console.log(`  signed ${f}`);
}

const updater = {
  version,
  notes: '',
  pub_date: new Date().toISOString(),
  platforms,
};

writeFileSync(path.join(X64_DIR, 'updater.json'), JSON.stringify(updater, null, 2));
console.log('  -> updater.json');

console.log('\n[5/5] Cleaning up x86 build artifacts...');
const X86_TARGET = path.join(SRC_TAURI, 'target', 'i686-pc-windows-msvc');
if (existsSync(X86_TARGET)) {
  rmSync(X86_TARGET, { recursive: true, force: true });
  console.log('  removed x86 build artifacts');
}

console.log('\nDone! Bundles at: src-tauri/target/release/bundle/nsis/');
for (const f of readdirSync(X64_DIR)) {
  console.log(`  - ${f}`);
}
