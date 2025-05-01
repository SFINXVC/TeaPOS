import { useState } from "react"
import { Input } from "@/components/ui/input"
import { Search, Mic } from "lucide-react"

interface SearchBarProps {
  placeholder?: string
}

export function SearchBar({ placeholder = "Search..." }: SearchBarProps) {
  const [searchQuery, setSearchQuery] = useState("")

  return (
    <div className="relative mt-1">
      <div className="relative">
        <Input
          type="text"
          placeholder={placeholder}
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="pl-10 pr-10 py-2.5 rounded-full border-none focus:ring-1 focus:ring-primary/40 focus:shadow-[0_0_8px_2px_var(--tw-shadow-color)] focus:shadow-primary/30"
        />
        <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground">
          <Search size={18} strokeWidth={2.5} />
        </div>
        <div className="absolute right-3 top-1/2 transform -translate-y-1/2 text-muted-foreground">
          <Mic size={18} strokeWidth={2.5} />
        </div>
      </div>
    </div>
  )
}
