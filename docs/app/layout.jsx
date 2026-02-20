import 'nextra-theme-docs/style.css'

export default function RootLayout({ children }) {
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <body className="antialiased">{children}</body>
    </html>
  )
}
