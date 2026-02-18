import test from 'ava'
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
