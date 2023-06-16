import {fetchPersonalData} from "@/api/user/fetching";
import Link from "next/link";

const Home = async () => {
    const user = await fetchPersonalData();

    return (
        <>
            <main className="container" style={{"textAlign": "center"}}>
                <h1 style={{"fontSize": "4rem"}}>Seshin</h1>  
                <h2 style={{"opacity": "0.75"}}>Plan events with your chums</h2>
            
                {user === undefined ? <Link href="/login">Login</Link> : <Link href="/app">Enter Seshin</Link>}
            </main>
        </>
    );
}

export default Home;
