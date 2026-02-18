import test from 'ava'
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
