#!/usr/bin/env node
/**
 * Protlin NPM Package - Installation Script
 * Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('>> Installing Protlin Programming Language...');
console.log('   Copyright © 2026 Moude AI LLC and Moude Corp\n');

// Check if Rust is installed
try {
  execSync('cargo --version', { stdio: 'ignore' });
  console.log('[OK] Rust/Cargo detected');
} catch (err) {
  console.error('[ERROR] Rust/Cargo not found!');
  console.error('');
  console.error('Please install Rust from: https://rustup.rs/');
  console.error('');
  console.error('Linux/macOS:');
  console.error('  curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh');
  console.error('');
  console.error('Windows:');
  console.error('  Download from: https://rustup.rs/');
  process.exit(1);
}

console.log('>> Building Protlin binaries...');
console.log('   This may take a few minutes on first install...\n');

try {
  // Build SDK
  console.log('>> Building Protlin SDK...');
  execSync('cargo build --manifest-path sdk/Cargo.toml', { 
    stdio: 'inherit',
    cwd: __dirname + '/..'
  });
  
  // Build 4protlin
  console.log('\n>> Building 4protlin package manager...');
  execSync('cargo build --manifest-path 4protlin/Cargo.toml', { 
    stdio: 'inherit',
    cwd: __dirname + '/..'
  });
  
  console.log('\n[OK] Build complete!');
  console.log('\n>> Protlin installed successfully!');
  console.log('\nUsage:');
  console.log('  protlin run <file.prot>  - Run a Protlin file');
  console.log('  protlin repl             - Start interactive REPL');
  console.log('  4protlin install <pkg>   - Install a package');
  console.log('  4protlin list            - List installed packages');
  
} catch (err) {
  console.error('\n[ERROR] Build failed:', err.message);
  process.exit(1);
}
