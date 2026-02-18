import test from 'ava'
import * as nodeFs from 'node:fs'
import { existsSync, exists } from '../index.js'

test('existsSync: should return true for existing file', (t) => {
  t.true(existsSync('./package.json'))
})

test('existsSync: should return true for existing directory', (t) => {
  t.true(existsSync('./src'))
})

test('existsSync: should return false for non-existent path', (t) => {
  t.false(existsSync('./no-such-file'))
})

test('exists: async should return true for existing file', async (t) => {
  const result = await exists('./package.json')
  t.true(result)
})

test('exists: async should return false for non-existent file', async (t) => {
  const result = await exists('./no-such-file')
  t.false(result)
})

// ===== dual-run comparison =====

test('dual-run: existsSync should match node:fs.existsSync', (t) => {
  const paths = ['./package.json', './src', './no-such-file', './node_modules', './Cargo.toml']
  for (const p of paths) {
    t.is(existsSync(p), nodeFs.existsSync(p), `existsSync mismatch for "${p}"`)
  }
})
