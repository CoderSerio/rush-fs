import { Bench } from 'tinybench'
import * as fs from 'node:fs'
import { readdirSync } from '../index.js'
import * as path from 'node:path'

const bench = new Bench({ time: 1000 })
const targetDir = path.resolve(process.cwd(), 'node_modules')
const dir = fs.existsSync(targetDir) ? targetDir : process.cwd()

console.log(`Benchmarking readdir on: ${dir}`)

bench
  .add('Node.js fs.readdirSync', () => {
    fs.readdirSync(dir, { withFileTypes: true })
  })
  .add('Node.js fs.readdirSync (recursive)', () => {
    fs.readdirSync(dir, { recursive: true, withFileTypes: true })
  })
  .add('hyper-fs readdirSync (default)', () => {
    readdirSync(dir)
  })
  .add('hyper-fs readdirSync (4 threads)', () => {
    readdirSync(dir, { concurrency: 4 })
  })

await bench.run()

console.table(bench.table())
