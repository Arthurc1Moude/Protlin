#!/usr/bin/env node
/**
 * Post-installation script
 */

const fs = require('fs');
const path = require('path');

// Make binaries executable on Unix systems
if (process.platform !== 'win32') {
  const binDir = path.join(__dirname, '..', 'bin');
  
  if (fs.existsSync(binDir)) {
    const files = fs.readdirSync(binDir);
    files.forEach(file => {
      const filePath = path.join(binDir, file);
      try {
        fs.chmodSync(filePath, '755');
      } catch (err) {
        // Ignore errors
      }
    });
  }
}

console.log('\n>> Protlin is ready to use!');
console.log('   Run: protlin --help');
