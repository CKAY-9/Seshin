import Footer from "@/components/footer/footer";
import HomeServer from "./server";

const AppHome = () => {
    return (
        <>
            <title>App - Seshin</title>
            {/* @ts-expect-error Async Server Component */}
            <HomeServer></HomeServer>
            <Footer></Footer>
        </>
    );        
}

export default AppHome;
