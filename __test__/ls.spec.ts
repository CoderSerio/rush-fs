import test from 'ava'
import { ls } from '../index.js'

test('ls should list files in current directory', (t) => {
  const files = ls('.')

  // asserts
  t.true(Array.isArray(files))
  t.true(files.length > 0)
  t.true(
    files.some((f: string) => f.includes('package.json')),
    'Result should contain package.json',
  )
})
