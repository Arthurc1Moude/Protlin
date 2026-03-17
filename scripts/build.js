#!/usr/bin/env node
/**
 * Build script for Protlin
 */

const { execSync } = require('child_process');

console.log('>> Building Protlin...\n');

try {
  console.log('Building SDK...');
  execSync('cargo build --manifest-path sdk/Cargo.toml --release', { stdio: 'inherit' });
  
  console.log('\nBuilding 4protlin...');
  execSync('cargo build --manifest-path 4protlin/Cargo.toml --release', { stdio: 'inherit' });
  
  console.log('\n[OK] Build complete!');
} catch (err) {
  console.error('[ERROR] Build failed');
  process.exit(1);
}
