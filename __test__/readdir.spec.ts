import test from 'ava'
import { readdirSync, readdir } from '../index.js'

test('sync: should list files in current directory (strings by default)', (t) => {
  const files = readdirSync('.')

  t.true(Array.isArray(files))
  t.true(files.length > 0)

  // Verify it returns strings
  t.is(typeof files[0], 'string')

  const packageJson = files.find((f) => f === 'package.json')
  t.truthy(packageJson, 'Result should contain package.json')
})

test('sync: should return Dirent objects when withFileTypes is true', (t) => {
  const files = readdirSync('.', { withFileTypes: true })

  t.true(Array.isArray(files))
  t.true(files.length > 0)

  // Verify Dirent structure
  // We need to cast or check type because typescript might infer union type
  const first = files[0]
  if (typeof first === 'object') {
    t.is(typeof first.name, 'string')
    t.is(typeof first.isDir, 'boolean')
  } else {
    t.fail('Should return objects when withFileTypes is true')
  }

  const packageJson = files.find((f) => typeof f !== 'string' && f.name === 'package.json')
  t.truthy(packageJson, 'Result should contain package.json')

  if (typeof packageJson !== 'string' && packageJson) {
    t.is(packageJson.isDir, false)
  }

  const srcDir = files.find((f) => typeof f !== 'string' && f.name === 'src')
  if (srcDir && typeof srcDir !== 'string') {
    t.is(srcDir.isDir, true, 'src should be identified as a directory')
  }
})

test('async: should list files in current directory', async (t) => {
  const files = await readdir('.')
  t.true(files.length > 0)
  t.is(typeof files[0], 'string')
  t.truthy(files.find((f) => f === 'package.json'))
})

test('concurrency: run with specific thread count', (t) => {
  const files = readdirSync('.', {
    concurrency: 4,
    recursive: true, // concurrency only works with recursive/walk_dir
  })
  t.true(files.length > 0)
})

test('concurrency: run with high thread count (stress test)', (t) => {
  const files = readdirSync('.', {
    concurrency: 100,
    recursive: true,
  })
  t.true(files.length > 0)
})

test('options: skip_hidden should filter out dotfiles', (t) => {
  // First, ensure we can see hidden files (default behavior usually depends on impl,
  // but based on your rust code, default is false)
  const allFiles = readdirSync('.', { skipHidden: false })
  // Assuming this repo has a .git folder or similar
  // files are strings now
  const hasHidden = allFiles.some((f) => (typeof f === 'string' ? f : f.name).startsWith('.'))

  if (hasHidden) {
    const visibleFiles = readdirSync('.', { skipHidden: true })
    const hiddenRemains = visibleFiles.some((f) => (typeof f === 'string' ? f : f.name).startsWith('.'))
    t.false(hiddenRemains, 'Should not contain hidden files when skip_hidden is true')
  } else {
    t.pass('No hidden files found in root to test skipping')
  }
})

test('error: should throw on non-existent directory', async (t) => {
  t.throws(() => readdirSync('./path/to/nowhere'))
  await t.throwsAsync(async () => await readdir('./path/to/nowhere'))
})
