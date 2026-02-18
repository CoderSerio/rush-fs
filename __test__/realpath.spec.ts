import test from 'ava'
import { realpathSync, realpath } from '../index.js'
import * as nodeFs from 'node:fs'
import * as path from 'node:path'

test('realpathSync: should resolve to absolute path', (t) => {
  const result = realpathSync('.')
  t.true(path.isAbsolute(result))
})

test('realpathSync: should match node:fs realpathSync', (t) => {
  const nodeResult = nodeFs.realpathSync('.')
  const hyperResult = realpathSync('.')
  t.is(hyperResult, nodeResult)
})

test('realpathSync: should throw on non-existent path', (t) => {
  t.throws(() => realpathSync('./no-such-path'), { message: /ENOENT/ })
})

test('realpath: async should resolve path', async (t) => {
  const result = await realpath('.')
  t.true(path.isAbsolute(result))
})

test('realpath: async should throw on non-existent path', async (t) => {
  await t.throwsAsync(async () => await realpath('./no-such-path'), { message: /ENOENT/ })
})
