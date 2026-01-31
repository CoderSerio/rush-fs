import * as fs from 'node:fs'
import * as path from 'node:path'
import * as os from 'node:os'
import { rmSync as hyperRmSync } from '../index.js'

const tmpDir = os.tmpdir()
const baseDir = path.join(tmpDir, 'hyper-fs-bench-rm')

// Clean up previous runs
if (fs.existsSync(baseDir)) {
  fs.rmSync(baseDir, { recursive: true, force: true })
}
fs.mkdirSync(baseDir)

/**
 * Create a flat directory containing a specified number of files.
 *
 * Ensures the target directory exists and writes `count` files named
 * `file-0.txt` through `file-(count-1).txt`, each containing the string "content".
 *
 * @param dir - The directory path where files will be created
 * @param count - The number of files to create (non-negative integer)
 */
function createFlatStructure(dir: string, count: number) {
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true })
  for (let i = 0; i < count; i++) {
    fs.writeFileSync(path.join(dir, `file-${i}.txt`), 'content')
  }
}

/**
 * Creates a chain of nested subdirectories under the given root and places a file in each level.
 *
 * Ensures the root directory exists, then creates `depth` nested directories named `depth-0`, `depth-1`, ...,
 * and writes a file named `file.txt` containing "content" into each created subdirectory.
 *
 * @param dir - The root directory under which the nested structure will be created
 * @param depth - The number of nested subdirectory levels to create
 */
function createDeepStructure(dir: string, depth: number) {
  let current = dir
  if (!fs.existsSync(current)) fs.mkdirSync(current, { recursive: true })
  for (let i = 0; i < depth; i++) {
    current = path.join(current, `depth-${i}`)
    fs.mkdirSync(current)
    fs.writeFileSync(path.join(current, 'file.txt'), 'content')
  }
}

/**
 * Runs a benchmark group that compares multiple rmSync implementations and prints a Mitata-like comparison table.
 *
 * Executes a warmup run then performs 10 timed iterations per implementation using the provided setup function to create
 * each test directory (setup time is excluded from measurements). For each implementation it computes the average time
 * in milliseconds and prints each implementation's average alongside a ratio compared to the first (baseline).
 *
 * @param setupFn - A function that creates the test directory structure at the given path before removal
 */
async function runGroup(groupName: string, setupFn: (dir: string) => void) {
  console.log(`\n${groupName}`)

  const implementations = [
    { name: 'Node.js fs.rmSync', fn: (p: string) => fs.rmSync(p, { recursive: true, force: true }) },
    { name: 'hyper-fs rmSync', fn: (p: string) => hyperRmSync(p, { recursive: true, force: true }) },
    {
      name: 'hyper-fs rmSync (4 threads)',
      fn: (p: string) => hyperRmSync(p, { recursive: true, force: true, concurrency: 4 }),
    },
  ]

  const results: { name: string; time: number }[] = []

  for (const impl of implementations) {
    const times: number[] = []
    const iterations = 10

    // Warmup (1 run)
    const warmupDir = path.join(baseDir, `warmup-${impl.name.replace(/[^a-zA-Z0-9]/g, '')}`)
    setupFn(warmupDir)
    impl.fn(warmupDir)

    for (let i = 0; i < iterations; i++) {
      const testDir = path.join(baseDir, `${impl.name.replace(/[^a-zA-Z0-9]/g, '-')}-${i}`)
      setupFn(testDir) // Setup time NOT included

      const start = process.hrtime.bigint()
      impl.fn(testDir) // Measured time
      const end = process.hrtime.bigint()

      const ms = Number(end - start) / 1_000_000
      times.push(ms)
    }

    const avg = times.reduce((a, b) => a + b, 0) / times.length
    results.push({ name: impl.name, time: avg })
  }

  // Render Mitata-like output
  // Example:
  // Node.js fs.rmSync    10.50 ms  (baseline)
  // hyper-fs rmSync      12.00 ms  1.14x (slower)

  const baseline = results[0]

  results.forEach((res) => {
    const isBaseline = res === baseline
    const ratio = res.time / baseline.time
    const diffStr = isBaseline ? '(baseline)' : `${ratio.toFixed(2)}x ${ratio > 1 ? '(slower)' : '(faster)'}`

    console.log(`  ${res.name.padEnd(25)} ${res.time.toFixed(2)} ms  ${diffStr}`)
  })
}

/**
 * Execute the benchmark suite for the two test scenarios and remove temporary data.
 *
 * Runs the "Flat directory (2000 files)" and "Deep nested directory (depth 100)" benchmark groups sequentially, then deletes the temporary base directory if it exists.
 */
async function run() {
  await runGroup('Flat directory (2000 files)', (dir) => createFlatStructure(dir, 2000))
  await runGroup('Deep nested directory (depth 100)', (dir) => createDeepStructure(dir, 100))

  // Clean up
  if (fs.existsSync(baseDir)) {
    fs.rmSync(baseDir, { recursive: true, force: true })
  }
}

run()