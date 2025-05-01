import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Layout } from "@/components/layout";
import { Header } from "@/components/header";
import { SearchBar } from "@/components/search-bar";
import { PromoCarousel } from "@/components/promo-carousel";
import { CategoryList } from "@/components/category-list";
import { MenuItemCard, MenuItem } from "@/components/menu-item-card";
import { BottomNavigation } from "@/components/bottom-navigation";

// Sample Data (Keep this or fetch from API)
const categories = [
  { name: "Drinks", icon: "🥤" },
  { name: "Snacks", icon: "🥨" },
  { name: "Desserts", icon: "🍰" },
  { name: "Meals", icon: "🍔" },
  { name: "Healthy", icon: "🥗" }
];

const menuItems: MenuItem[] = [
  { 
    name: "Bubble Milk Tea", 
    description: "Classic Taiwanese tea drink with chewy tapioca pearls.", 
    rating: 4.5, 
    ratingCount: 1234, 
    price: "$4.99", 
    prepTime: 5,
    image: ""
  },
  { 
    name: "Taro Smoothie", 
    description: "Creamy and sweet smoothie made with real taro root.", 
    rating: 4.8, 
    ratingCount: 987, 
    price: "$5.49", 
    prepTime: 7,
    image: ""
  },
   { 
    name: "Matcha Latte", 
    description: "Earthy green tea latte, available hot or iced.", 
    rating: 4.6, 
    ratingCount: 850, 
    price: "$5.29", 
    prepTime: 6,
    image: ""
  },
  { 
    name: "Popcorn Chicken", 
    description: "Crispy bite-sized fried chicken with seasoning.", 
    rating: 4.7, 
    ratingCount: 1502, 
    price: "$6.99", 
    prepTime: 10,
    image: ""
  },
  { 
    name: "Mango Green Tea", 
    description: "Refreshing green tea infused with sweet mango flavor.", 
    rating: 4.4, 
    ratingCount: 765, 
    price: "$4.79", 
    prepTime: 5,
    image: ""
  },
  { 
    name: "Strawberry Slush", 
    description: "Icy slush drink bursting with fresh strawberry taste.", 
    rating: 4.9, 
    ratingCount: 1120, 
    price: "$5.99", 
    prepTime: 8,
    image: ""
  },
  // Add more items as needed
];

export default function Home() {
  return (
    <Layout>
      <Header>
        <SearchBar placeholder="Search for yummy foods 😋" />
      </Header>
      
      <main className="flex-1 px-4 sm:px-6 lg:px-8 pb-20 md:pb-8">
        <PromoCarousel />
        
        <CategoryList categories={categories} />
        
        {/* Popular Menu Items Section */}
        <div className="mt-6 md:mt-10">
          <div className="flex justify-between items-center mb-3">
            <h2 className="text-lg md:text-xl lg:text-2xl font-bold text-foreground">Popular Menu Items</h2>
            <Button variant="link" className="text-sm text-primary p-0">View Full Menu</Button>
          </div>
          
          <div className="grid grid-cols-2 gap-4 md:grid-cols-2 lg:grid-cols-3">
            {menuItems.map((item, index) => (
              <MenuItemCard key={index} item={item} />
            ))}
          </div>
        </div>
      </main>
      
      <BottomNavigation />
    </Layout>
  );
}
