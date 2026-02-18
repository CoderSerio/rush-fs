import test from 'ava'
import { writeFileSync, writeFile, readFileSync, appendFileSync, appendFile } from '../index.js'
import { existsSync, unlinkSync, mkdirSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

function tmpFile(name: string): string {
  const dir = join(tmpdir(), `hyper-fs-test-write-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  mkdirSync(dir, { recursive: true })
  return join(dir, name)
}

test('writeFileSync: should write string to file', (t) => {
  const file = tmpFile('str.txt')
  writeFileSync(file, 'hello world')
  const content = readFileSync(file, { encoding: 'utf8' })
  t.is(content, 'hello world')
})

test('writeFileSync: should write Buffer to file', (t) => {
  const file = tmpFile('buf.txt')
  writeFileSync(file, Buffer.from('buffer data'))
  const content = readFileSync(file, { encoding: 'utf8' })
  t.is(content, 'buffer data')
})

test('writeFileSync: should overwrite existing file', (t) => {
  const file = tmpFile('overwrite.txt')
  writeFileSync(file, 'first')
  writeFileSync(file, 'second')
  const content = readFileSync(file, { encoding: 'utf8' })
  t.is(content, 'second')
})

test('writeFile: async should write file', async (t) => {
  const file = tmpFile('async.txt')
  await writeFile(file, 'async content')
  const content = readFileSync(file, { encoding: 'utf8' })
  t.is(content, 'async content')
})

test('appendFileSync: should append to file', (t) => {
  const file = tmpFile('append.txt')
  writeFileSync(file, 'start')
  appendFileSync(file, '-end')
  const content = readFileSync(file, { encoding: 'utf8' })
  t.is(content, 'start-end')
})

test('appendFile: async should append', async (t) => {
  const file = tmpFile('append-async.txt')
  writeFileSync(file, 'a')
  await appendFile(file, 'b')
  const content = readFileSync(file, { encoding: 'utf8' })
  t.is(content, 'ab')
})
