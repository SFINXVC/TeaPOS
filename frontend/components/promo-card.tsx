import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import Image from "next/image"

export function PromoCard() {
  const promoImg = "/promo_bg.png" // Place your image in public/promo.jpg or change the path
  return (
    <Card className="mt-4 bg-primary text-primary-foreground overflow-hidden shadow-lg border-none relative h-56 md:h-72 lg:h-96">
      <Image
        src={promoImg}
        alt="Promo"
        className="absolute inset-0 w-full h-full object-cover z-0 select-none"
        draggable="false"
      />
      <div className="absolute inset-0 bg-gradient-to-r from-black/70 to-black/30 to-80% z-10" />
      <CardContent className="relative z-20 flex flex-col justify-center items-start h-full px-4 md:px-8 text-white">
        <div className="text-xs mb-1">
          <span className="bg-white/80 text-black px-2 py-0.5 rounded-full text-xs font-bold">Use code: CREATE6969</span>
          <span className="ml-2 text-white/80 text-xs">at checkout</span>
        </div>
        <div className="text-white mt-1 font-bold text-xl md:text-2xl lg:text-3xl mb-1">Get 50% Off<br />By Creating an Account!</div>
        <Button className="bg-white text-black hover:bg-white/90 mt-5 text-sm md:text-base">Create Account</Button>
      </CardContent>
    </Card>
  )
}
