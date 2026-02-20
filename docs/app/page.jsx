import Link from 'next/link'

export const metadata = {
  title: 'Rush-FS',
  description:
    'API-aligned with Node.js fs for painless drop-in replacement. Get multi-fold performance in heavy file operations, powered by Rust.',
}

const styles = {
  wrapper: {
    minHeight: '100vh',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    padding: '1.5rem',
    textAlign: 'center',
  },
  title: { fontSize: '2.5rem', fontWeight: 700, marginBottom: '1rem', letterSpacing: '-0.025em' },
  lead: { fontSize: '1.25rem', color: '#666', maxWidth: '36rem', marginBottom: '2rem' },
  code: { padding: '0.125rem 0.5rem', borderRadius: '0.25rem', background: '#e5e5e5', fontSize: '0.9em' },
  buttons: { display: 'flex', flexWrap: 'wrap', gap: '1rem', justifyContent: 'center' },
  primary: {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: '0.5rem',
    background: '#f97316',
    color: '#fff',
    fontWeight: 500,
    padding: '0.75rem 1.5rem',
    textDecoration: 'none',
  },
  secondary: {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: '0.5rem',
    border: '1px solid #d4d4d4',
    fontWeight: 500,
    padding: '0.75rem 1.5rem',
    textDecoration: 'none',
    color: 'inherit',
  },
  tagline: { marginTop: '3rem', fontSize: '0.875rem', color: '#737373' },
}

export default function LandingPage() {
  return (
    <div style={styles.wrapper}>
      <h1 style={styles.title}>Rush-FS</h1>
      <p style={styles.lead}>
        API-aligned with Node.js <code style={styles.code}>fs</code> for painless drop-in replacement. Get multi-fold
        performance in heavy file operations, powered by Rust.
      </p>
      <div style={styles.buttons}>
        <Link href="/docs" style={styles.primary}>
          Read docs →
        </Link>
        <a href="https://github.com/CoderSerio/rush-fs" target="_blank" rel="noopener noreferrer" style={styles.secondary}>
          GitHub
        </a>
        <a href="https://www.npmjs.com/package/rush-fs" target="_blank" rel="noopener noreferrer" style={styles.secondary}>
          npm
        </a>
      </div>
      <p style={styles.tagline}>
        Drop-in replacement · Recursive readdir, glob, rm, cp 2–70× faster · Prebuilt binaries
      </p>
    </div>
  )
}
