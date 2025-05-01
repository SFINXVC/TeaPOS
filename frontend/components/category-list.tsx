interface Category {
  name: string
  icon: string
}

interface CategoryListProps {
  categories: Category[]
}

export function CategoryList({ categories }: CategoryListProps) {
  return (
    <div className="mt-8 mb-6">
      <h2 className="text-lg md:text-xl font-bold text-foreground mb-4">Categories</h2>
      <div className="grid grid-cols-5 gap-2 md:gap-4" data-component-name="CategoryList">
        {categories.map((category, index) => (
          <div key={index} className="group flex flex-col items-center">
            <div className="w-full aspect-[4/3] rounded-xl bg-gradient-to-br from-primary/20 to-primary/5 border border-primary/10 flex items-center justify-center mb-2 transition-all hover:shadow-md hover:shadow-primary/10 hover:-translate-y-1 cursor-pointer">
              <span className="text-2xl md:text-3xl group-hover:scale-110 transition-transform">{category.icon}</span>
            </div>
            <span className="text-xs md:text-sm font-medium text-foreground text-center">{category.name}</span>
          </div>
        ))}
      </div>
    </div>
  )
}
