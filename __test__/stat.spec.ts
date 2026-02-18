import test from 'ava'
import { statSync, stat, lstatSync, lstat } from '../index.js'
import * as nodeFs from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpDir(): string {
  const dir = join(tmpdir(), `hyper-fs-test-stat-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  nodeFs.mkdirSync(dir, { recursive: true })
  return dir
}

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

test('statSync: atime/mtime/ctime/birthtime should be Date objects', (t) => {
  const s = statSync('./package.json')
  t.true(s.atime instanceof Date)
  t.true(s.mtime instanceof Date)
  t.true(s.ctime instanceof Date)
  t.true(s.birthtime instanceof Date)
  t.true(s.mtime.getTime() > 0)
})

test('statSync: atime.getTime() should be close to atimeMs', (t) => {
  const s = statSync('./package.json')
  t.true(Math.abs(s.atime.getTime() - s.atimeMs) < 1000)
})

test('statSync: should match node:fs atime/mtime Date values', (t) => {
  const nodeStat = nodeFs.statSync('./package.json')
  const hyperStat = statSync('./package.json')
  t.is(hyperStat.mtime.getTime(), nodeStat.mtime.getTime())
})

test('lstatSync: dual-run — symlink should report isSymbolicLink()', (t) => {
  const dir = tmpDir()
  const target = join(dir, 'target.txt')
  const link = join(dir, 'link.txt')
  nodeFs.writeFileSync(target, 'hello')
  nodeFs.symlinkSync(target, link)

  const nodeLstat = nodeFs.lstatSync(link)
  const hyperLstat = lstatSync(link)

  t.is(hyperLstat.isSymbolicLink(), nodeLstat.isSymbolicLink())
  t.true(hyperLstat.isSymbolicLink())
  t.is(hyperLstat.isFile(), nodeLstat.isFile())
  t.false(hyperLstat.isFile())
})

test('statSync: dual-run — stat follows symlink (shows target not link)', (t) => {
  const dir = tmpDir()
  const target = join(dir, 'target.txt')
  const link = join(dir, 'link.txt')
  nodeFs.writeFileSync(target, 'hello')
  nodeFs.symlinkSync(target, link)

  const nodeStat = nodeFs.statSync(link)
  const hyperStat = statSync(link)

  t.is(hyperStat.isFile(), nodeStat.isFile())
  t.true(hyperStat.isFile())
  t.is(hyperStat.isSymbolicLink(), nodeStat.isSymbolicLink())
  t.false(hyperStat.isSymbolicLink())
})
