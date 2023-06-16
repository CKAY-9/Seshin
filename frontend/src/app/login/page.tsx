import Image from "next/image";
import Link from "next/link";
import ClientLogin from "./client";

const Login = () => {
    return (
        <>
            <title>Login - Seshin</title>
            <main className="container" style={{
                "textAlign": "center",
                "position": "absolute",
                "top": 0,
                "left": 0,
                "width": "100vw",
                "height": "100vh",
                "margin": 0,
                "display": "grid",
                "placeContent": "center",
            }}>
                <div style={{
                    "display": "flex",
                    "flexDirection": "column",
                    "gap": "2rem",
                    "padding": "3rem",
                    "borderRadius": "3rem",
                    "border": "1px solid rgb(var(--bText), 0.5)"
                }}>
                    <h1>Seshin Login</h1>
                    <Link href={process.env.NEXT_PUBLIC_DISCORD_OAUTH || ""} style={{
                        "backgroundColor": "#5865F2",
                        "padding": "1rem 3rem",
                        "borderRadius": "10px",
                        "color": "rgb(var(--cText))",
                        "display": "flex",
                        "alignItems": "center",
                        "justifyContent": "center",
                        "gap": "1rem",
                        "boxShadow": "var(--shdw1)"
                    }}>
                        <div style={{"position": "relative"}}>
                            <Image src="/icon_clyde_white.png" alt="Discord Clyde" sizes="100%" width={0} height={0} style={{
                                "width": "auto",
                                "height": "1.5rem"
                            }}></Image>
                        </div>
                        <span>Login using Discord</span>
                    </Link>
                    <Link href={process.env.NEXT_PUBLIC_GITHUB_OAUTH || ""} style={{
                        "backgroundColor": "#333",
                        "padding": "1rem 3rem",
                        "borderRadius": "10px",
                        "color": "rgb(var(--cText))",
                        "display": "flex",
                        "alignItems": "center",
                        "justifyContent": "center",
                        "gap": "1rem",
                        "boxShadow": "var(--shdw1)"
                    }}>
                        <div style={{"position": "relative"}}>
                            <Image src="/github_mark_white.png" alt="Github Mark" sizes="100%" width={0} height={0} style={{
                                "width": "auto",
                                "height": "1.5rem"
                            }}></Image>
                        </div>
                        <span>Login using Github</span>
                    </Link>
                    <Link href="/" className="casual">Back</Link>
                </div>
            </main> 
            
            <ClientLogin></ClientLogin>
        </>
    ); 
}

export default Login;
