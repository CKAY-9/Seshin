import "./globals.scss"

export const metadata = {
  title: "Seshin",
  description: "Plan events with your chums!",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
