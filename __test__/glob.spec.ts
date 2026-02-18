import test from 'ava'
import { globSync, glob } from '../index.js'
import * as nodeFs from 'node:fs'
import { join } from 'path'
import { tmpdir } from 'node:os'

const CWD = process.cwd()

// 构造包含文件和子目录的临时目录，用于验证目录匹配行为
function makeDirFixture(): string {
  const base = join(tmpdir(), `hyper-glob-test-${Date.now()}-${Math.random().toString(36).slice(2)}`)
  nodeFs.mkdirSync(join(base, 'src/sub'), { recursive: true })
  nodeFs.writeFileSync(join(base, 'src/a.ts'), '')
  nodeFs.writeFileSync(join(base, 'src/b.ts'), '')
  nodeFs.writeFileSync(join(base, 'src/sub/c.ts'), '')
  nodeFs.mkdirSync(join(base, 'dist'), { recursive: true })
  nodeFs.writeFileSync(join(base, 'dist/out.js'), '')
  return base
}

test('globSync: should find files in current directory', (t) => {
  const files = globSync('*.json', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.endsWith('package.json')))
})

test('globSync: should match files in subdirectories', (t) => {
  const files = globSync('src/*.rs', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.endsWith('lib.rs')))
})

test('globSync: should return Dirent objects when withFileTypes is true', (t) => {
  const files = globSync('src/*.rs', { cwd: CWD, withFileTypes: true })
  t.true(files.length > 0)

  const first = files[0]
  if (typeof first === 'object') {
    t.is(typeof first.isFile, 'function')
    t.true(first.isFile())
    t.is(typeof first.name, 'string')
    t.true(first.name.endsWith('.rs'))
    t.is(typeof first.parentPath, 'string')
  } else {
    t.fail('Should return objects')
  }
})

test('globSync: should support exclude option', (t) => {
  // Should match multiple .rs files normally
  const allFiles = globSync('src/*.rs', { cwd: CWD })
  t.true(allFiles.some((f) => f.endsWith('lib.rs')))

  // Exclude lib.rs
  const filteredFiles = globSync('src/*.rs', { cwd: CWD, exclude: ['lib.rs'] })
  t.true(filteredFiles.length > 0)
  t.false(
    filteredFiles.some((f) => f.endsWith('lib.rs')),
    'Should exclude lib.rs',
  )
  t.true(filteredFiles.length < allFiles.length)
})

test('globSync: should respect gitIgnore (default: true)', (t) => {
  // When gitIgnore: true (default), files in .gitignore are excluded.
  // When gitIgnore: false, they are included.
  const ignoredFiles = globSync('target/**/*.d', { cwd: CWD })
  const includedFiles = globSync('target/**/*.d', { cwd: CWD, gitIgnore: false })

  if (includedFiles.length > 0) {
    t.true(ignoredFiles.length < includedFiles.length, 'Should find fewer files when respecting gitignore')
  } else {
    t.pass('Target directory empty or not present, skipping gitIgnore comparison')
  }
})

test('globSync: concurrency option should not crash', (t) => {
  const files = globSync('src/**/*.rs', { cwd: CWD, concurrency: 2 })
  t.true(files.length > 0)
})

test('async: should work basically', async (t) => {
  const files = await glob('*.json', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.endsWith('package.json')))
})

test('async: withFileTypes', async (t) => {
  const files = await glob('src/*.rs', { cwd: CWD, withFileTypes: true })
  t.true(files.length > 0)
  const first = files[0]
  t.is(typeof first, 'object')
  t.true(first.isFile())
})

test('async: should return empty array for no matches', async (t) => {
  const files = await glob('non_existent_*.xyz', { cwd: CWD })
  t.true(Array.isArray(files))
  t.is(files.length, 0)
})

test('async: recursive match', async (t) => {
  const files = await glob('**/*.rs', { cwd: CWD })
  t.true(files.length > 0)
  t.true(files.some((f) => f.includes('src/lib.rs')))
})

// ===== 目录匹配行为（对齐 Node.js fs.globSync）=====

test('globSync: "src/*" should include subdirectories matching the pattern', (t) => {
  const base = makeDirFixture()
  // Node.js: fs.globSync('src/*') 返回 src/ 下的文件 AND 子目录
  const results = globSync('src/*', { cwd: base })
  const names = results.map((r) => r.replace(/\\/g, '/'))
  t.true(names.includes('src/a.ts'), 'should include files')
  t.true(names.includes('src/b.ts'), 'should include files')
  t.true(names.includes('src/sub'), 'should include directories matching the pattern')
  // 不应包含 dist/ 下的内容（不匹配 src/*）
  t.false(names.some((n) => n.startsWith('dist')))
})

test('globSync: "**/*.ts" should NOT include directories (dirs lack .ts extension)', (t) => {
  const base = makeDirFixture()
  const results = globSync('**/*.ts', { cwd: base })
  const names = results.map((r) => r.replace(/\\/g, '/'))
  // 所有结果应以 .ts 结尾（目录不应被包含）
  t.true(names.length > 0)
  t.true(
    names.every((n) => n.endsWith('.ts')),
    `non-.ts entry found: ${names.join(', ')}`,
  )
})

test('globSync: "**" should include both files and directories recursively', (t) => {
  const base = makeDirFixture()
  const results = globSync('**', { cwd: base })
  const names = results.map((r) => r.replace(/\\/g, '/'))
  // 应包含目录 src、src/sub、dist
  t.true(names.includes('src'), 'should include top-level directories')
  t.true(names.includes('src/sub'), 'should include nested directories')
  // 也应包含文件
  t.true(names.some((n) => n.endsWith('.ts')))
})

test('globSync: dir-matching result should have isDirectory()=true with withFileTypes', (t) => {
  const base = makeDirFixture()
  const results = globSync('src/*', { cwd: base, withFileTypes: true })
  t.true(results.length > 0)
  const subDir = results.find((r) => typeof r === 'object' && r.name === 'sub')
  t.truthy(subDir, 'should include sub directory as Dirent')
  if (subDir && typeof subDir === 'object') {
    t.true(subDir.isDirectory())
    t.false(subDir.isFile())
  }
})

test('dual-run: globSync "src/*" should match node:fs.globSync behavior for directories', (t) => {
  const base = makeDirFixture()
  // node:fs.globSync 自 v22.0.0 起稳定，对齐其目录匹配行为
  const nodeResults: string[] = []
  try {
    // @ts-ignore - globSync 在旧版 Node 可能不存在
    const nodeGlob = nodeFs.globSync as ((p: string, o: object) => string[]) | undefined
    if (typeof nodeGlob === 'function') {
      nodeResults.push(...nodeGlob('src/*', { cwd: base }))
    }
  } catch {
    // 旧版 Node.js 不支持 fs.globSync，跳过对比
    t.pass('node:fs.globSync not available, skipping dual-run comparison')
    return
  }
  const hyperResults = globSync('src/*', { cwd: base }).map((r) => r.replace(/\\/g, '/'))
  const nodeNorm = nodeResults.map((r) => r.replace(/\\/g, '/'))
  // 检查 node.js 返回的目录条目我们也有
  const nodeDirs = nodeNorm.filter((n) => {
    try {
      return nodeFs.statSync(join(base, n)).isDirectory()
    } catch {
      return false
    }
  })
  for (const d of nodeDirs) {
    t.true(hyperResults.includes(d), `should include directory '${d}' as node:fs does`)
  }
})
