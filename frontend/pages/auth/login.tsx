import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { useState } from "react"
import Link from "next/link"

const AuthLogin = () => {
    const [email, setEmail] = useState("")
    const [password, setPassword] = useState("")
    const [loading, setLoading] = useState(false)

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()
        setLoading(true)
        // handle login logic here
        setTimeout(() => setLoading(false), 1200)
    }

    return (
        <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary/10 to-background px-4">
            <Card className="w-full max-w-md">
                <CardHeader>
                    <CardTitle className="text-2xl mb-2">Sign in to TeaPOS</CardTitle>
                </CardHeader>
                <CardContent>
                    <form className="space-y-5" onSubmit={handleSubmit}>
                        <div className="space-y-2">
                            <Label htmlFor="email">Email</Label>
                            <Input
                                id="email"
                                type="email"
                                autoComplete="email"
                                required
                                value={email}
                                onChange={e => setEmail(e.target.value)}
                                placeholder="you@email.com"
                            />
                        </div>
                        <div className="space-y-2">
                            <Label htmlFor="password">Password</Label>
                            <Input
                                id="password"
                                type="password"
                                autoComplete="current-password"
                                required
                                value={password}
                                onChange={e => setPassword(e.target.value)}
                                placeholder="••••••••"
                            />
                        </div>
                        <div className="flex items-center justify-between pt-2">
                            <Link href="/auth?type=forgot" className="text-sm text-primary hover:underline">Forgot password?</Link>
                        </div>
                        <Button type="submit" className="w-full mt-2" disabled={loading}>
                            {loading ? "Signing in..." : "Sign In"}
                        </Button>
                    </form>
                </CardContent>
                <CardFooter className="flex flex-col gap-2">
                    <span className="text-sm text-muted-foreground">Don&apos;t have an account? <Link href="/auth?type=register" className="text-primary hover:underline">Sign up</Link></span>
                </CardFooter>
            </Card>
        </div>
    )
}

export default AuthLogin;