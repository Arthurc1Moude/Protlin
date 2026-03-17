#!/usr/bin/env node
/**
 * Test script for Protlin NPM package
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('>> Protlin NPM Package - Test Suite\n');

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    console.log(`>> ${name}...`);
    fn();
    console.log('   [OK] PASSED\n');
    passed++;
  } catch (err) {
    console.log(`   [ERROR] FAILED: ${err.message}\n`);
    failed++;
  }
}

// Test binaries exist
test('Binaries exist', () => {
  const protlinBinary = path.join(__dirname, '..', 'sdk', 'target', 'debug', 'protlin');
  const forprotlinBinary = path.join(__dirname, '..', '4protlin', 'target', 'debug', '4protlin');
  
  if (!fs.existsSync(protlinBinary)) throw new Error('protlin binary not found');
  if (!fs.existsSync(forprotlinBinary)) throw new Error('4protlin binary not found');
});

// Test protlin help
test('Protlin --help', () => {
  const output = execSync('node bin/protlin.js --help', { encoding: 'utf8', cwd: path.join(__dirname, '..') });
  if (!output.includes('Protlin SDK')) throw new Error('Help output incorrect');
});

// Test 4protlin help
test('4protlin --help', () => {
  const output = execSync('node bin/4protlin.js --help', { encoding: 'utf8', cwd: path.join(__dirname, '..') });
  if (!output.includes('4protlin')) throw new Error('Help output incorrect');
});

// Test run hello.prot
test('Run hello.prot', () => {
  const output = execSync('node bin/protlin.js run examples/hello.prot', { encoding: 'utf8', cwd: path.join(__dirname, '..') });
  if (!output.includes('Hello, Protlin!')) throw new Error('Output incorrect');
});

// Test API module
test('API module', () => {
  const protlin = require('../index.js');
  if (protlin.version !== '1.0.0') throw new Error('Version incorrect');
  const binaries = protlin.getBinaries();
  if (!binaries.protlin || !binaries['4protlin']) throw new Error('getBinaries failed');
});

console.log('='.repeat(50));
console.log(`\n>> Results: ${passed} passed, ${failed} failed\n`);

if (failed === 0) {
  console.log('[OK] All tests passed!');
  process.exit(0);
} else {
  console.error('[ERROR] Some tests failed!');
  process.exit(1);
}
