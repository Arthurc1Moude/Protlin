#!/usr/bin/env node
/**
 * Protlin Programming Language - NPM Package
 * Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

const PROTLIN_VERSION = '1.0.0';

/**
 * Get the platform-specific binary path
 */
function getBinaryPath(binaryName) {
  const platform = os.platform();
  const arch = os.arch();
  
  let binaryExt = '';
  if (platform === 'win32') {
    binaryExt = '.exe';
  }
  
  // Check if binaries are built
  const sdkBinary = path.join(__dirname, 'sdk', 'target', 'debug', binaryName + binaryExt);
  const forprotlinBinary = path.join(__dirname, '4protlin', 'target', 'debug', binaryName + binaryExt);
  
  if (fs.existsSync(sdkBinary)) {
    return sdkBinary;
  }
  
  if (fs.existsSync(forprotlinBinary)) {
    return forprotlinBinary;
  }
  
  throw new Error(`Binary not found: ${binaryName}. Please run 'npm run build' first.`);
}

/**
 * Execute a Protlin binary
 */
function executeBinary(binaryName, args) {
  try {
    const binaryPath = getBinaryPath(binaryName);
    
    const child = spawn(binaryPath, args, {
      stdio: 'inherit',
      shell: false
    });
    
    child.on('error', (err) => {
      console.error(`Error executing ${binaryName}:`, err.message);
      process.exit(1);
    });
    
    child.on('exit', (code) => {
      process.exit(code || 0);
    });
  } catch (err) {
    console.error(err.message);
    process.exit(1);
  }
}

/**
 * Main API
 */
module.exports = {
  version: PROTLIN_VERSION,
  
  /**
   * Run a Protlin file
   */
  run: function(file, options = {}) {
    const args = ['run', file];
    executeBinary('protlin', args);
  },
  
  /**
   * Start REPL
   */
  repl: function() {
    executeBinary('protlin', ['repl']);
  },
  
  /**
   * Check syntax
   */
  check: function(file) {
    executeBinary('protlin', ['check', file]);
  },
  
  /**
   * Install package
   */
  install: function(packageName, options = {}) {
    const args = ['install', packageName];
    if (options.native) args.push('--native');
    if (options.server) args.push('--server');
    executeBinary('4protlin', args);
  },
  
  /**
   * List packages
   */
  list: function() {
    executeBinary('4protlin', ['list']);
  },
  
  /**
   * Search packages
   */
  search: function(query) {
    executeBinary('4protlin', ['search', query]);
  },
  
  /**
   * Get binary paths
   */
  getBinaries: function() {
    return {
      protlin: getBinaryPath('protlin'),
      '4protlin': getBinaryPath('4protlin')
    };
  }
};

// If run directly from command line
if (require.main === module) {
  console.log('Protlin Programming Language v' + PROTLIN_VERSION);
  console.log('Copyright © 2026 Moude AI LLC and Moude Corp');
  console.log('');
  console.log('Usage:');
  console.log('  protlin <command>   - Run Protlin SDK');
  console.log('  4protlin <command>  - Run package manager');
  console.log('');
  console.log('Or use as a Node.js module:');
  console.log('  const protlin = require("protlin");');
  console.log('  protlin.run("file.prot");');
}
