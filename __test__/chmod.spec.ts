import test from 'ava'
import { chmodSync, chmod, statSync } from '../index.js'
import { writeFileSync, mkdirSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpFile(name: string): string {
  const dir = join(tmpdir(), `hyper-fs-test-chmod-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  mkdirSync(dir, { recursive: true })
  const file = join(dir, name)
  writeFileSync(file, 'test')
  return file
}

test('chmodSync: should change file permissions', (t) => {
  if (process.platform === 'win32') {
    t.pass('Skipping chmod test on Windows')
    return
  }
  const file = tmpFile('chmod.txt')
  chmodSync(file, 0o644)
  const s = statSync(file)
  t.is(s.mode & 0o777, 0o644)

  chmodSync(file, 0o755)
  const s2 = statSync(file)
  t.is(s2.mode & 0o777, 0o755)
})

test('chmodSync: should throw on non-existent file', (t) => {
  t.throws(() => chmodSync('/tmp/no-such-file-' + Date.now(), 0o644), { message: /ENOENT/ })
})

test('chmod: async should change permissions', async (t) => {
  if (process.platform === 'win32') {
    t.pass('Skipping chmod test on Windows')
    return
  }
  const file = tmpFile('async-chmod.txt')
  await chmod(file, 0o600)
  const s = statSync(file)
  t.is(s.mode & 0o777, 0o600)
})
