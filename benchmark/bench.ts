import * as fs from 'node:fs'
import * as path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

/**
 * Discovers and runs benchmark files in the current directory, optionally filtered by the first command-line argument.
 *
 * Selects files ending with `.ts` (excluding `bench.ts` and declaration files ending with `.d.ts`). If a filter string is provided as the first CLI argument, only files whose names include that substring (case-insensitive) are selected. Logs a message and exits if no files match, logs the number of files found, and imports each matched file to execute its benchmark; errors importing individual files are logged.
 */
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