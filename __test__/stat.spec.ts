import test from 'ava'
import { statSync, stat, lstatSync, lstat } from '../index.js'
import * as nodeFs from 'node:fs'

test('statSync: should return stats for a file', (t) => {
  const s = statSync('./package.json')
  t.is(typeof s.size, 'number')
  t.true(s.size > 0)
  t.true(s.isFile())
  t.false(s.isDirectory())
})

test('statSync: should return stats for a directory', (t) => {
  const s = statSync('./src')
  t.true(s.isDirectory())
  t.false(s.isFile())
})

test('statSync: should match node:fs stat values', (t) => {
  const nodeStat = nodeFs.statSync('./package.json')
  const hyperStat = statSync('./package.json')

  t.is(hyperStat.size, nodeStat.size)
  t.is(hyperStat.isFile(), nodeStat.isFile())
  t.is(hyperStat.isDirectory(), nodeStat.isDirectory())
  t.is(hyperStat.isSymbolicLink(), nodeStat.isSymbolicLink())
  t.is(hyperStat.mode, nodeStat.mode)
  t.is(hyperStat.uid, nodeStat.uid)
  t.is(hyperStat.gid, nodeStat.gid)
  t.is(hyperStat.nlink, nodeStat.nlink)
})

test('statSync: should throw on non-existent path', (t) => {
  t.throws(() => statSync('./no-such-file'), { message: /ENOENT/ })
})

test('stat: async should return stats', async (t) => {
  const s = await stat('./package.json')
  t.true(s.isFile())
  t.true(s.size > 0)
})

test('stat: async should throw on non-existent path', async (t) => {
  await t.throwsAsync(async () => await stat('./no-such-file'), { message: /ENOENT/ })
})

test('lstatSync: should return stats without following symlinks', (t) => {
  const s = lstatSync('./package.json')
  t.true(s.isFile())
})

test('lstat: async should work', async (t) => {
  const s = await lstat('./package.json')
  t.true(s.isFile())
})

test('statSync: atimeMs/mtimeMs/ctimeMs/birthtimeMs should be numbers', (t) => {
  const s = statSync('./package.json')
  t.is(typeof s.atimeMs, 'number')
  t.is(typeof s.mtimeMs, 'number')
  t.is(typeof s.ctimeMs, 'number')
  t.is(typeof s.birthtimeMs, 'number')
  t.true(s.mtimeMs > 0)
})
