import test from 'ava'
import { utimesSync, utimes, statSync } from '../index.js'
import { writeFileSync, mkdirSync, statSync as nodeStatSync, utimesSync as nodeUtimesSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpFile(name: string): string {
  const dir = join(tmpdir(), `hyper-fs-test-utimes-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  mkdirSync(dir, { recursive: true })
  const file = join(dir, name)
  writeFileSync(file, 'test')
  return file
}

test('utimesSync: should update atime and mtime', (t) => {
  const file = tmpFile('utimes.txt')
  const atime = 1000
  const mtime = 2000

  utimesSync(file, atime, mtime)
  const s = statSync(file)

  t.is(Math.floor(s.atimeMs / 1000), atime)
  t.is(Math.floor(s.mtimeMs / 1000), mtime)
})

test('utimesSync: should match node:fs behavior', (t) => {
  const file1 = tmpFile('node-utimes.txt')
  const file2 = tmpFile('hyper-utimes.txt')
  const atime = 1500000000
  const mtime = 1600000000

  nodeUtimesSync(file1, atime, mtime)
  utimesSync(file2, atime, mtime)

  const nodeStat = nodeStatSync(file1)
  const hyperStat = statSync(file2)

  t.is(Math.floor(hyperStat.atimeMs / 1000), Math.floor(nodeStat.atimeMs / 1000))
  t.is(Math.floor(hyperStat.mtimeMs / 1000), Math.floor(nodeStat.mtimeMs / 1000))
})

test('utimesSync: should throw on non-existent file', (t) => {
  t.throws(() => utimesSync('/tmp/no-such-file-' + Date.now(), 1000, 2000), { message: /ENOENT/ })
})

test('utimes: async should update times', async (t) => {
  const file = tmpFile('async-utimes.txt')
  const atime = 1000
  const mtime = 2000

  await utimes(file, atime, mtime)
  const s = statSync(file)

  t.is(Math.floor(s.atimeMs / 1000), atime)
  t.is(Math.floor(s.mtimeMs / 1000), mtime)
})

test('utimes: async should throw on non-existent file', async (t) => {
  await t.throwsAsync(async () => await utimes('/tmp/no-such-file-' + Date.now(), 1000, 2000), {
    message: /ENOENT/,
  })
})
