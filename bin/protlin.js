#!/usr/bin/env node
/**
 * Protlin SDK CLI wrapper
 * Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

function getBinaryPath() {
  const platform = os.platform();
  let binaryExt = platform === 'win32' ? '.exe' : '';
  
  const binaryPath = path.join(__dirname, '..', 'sdk', 'target', 'debug', 'protlin' + binaryExt);
  
  if (!fs.existsSync(binaryPath)) {
    console.error('[ERROR] Protlin binary not found. Please run: npm run build');
    process.exit(1);
  }
  
  return binaryPath;
}

const binaryPath = getBinaryPath();
const args = process.argv.slice(2);

const child = spawn(binaryPath, args, {
  stdio: 'inherit',
  shell: false
});

child.on('error', (err) => {
  console.error('Error executing protlin:', err.message);
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code || 0);
});
