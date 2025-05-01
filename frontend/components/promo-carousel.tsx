import {
  Carousel,
  CarouselContent,
  CarouselItem,
} from "@/components/ui/carousel"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { useState } from "react"

const promos = [
  {
    img: "/promo_bg.jpg",
    code: "CREATE6969",
    message: "Get <span class=\"text-yellow-300\">50% Off</span><br/>By Creating an Account!",
    cta: "Create Account"
  },
  {
    img: "/promo_bg.jpg",
    code: "TEALOVER",
    message: "Free <span class=\"text-green-300\">Drink</span><br/>On Your First Order!",
    cta: "Order Now"
  }
]

export function PromoCarousel() {
  const [current, setCurrent] = useState(0)
  const [api, setApi] = useState<any>(null)

  function handleApi(newApi: any) {
    setApi(newApi)
    if (newApi) {
      setCurrent(newApi.selectedScrollSnap())
      newApi.on("select", () => setCurrent(newApi.selectedScrollSnap()))
    }
  }

  return (
    <Carousel className="w-full" setApi={handleApi}>
      <CarouselContent>
        {promos.map((promo, idx) => (
          <CarouselItem key={idx}>
            <Card className="mt-4 bg-primary text-primary-foreground overflow-hidden shadow-lg border-none relative h-56 md:h-72 lg:h-96">
              <img
                src={promo.img}
                alt="Promo"
                className="absolute inset-0 w-full h-full object-cover z-0 select-none"
                draggable="false"
              />
              <div className="absolute inset-0 bg-gradient-to-r from-black/70 to-black/30 to-80% z-10" />
              <CardContent className="relative z-20 flex flex-col justify-center items-start h-full px-4 md:px-8 text-white">
                <div className="text-xs mb-1">
                  <span className="bg-white/80 text-black px-2 py-0.5 rounded-full text-xs font-bold">Use code: {promo.code}</span>
                  <span className="ml-2 text-white/80 text-xs">at checkout</span>
                </div>
                <div 
                  className="text-white mt-1 font-bold text-xl md:text-2xl lg:text-3xl mb-1" 
                  dangerouslySetInnerHTML={{ __html: promo.message }}
                />
                <Button className="bg-white text-black hover:bg-white/90 mt-5 text-sm md:text-base">{promo.cta}</Button>
              </CardContent>
            </Card>
          </CarouselItem>
        ))}
      </CarouselContent>
      
      <div className="flex justify-center items-center gap-2 mt-3">
        {promos.map((_, idx) => (
          <div
            key={idx}
            className={`h-1.5 w-8 rounded-full transition-all duration-300 ${current === idx ? 'bg-white' : 'bg-white/40'}`}
          />
        ))}
      </div>
    </Carousel>
  )
}
