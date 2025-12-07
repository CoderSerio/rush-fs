import * as fs from 'fs'
import * as path from 'path'
import nativeBinding from '../../index.js'
import type { ReaddirOptions } from '../../index.d.ts'

export function readdirSync(dirPath: string, options: ReaddirOptions = {}) {
  if (!options.recursive) {
    return fs.readdirSync(dirPath, { withFileTypes: true, ...options }).map((entry) => ({
      name: entry.name,
      path: path.join(dirPath, entry.name),
      is_dir: entry.isDirectory(),
    }))
  }

  return nativeBinding.readdirSync(dirPath, {
    skipHidden: options.skipHidden,
    concurrency: options.concurrency,
  })
}

export async function readdir(dirPath: string, options: ReaddirOptions = {}) {
  if (!options.recursive) {
    const files = await fs.promises.readdir(dirPath, { withFileTypes: true, ...options })
    return files.map((entry) => ({
      name: entry.name,
      path: path.join(dirPath, entry.name),
      is_dir: entry.isDirectory(),
    }))
  }

  return nativeBinding.readdir(dirPath, options)
}
