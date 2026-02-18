import test from 'ava'
import { mkdirSync, mkdir, rmdirSync } from '../index.js'
import { existsSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpPath(name: string): string {
  return join(tmpdir(), `hyper-fs-test-mkdir-${Date.now()}-${name}`)
}

test('mkdirSync: should create a directory', (t) => {
  const dir = tmpPath('basic')
  mkdirSync(dir)
  t.true(existsSync(dir))
  rmdirSync(dir)
})

test('mkdirSync: should throw on existing directory without recursive', (t) => {
  const dir = tmpPath('existing')
  mkdirSync(dir)
  t.throws(() => mkdirSync(dir), { message: /EEXIST/ })
  rmdirSync(dir)
})

test('mkdirSync: recursive should create nested dirs', (t) => {
  const dir = tmpPath('recursive')
  const nested = join(dir, 'a', 'b', 'c')
  mkdirSync(nested, { recursive: true })
  t.true(existsSync(nested))
  // cleanup
  rmdirSync(join(dir, 'a', 'b', 'c'))
  rmdirSync(join(dir, 'a', 'b'))
  rmdirSync(join(dir, 'a'))
  rmdirSync(dir)
})

test('mkdirSync: recursive should not throw if dir already exists', (t) => {
  const dir = tmpPath('recursive-exists')
  mkdirSync(dir)
  t.notThrows(() => mkdirSync(dir, { recursive: true }))
  rmdirSync(dir)
})

test('mkdir: async should create a directory', async (t) => {
  const dir = tmpPath('async')
  await mkdir(dir)
  t.true(existsSync(dir))
  rmdirSync(dir)
})

test('mkdir: async recursive', async (t) => {
  const dir = tmpPath('async-recursive')
  const nested = join(dir, 'x', 'y')
  await mkdir(nested, { recursive: true })
  t.true(existsSync(nested))
  rmdirSync(join(dir, 'x', 'y'))
  rmdirSync(join(dir, 'x'))
  rmdirSync(dir)
})
