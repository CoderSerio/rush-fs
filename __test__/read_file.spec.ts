import test from 'ava'
import { readFileSync, readFile, writeFileSync } from '../index.js'
import * as nodeFs from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

test('readFileSync: should read file as Buffer by default', (t) => {
  const result = readFileSync('./package.json')
  t.true(Buffer.isBuffer(result))
  t.true((result as Buffer).length > 0)
})

test('readFileSync: should read file as string with encoding=utf8', (t) => {
  const result = readFileSync('./package.json', { encoding: 'utf8' })
  t.is(typeof result, 'string')
  t.true((result as string).includes('hyper-fs'))
})

test('readFileSync: should match node:fs readFileSync', (t) => {
  const nodeResult = nodeFs.readFileSync('./package.json', 'utf8')
  const hyperResult = readFileSync('./package.json', { encoding: 'utf8' })
  t.is(hyperResult, nodeResult)
})

test('readFileSync: should throw on non-existent file', (t) => {
  t.throws(() => readFileSync('./no-such-file'), { message: /ENOENT/ })
})

test('readFile: async should read file', async (t) => {
  const result = await readFile('./package.json', { encoding: 'utf8' })
  t.is(typeof result, 'string')
  t.true((result as string).includes('hyper-fs'))
})

test('readFile: async should throw on non-existent file', async (t) => {
  await t.throwsAsync(async () => await readFile('./no-such-file'), { message: /ENOENT/ })
})

test('dual-run: readFileSync Buffer should match node:fs byte-for-byte', (t) => {
  const nodeResult = nodeFs.readFileSync('./package.json')
  const hyperResult = readFileSync('./package.json') as Buffer
  t.true(Buffer.isBuffer(hyperResult))
  t.deepEqual(hyperResult, nodeResult)
})

test('dual-run: readFileSync utf8 string should match node:fs', (t) => {
  const nodeResult = nodeFs.readFileSync('./package.json', 'utf8')
  const hyperResult = readFileSync('./package.json', { encoding: 'utf8' }) as string
  t.is(hyperResult, nodeResult)
})
