import { Bench } from 'tinybench'
import * as fs from 'node:fs'
import { readdirSync } from '../index.js'
import * as path from 'node:path'

const bench = new Bench({ time: 1000 })
const targetDir = path.resolve(process.cwd(), 'node_modules')
const dir = fs.existsSync(targetDir) ? targetDir : process.cwd()

console.log(`Benchmarking readdir on: ${dir}`)

// Flat
bench
  .add('Node.js fs.readdirSync (flat)', () => {
    fs.readdirSync(dir, { withFileTypes: true })
  })
  .add('hyper-fs readdirSync (flat)', () => {
    readdirSync(dir, { recursive: false })
  })

// Recursive
bench
  .add('Node.js fs.readdirSync (recursive)', () => {
    try {
      // @ts-ignore
      fs.readdirSync(dir, { recursive: true, withFileTypes: true })
    } catch (e) {
      // Fallback for older node versions or if not supported
      fs.readdirSync(dir, { withFileTypes: true })
    }
  })
  .add('hyper-fs readdirSync (recursive)', () => {
    readdirSync(dir, { recursive: true })
  })
  .add('hyper-fs readdirSync (recursive, 4 threads)', () => {
    readdirSync(dir, { recursive: true, concurrency: 4 })
  })

await bench.run()

console.table(bench.table())
