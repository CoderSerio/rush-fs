import test from 'ava'
import { readdirSync, readdir } from '../src/wrapper/index'

test('sync: should list files in current directory', (t) => {
  const files = readdirSync('.')

  t.true(Array.isArray(files))
  t.true(files.length > 0)

  // Verify Dirent structure
  const packageJson = files.find((f) => f.name === 'package.json')
  console.log(packageJson)
  t.truthy(packageJson, 'Result should contain package.json')
  t.is(packageJson?.isDir, false)
  t.true(packageJson?.path.includes('package.json'))

  const srcDir = files.find((f) => f.name === 'src')
  if (srcDir) {
    t.is(srcDir.isDir, true, 'src should be identified as a directory')
  }
})

test('async: should list files in current directory', async (t) => {
  const files = await readdir('.')
  t.true(files.length > 0)
  t.truthy(files.find((f) => f.name === 'package.json'))
})

test('concurrency: run with specific thread count', (t) => {
  const files = readdirSync('.', {
    concurrency: 4,
  })
  t.true(files.length > 0)
})

test('concurrency: run with high thread count (stress test)', (t) => {
  const files = readdirSync('.', {
    concurrency: 100,
  })
  t.true(files.length > 0)
})

test('options: skip_hidden should filter out dotfiles', (t) => {
  // First, ensure we can see hidden files (default behavior usually depends on impl,
  // but based on your rust code, default is false)
  const allFiles = readdirSync('.', { skipHidden: false })
  // Assuming this repo has a .git folder or similar
  const hasHidden = allFiles.some((f) => f.name.startsWith('.'))

  if (hasHidden) {
    const visibleFiles = readdirSync('.', { skipHidden: true })
    const hiddenRemains = visibleFiles.some((f) => f.name.startsWith('.'))
    t.false(hiddenRemains, 'Should not contain hidden files when skip_hidden is true')
  } else {
    t.pass('No hidden files found in root to test skipping')
  }
})

test('error: should throw on non-existent directory', async (t) => {
  t.throws(() => readdirSync('./path/to/nowhere'))
  await t.throwsAsync(async () => await readdir('./path/to/nowhere'))
})
