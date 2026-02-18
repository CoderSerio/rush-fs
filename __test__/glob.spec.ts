import test from 'ava'
import { globSync, glob } from '../index.js'
import { join } from 'path'

const CWD = process.cwd()

test('globSync: should find files in current directory', (t) => {
  const files = globSync('*.json', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.endsWith('package.json')))
})

test('globSync: should match files in subdirectories', (t) => {
  const files = globSync('src/*.rs', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.endsWith('lib.rs')))
})

test('globSync: should return Dirent objects when withFileTypes is true', (t) => {
  const files = globSync('src/*.rs', { cwd: CWD, withFileTypes: true })
  t.true(files.length > 0)

  const first = files[0]
  if (typeof first === 'object') {
    t.is(typeof first.isFile, 'function')
    t.true(first.isFile())
    t.is(typeof first.name, 'string')
    t.true(first.name.endsWith('.rs'))
    t.is(typeof first.parentPath, 'string')
  } else {
    t.fail('Should return objects')
  }
})

test('globSync: should support exclude option', (t) => {
  // Should match multiple .rs files normally
  const allFiles = globSync('src/*.rs', { cwd: CWD })
  t.true(allFiles.some((f) => f.endsWith('lib.rs')))

  // Exclude lib.rs
  const filteredFiles = globSync('src/*.rs', { cwd: CWD, exclude: ['lib.rs'] })
  t.true(filteredFiles.length > 0)
  t.false(
    filteredFiles.some((f) => f.endsWith('lib.rs')),
    'Should exclude lib.rs',
  )
  t.true(filteredFiles.length < allFiles.length)
})

test('globSync: should respect gitIgnore (default: true)', (t) => {
  // When gitIgnore: true (default), files in .gitignore are excluded.
  // When gitIgnore: false, they are included.
  const ignoredFiles = globSync('target/**/*.d', { cwd: CWD })
  const includedFiles = globSync('target/**/*.d', { cwd: CWD, gitIgnore: false })

  if (includedFiles.length > 0) {
    t.true(ignoredFiles.length < includedFiles.length, 'Should find fewer files when respecting gitignore')
  } else {
    t.pass('Target directory empty or not present, skipping gitIgnore comparison')
  }
})

test('globSync: concurrency option should not crash', (t) => {
  const files = globSync('src/**/*.rs', { cwd: CWD, concurrency: 2 })
  t.true(files.length > 0)
})

test('async: should work basically', async (t) => {
  const files = await glob('*.json', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.endsWith('package.json')))
})

test('async: withFileTypes', async (t) => {
  const files = await glob('src/*.rs', { cwd: CWD, withFileTypes: true })
  t.true(files.length > 0)
  const first = files[0]
  t.is(typeof first, 'object')
  t.true(first.isFile())
})

test('async: should return empty array for no matches', async (t) => {
  const files = await glob('non_existent_*.xyz', { cwd: CWD })
  t.true(Array.isArray(files))
  t.is(files.length, 0)
})

test('async: recursive match', async (t) => {
  const files = await glob('**/*.rs', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.includes('src/lib.rs')))
})
