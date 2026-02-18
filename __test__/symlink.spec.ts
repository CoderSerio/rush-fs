import test from 'ava'
import { symlinkSync, symlink, readlinkSync, statSync, lstatSync } from '../index.js'
import { writeFileSync, mkdirSync, existsSync, readlinkSync as nodeReadlinkSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpDir(): string {
  const dir = join(tmpdir(), `hyper-fs-test-symlink-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  mkdirSync(dir, { recursive: true })
  return dir
}

// ===== sync =====

test('symlinkSync: should create a symbolic link to a file', (t) => {
  const dir = tmpDir()
  const target = join(dir, 'target.txt')
  const link = join(dir, 'link.txt')
  writeFileSync(target, 'hello')

  symlinkSync(target, link)
  t.true(existsSync(link))

  const resolved = readlinkSync(link)
  t.is(resolved, target)
})

test('symlinkSync: should create a symbolic link to a directory', (t) => {
  const dir = tmpDir()
  const targetDir = join(dir, 'subdir')
  mkdirSync(targetDir)
  const link = join(dir, 'link-dir')

  symlinkSync(targetDir, link)
  t.true(lstatSync(link).isSymbolicLink())
  t.true(statSync(link).isDirectory())
})

test('symlinkSync: should match node:fs readlink result', (t) => {
  const dir = tmpDir()
  const target = join(dir, 'target2.txt')
  const link = join(dir, 'link2.txt')
  writeFileSync(target, 'hello')

  symlinkSync(target, link)
  const nodeResult = nodeReadlinkSync(link, 'utf8')
  const hyperResult = readlinkSync(link)
  t.is(hyperResult, nodeResult)
})

test('symlinkSync: should throw EEXIST if link path already exists', (t) => {
  const dir = tmpDir()
  const target = join(dir, 'target3.txt')
  const link = join(dir, 'link3.txt')
  writeFileSync(target, 'hello')
  writeFileSync(link, 'existing')

  t.throws(() => symlinkSync(target, link), { message: /EEXIST/ })
})

// ===== async =====

test('symlink: async should create a symbolic link', async (t) => {
  const dir = tmpDir()
  const target = join(dir, 'async-target.txt')
  const link = join(dir, 'async-link.txt')
  writeFileSync(target, 'hello')

  await symlink(target, link)
  t.true(existsSync(link))

  const resolved = readlinkSync(link)
  t.is(resolved, target)
})
