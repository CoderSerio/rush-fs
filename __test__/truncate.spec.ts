import test from 'ava'
import { truncateSync, truncate } from '../index.js'
import { writeFileSync, readFileSync, mkdirSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpFile(name: string): string {
  const dir = join(tmpdir(), `hyper-fs-test-truncate-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  mkdirSync(dir, { recursive: true })
  const file = join(dir, name)
  writeFileSync(file, 'hello world 12345')
  return file
}

test('truncateSync: should truncate to 0 by default', (t) => {
  const file = tmpFile('zero.txt')
  truncateSync(file)
  const content = readFileSync(file, 'utf8')
  t.is(content, '')
})

test('truncateSync: should truncate to specific length', (t) => {
  const file = tmpFile('len.txt')
  truncateSync(file, 5)
  const content = readFileSync(file, 'utf8')
  t.is(content, 'hello')
})

test('truncateSync: should throw on non-existent file', (t) => {
  t.throws(() => truncateSync('/tmp/no-such-file-' + Date.now()), { message: /ENOENT/ })
})

test('truncate: async should truncate', async (t) => {
  const file = tmpFile('async.txt')
  await truncate(file, 5)
  const content = readFileSync(file, 'utf8')
  t.is(content, 'hello')
})
