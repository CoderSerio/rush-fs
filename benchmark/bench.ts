import * as fs from 'node:fs'
import * as path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

async function runBenchmarks() {
  const args = process.argv.slice(2)
  const filter = args[0]

  const files = fs.readdirSync(__dirname).filter((file) => {
    const isBenchFile = file.endsWith('.ts') && file !== 'bench.ts' && !file.endsWith('.d.ts')
    if (!isBenchFile) return false

    if (filter) {
      return file.toLowerCase().includes(filter.toLowerCase())
    }
    return true
  })

  if (files.length === 0) {
    console.log(`No benchmark files found matching filter "${filter}"`)
    return
  }

  console.log(`Found ${files.length} benchmark files to run...`)

  for (const file of files) {
    console.log(`\n========================================`)
    console.log(`Running benchmark: ${file}`)
    console.log(`========================================`)

    try {
      await import(path.join(__dirname, file))
    } catch (e) {
      console.error(`Error running ${file}:`, e)
    }
  }
}

runBenchmarks()
