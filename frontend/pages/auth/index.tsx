import { GetServerSideProps } from "next"
import AuthLogin from "@/pages/auth/login"
import AuthRegister from "@/pages/auth/register"
import AuthForgot from "@/pages/auth/forgot"

type AuthProps = {
    type: "login" | "register" | "forgot"
}

export default function AuthIndex({ type }: AuthProps) {
    return (
        <>
            {type === "login" && <AuthLogin />}
            {type === "register" && <AuthRegister />}
            {type === "forgot" && <AuthForgot />}
        </>
    )
}

export const getServerSideProps: GetServerSideProps<AuthProps> = async (context) => {
    const type = context.query.type;

    if (type !== "login" && type !== "register" && type !== "forgot") {
        return {
            notFound: true
        }
    }

    return {
        props: {
            type: type as "login" | "register" | "forgot"
        }
    }
}