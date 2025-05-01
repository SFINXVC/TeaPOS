import { ReactNode } from "react"

interface LayoutProps {
  children: ReactNode
}

export function Layout({ children }: LayoutProps) {
  return (
    <div className="flex flex-col min-h-screen bg-background max-w-6xl mx-auto overflow-x-hidden overflow-y-auto w-full">
      {children}
    </div>
  )
}
