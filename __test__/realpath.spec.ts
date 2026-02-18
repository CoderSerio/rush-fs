import test from 'ava'
import { realpathSync, realpath } from '../index.js'
import * as nodeFs from 'node:fs'
import * as path from 'node:path'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpDir(): string {
  const dir = join(tmpdir(), `hyper-fs-test-realpath-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  nodeFs.mkdirSync(dir, { recursive: true })
  return dir
}

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

test('dual-run: realpathSync should resolve symlink to real path', (t) => {
  const dir = tmpDir()
  const target = join(dir, 'real-target.txt')
  const link = join(dir, 'link.txt')
  nodeFs.writeFileSync(target, 'hello')
  nodeFs.symlinkSync(target, link)

  const nodeResult = nodeFs.realpathSync(link)
  const hyperResult = realpathSync(link)
  // Compare against node:fs (not raw `target`): on macOS /tmp is a symlink to /private/tmp,
  // so realpath resolves through it.
  t.is(hyperResult, nodeResult)
  // The resolved path should end with the target filename
  t.true(hyperResult.endsWith('real-target.txt'))
})

test('dual-run: realpathSync should resolve relative path same as node:fs', (t) => {
  const nodeResult = nodeFs.realpathSync('src')
  const hyperResult = realpathSync('src')
  t.is(hyperResult, nodeResult)
})

test('realpath: async dual-run should resolve symlink same as node:fs', async (t) => {
  const dir = tmpDir()
  const target = join(dir, 'async-target.txt')
  const link = join(dir, 'async-link.txt')
  nodeFs.writeFileSync(target, 'hello')
  nodeFs.symlinkSync(target, link)

  const nodeResult = nodeFs.realpathSync(link)
  const hyperResult = await realpath(link)
  t.is(hyperResult, nodeResult)
})
