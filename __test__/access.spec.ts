import test from 'ava'
import * as nodeFs from 'node:fs'
import { accessSync, access } from '../index.js'

const F_OK = 0
const R_OK = 4
const W_OK = 2

test('accessSync: should succeed for existing file (F_OK)', (t) => {
  t.notThrows(() => accessSync('./package.json'))
})

test('accessSync: should succeed with explicit F_OK', (t) => {
  t.notThrows(() => accessSync('./package.json', F_OK))
})

test('accessSync: should succeed with R_OK', (t) => {
  t.notThrows(() => accessSync('./package.json', R_OK))
})

test('accessSync: should succeed with W_OK', (t) => {
  t.notThrows(() => accessSync('./package.json', W_OK))
})

test('accessSync: should throw on non-existent file', (t) => {
  t.throws(() => accessSync('./no-such-file'), { message: /ENOENT/ })
})

test('access: async should succeed for existing file', async (t) => {
  await t.notThrowsAsync(async () => await access('./package.json'))
})

test('access: async should throw on non-existent file', async (t) => {
  await t.throwsAsync(async () => await access('./no-such-file'), { message: /ENOENT/ })
})

// ===== dual-run comparison =====

test('dual-run: accessSync should behave same as node:fs for existing file', (t) => {
  let nodeErr: Error | null = null
  let hyperErr: Error | null = null

  try {
    nodeFs.accessSync('./package.json', R_OK)
  } catch (e) {
    nodeErr = e as Error
  }
  try {
    accessSync('./package.json', R_OK)
  } catch (e) {
    hyperErr = e as Error
  }

  t.is(hyperErr, nodeErr)
})

test('dual-run: accessSync should both throw for non-existent file', (t) => {
  const target = './no-such-file-access-dual-' + Date.now()
  let nodeThrew = false
  let hyperThrew = false

  try {
    nodeFs.accessSync(target)
  } catch {
    nodeThrew = true
  }
  try {
    accessSync(target)
  } catch {
    hyperThrew = true
  }

  t.is(hyperThrew, nodeThrew)
})
