import { run, bench, group } from 'mitata'
import { globSync as hyperGlobSync } from '../index.js'
import { globSync as nodeGlobSync } from 'glob'
import fastGlob from 'fast-glob'

const cwd = process.cwd()

// Patterns to test
const patternSimple = 'src/*.rs'
const patternRecursive = '**/*.rs'
const patternDeep = 'node_modules/**/*.json'

console.log(`Benchmarking glob in: ${cwd}`)

// 1. Simple Flat Glob
group('Glob (Simple: src/*.rs)', () => {
  bench('node-glob', () => nodeGlobSync(patternSimple, { cwd }))
  bench('fast-glob', () => fastGlob.sync(patternSimple, { cwd }))
  bench('rush-fs', () => hyperGlobSync(patternSimple, { cwd })).baseline()
})

// 2. Recursive Glob
group('Glob (Recursive: **/*.rs)', () => {
  bench('node-glob', () => nodeGlobSync(patternRecursive, { cwd }))
  bench('fast-glob', () => fastGlob.sync(patternRecursive, { cwd }))
  bench('rush-fs', () => hyperGlobSync(patternRecursive, { cwd })).baseline()
})

// 3. Deep Recursive (if node_modules exists)
// This is a stress test
group('Glob (Deep: node_modules/**/*.json)', () => {
  // Only run if node_modules exists to avoid empty result bias
  const hasNodeModules = fastGlob.sync('node_modules').length > 0
  if (hasNodeModules) {
    bench('node-glob', () => nodeGlobSync(patternDeep, { cwd }))
    bench('fast-glob', () => fastGlob.sync(patternDeep, { cwd }))
    bench('rush-fs', () => hyperGlobSync(patternDeep, { cwd })).baseline()
    bench('rush-fs (8 threads)', () => hyperGlobSync(patternDeep, { cwd, concurrency: 8 }))
  }
})

await run({
  colors: true,
})
