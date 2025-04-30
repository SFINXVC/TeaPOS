import { useState, useEffect } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Search, Mic, Bell } from "lucide-react";

export default function Home() {
  const [searchQuery, setSearchQuery] = useState("");
  const [isMobile, setIsMobile] = useState(true);

  useEffect(() => {
    const handleResize = () => {
      setIsMobile(window.innerWidth < 768);
    };
    
    // Set initial value
    handleResize();
    
    // Add event listener
    window.addEventListener('resize', handleResize);
    
    // Cleanup
    return () => window.removeEventListener('resize', handleResize);
  }, []);
  
  const categories = [
    { name: "Category 1", icon: "🥹" },
    { name: "Category 2", icon: "🥹" },
    { name: "Category 3", icon: "🥹" },
    { name: "Category 4", icon: "🥹" },
    { name: "Category 5", icon: "🥹" }
  ];
  
  const menuItems = [
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
    { 
      name: "Menu Name", 
      description: "Menu Description should be here or lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor uwuhhh awgghhhhh ", 
      rating: 6.9, 
      ratingCount: 6969, 
      price: "$69.69", 
      prepTime: 69,
      image: ""
    },
  ];
  
  return (
    <div className="flex flex-col min-h-screen bg-white max-w-6xl mx-auto">
      {/* Header */}
      <header className="px-4 sm:px-6 lg:px-8 py-3 bg-white sticky top-0 z-10 w-full">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center">
            <span className="font-bold text-lg text-orange-500">TeaPOS</span>
            <span className="text-xs bg-green-100 text-green-700 px-2 py-0.5 rounded-full ml-2">Open</span>
          </div>
          <div>
            <Bell size={20} />
          </div>
        </div>
        
        {/* Search Bar */}
        <div className="relative mt-1">
          <div className="relative">
            <Input
              type="text"
              placeholder="Search for yummy foods 😋"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10 pr-10 py-2.5 rounded-full bg-gray-100 border-none focus-visible:ring-orange-500 focus-visible:ring-opacity-50"
            />
            <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400">
              <Search size={18} strokeWidth={2.5} />
            </div>
            <div className="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400">
              <Mic size={18} strokeWidth={2.5} />
            </div>
          </div>
        </div>
      </header>
      
      <main className="flex-1 px-4 sm:px-6 lg:px-8 pb-16 md:pb-8 bg-white">
        {/* Promo Card */}
        <Card className="mt-4 bg-orange-500 text-white overflow-hidden shadow-lg">
          <CardContent className="p-4 md:p-6 flex">
            <div className="flex-1">
              <div className="text-xs mb-1">
                <span className="bg-white text-orange-500 px-2 py-0.5 rounded-full text-xs font-bold">Use code: CREATE6969</span>
                <span className="ml-2 text-white text-xs">at checkout</span>
              </div>
              <div className="text-white mt-1 font-bold text-xl md:text-2xl lg:text-3xl mb-1">Get 50% Off<br />By Creating an Account!</div>
              <Button className="bg-white text-orange-500 hover:bg-gray-100 mt-5 text-sm md:text-base">Create Account</Button>
            </div>
            <div className="flex items-center">
              <div className="h-20 w-20 relative">
                <div className="absolute right-0 bottom-0">
                  <span className="text-3xl">🍟</span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
        
        {/* Categories */}
        <div className="mt-6 overflow-x-auto pb-2">
          <div className="flex space-x-4 md:space-x-6 lg:space-x-8 md:justify-center">
            {categories.map((category, index) => (
              <div key={index} className="flex flex-col items-center">
                <div className="w-16 h-16 md:w-20 md:h-20 rounded-full bg-orange-100 text-orange-500 flex items-center justify-center mb-1 transition-transform hover:scale-105 cursor-pointer shadow-sm">
                  <span className="text-2xl">{category.icon}</span>
                </div>
                <span className="text-xs md:text-sm font-medium">{category.name}</span>
              </div>
            ))}
          </div>
        </div>
        
        {/* Popular Menu Items */}
        <div className="mt-6 md:mt-10">
          <div className="flex justify-between items-center mb-3">
            <h2 className="text-lg md:text-xl lg:text-2xl font-bold">Popular Menu Items</h2>
            <Button variant="link" className="text-sm text-orange-500 p-0">View Full Menu</Button>
          </div>
          
          <div className="space-y-4 md:grid md:grid-cols-2 lg:grid-cols-3 md:gap-4 md:space-y-0">
            {menuItems.map((item, index) => (
              <Card key={index} className="overflow-hidden h-full transition-transform hover:scale-[1.02] cursor-pointer">
                <div className="relative">
                  <div className="h-40 md:h-48 bg-gray-100"></div>
                  <div className="absolute bottom-2 left-2 bg-black bg-opacity-70 text-white px-2 py-1 rounded-md text-xs flex items-center">
                    <span className="mr-1">⏱️</span>
                    <span>{item.prepTime} min</span>
                  </div>
                </div>
                <CardContent className="p-3">
                  <div className="flex justify-between">
                    <div>
                      <h3 className="font-bold">{item.name}</h3>
                      <div className="text-xs text-gray-500 mt-1 line-clamp-2">
                        {item.description}
                      </div>
                    </div>
                    <div className="flex items-center bg-green-50 px-2 py-1 rounded h-fit">
                      <span className="text-green-500 font-bold text-sm">{item.rating}</span>
                      <span className="text-xs text-gray-500 ml-1">({item.ratingCount})</span>
                    </div>
                  </div>
                  <div className="mt-2 text-sm">
                    <span className="float-right font-bold">{item.price}</span>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </main>
      
      {/* Bottom Navigation */}
      {isMobile ? (
        <nav className="fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 px-4 py-2 z-10">
        <div className="flex justify-between items-center">
          <div className="flex flex-col items-center">
            <Button variant="ghost" className="h-10 w-10 p-0 rounded-full">
              <Search className="text-orange-500" size={22} />
            </Button>
            <span className="text-xs">Explore</span>
          </div>
          <div className="flex flex-col items-center">
            <Button variant="ghost" className="h-10 w-10 p-0 rounded-full">
              <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-gray-400">
                <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
              </svg>
            </Button>
            <span className="text-xs text-gray-400">Favorites</span>
          </div>
          <div className="flex flex-col items-center">
            <Button variant="ghost" className="h-10 w-10 p-0 rounded-full">
              <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-gray-400">
                <circle cx="8" cy="21" r="1" />
                <circle cx="19" cy="21" r="1" />
                <path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" />
              </svg>
            </Button>
            <span className="text-xs text-gray-400">Orders</span>
          </div>
          <div className="flex flex-col items-center">
            <Button variant="ghost" className="h-10 w-10 p-0 rounded-full">
              <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-gray-400">
                <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" />
                <circle cx="12" cy="7" r="4" />
              </svg>
            </Button>
            <span className="text-xs text-gray-400">Account</span>
          </div>
        </div>
      </nav>
      ) : (
        <div className="hidden md:block fixed bottom-4 right-4 z-10">
          <Button className="h-14 w-14 rounded-full bg-orange-500 hover:bg-orange-600 shadow-lg flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" className="text-white">
              <circle cx="8" cy="21" r="1" />
              <circle cx="19" cy="21" r="1" />
              <path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" />
            </svg>
          </Button>
        </div>
      )}
    </div>
  );
}
