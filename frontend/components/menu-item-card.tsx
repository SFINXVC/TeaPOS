import { Card, CardContent } from "@/components/ui/card"

export interface MenuItem {
  name: string
  description: string
  rating: number
  ratingCount: number
  price: string
  prepTime: number
  image: string
}

export function MenuItemCard({ item }: { item: MenuItem }) {
  return (
    <Card className="overflow-hidden h-full transition-transform hover:scale-[1.02] cursor-pointer border border-border bg-card text-card-foreground p-0">
      <div className="relative">
        {/* Placeholder for image */}
        <div className="h-40 md:h-48 bg-muted"></div> 
        <div className="absolute bottom-2 left-2 bg-black bg-opacity-70 text-white px-2 py-1 rounded-md text-xs flex items-center">
          <span className="mr-1">⏱️</span>
          <span>{item.prepTime} min</span>
        </div>
      </div>
      <CardContent className="p-3 pt-3 relative h-28 flex flex-col justify-between">
        <div className="flex justify-between">
          <div>
            <h3 className="font-bold text-card-foreground">{item.name}</h3>
            <div className="text-xs text-muted-foreground mt-1 line-clamp-2">
              {item.description}
            </div>
          </div>
          <div className="flex items-center bg-green-50 dark:bg-green-900/30 px-2 py-1 rounded h-fit">
            <span className="text-green-600 dark:text-green-400 font-bold text-sm">{item.rating}</span>
            <span className="text-xs text-muted-foreground ml-1">({item.ratingCount})</span>
          </div>
        </div>
        <span className="absolute right-3 bottom-3 font-bold text-card-foreground text-sm">{item.price}</span>
      </CardContent>
    </Card>
  )
}
