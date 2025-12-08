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

test('globSync: should respect git_ignore (default: true)', (t) => {
  // 'target' directory is usually gitignored in Rust projects
  // Note: This test assumes 'target' directory exists and is ignored.
  // If running in a fresh clone without build, target might not exist.
  // We can skip if target doesn't exist, or just check node_modules which is definitely ignored?
  // node_modules is ignored by default in many setups but strict gitignore check depends on .gitignore file.

  // Let's assume 'target' exists because we built the project
  const ignoredFiles = globSync('target/**/*.d', { cwd: CWD })
  // Should be empty or very few if ignored (actually cargo ignores target/)
  // But wait, standard_filters includes .ignore and .gitignore.

  // If we force git_ignore: false, we should see files
  const includedFiles = globSync('target/**/*.d', { cwd: CWD, git_ignore: false })

  if (includedFiles.length > 0) {
    t.true(ignoredFiles.length < includedFiles.length, 'Should find fewer files when respecting gitignore')
  } else {
    t.pass('Target directory empty or not present, skipping git_ignore comparison')
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
