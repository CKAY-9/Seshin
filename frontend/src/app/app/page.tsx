import HomeServer from "./server";

const AppHome = () => {
    return (
        <>
            <title>App - Seshin</title>
            {/* @ts-expect-error Async Server Component */}
            <HomeServer></HomeServer>
        </>
    );        
}

export default AppHome;
