import { Bell } from "lucide-react"
import { ThemeToggle } from "@/components/theme-toggle"
import { ReactNode } from "react"

interface HeaderProps {
  children?: ReactNode
}

import { useEffect, useRef, useState } from "react"

export function Header({ children }: HeaderProps) {
  const [show, setShow] = useState(true)
  const lastScrollY = useRef(0)
  const [headerHeight, setHeaderHeight] = useState(0)
  const navRef = useRef<HTMLDivElement>(null)
  const searchRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY
      if (currentScrollY > lastScrollY.current) {
        setShow(false)
      } else {
        setShow(true)
      }
      lastScrollY.current = currentScrollY
    }

    const updateHeaderHeight = () => {
      const navHeight = navRef.current?.offsetHeight || 0
      const searchHeight = searchRef.current?.offsetHeight || 0
      setHeaderHeight(navHeight + searchHeight)
    }

    window.addEventListener("scroll", handleScroll)
    window.addEventListener("resize", updateHeaderHeight)
    
    // Initial calculation
    updateHeaderHeight()
    
    return () => {
      window.removeEventListener("scroll", handleScroll)
      window.removeEventListener("resize", updateHeaderHeight)
    }
  }, [])

  return (
    <>
      <div className="header-wrapper" style={{ height: headerHeight }}></div>
      
      <div className="header-container">
        <div 
          ref={navRef}
          style={{transform: show ? 'translateY(0)' : 'translateY(-100%)', transition: 'transform 0.3s'}}
          className="fixed top-0 left-0 right-0 z-50 w-full bg-white/70 dark:bg-black/40 backdrop-blur-md border-b border-transparent shadow-none"
        >
          <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-2 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="font-extrabold text-xl md:text-2xl tracking-tight text-primary drop-shadow-sm">TeaPOS</span>
            </div>
            <div className="flex items-center gap-2">
              <ThemeToggle />
              <button className="inline-flex items-center justify-center rounded-full ml-1 focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/60 transition-all w-9 h-9">
                <img src="/profile.jpg" alt="Profile" className="w-9 h-9 rounded-full object-cover border-2 border-white shadow-sm" />
              </button>
            </div>
          </div>
        </div>
        
        <div 
          ref={searchRef}
          style={{top: show ? '48px' : '0', transition: 'top 0.3s'}}
          className="fixed left-0 right-0 z-40 w-full bg-white/70 dark:bg-black/40 backdrop-blur-md border-b border-transparent shadow-none"
          data-component-name="Header"
        >
          <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-2" data-component-name="Header">{children}</div>
        </div>
      </div>
    </>
  )
}
