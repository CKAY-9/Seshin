import {fetchPersonalData} from "@/api/user/fetching";
import Header from "@/components/header/header";

const HomeServer = async () => {
    const personalData = await fetchPersonalData();    

    return (
        <>
            <Header user={personalData}></Header> 
        
            <main className="container">
                <h1>Seshin</h1>
            </main>
        </>
    );
}
     
export default HomeServer;
