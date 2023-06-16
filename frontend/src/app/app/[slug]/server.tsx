import {fetchPersonalData, fetchPublicProfile} from "@/api/user/fetching";
import Header from "@/components/header/header";
import Image from "next/image";

const UserServer = async (props: { params: { slug: string } }) => {
    let personalData = await fetchPersonalData();
    let userData;
    if (props.params.slug === "me") {
        userData = personalData;
    } else {
        userData = await fetchPublicProfile("", props.params.slug);
    }
    
    return (
        <>
            <title>{userData?.display_name || "User"} - Seshin</title>
            
            <Header user={personalData}></Header>

            <main className="container">
                <div style={{"position": "relative"}}>
                    <Image src={userData?.avatar_url || ""} alt="User Icon" width={0} height={0} sizes="100%" style={{
                        "width": "15rem",
                        "height": "15rem",
                        "borderRadius": "50%"
                    }}></Image>
                </div>
                <section style={{"display": "flex", "gap": "1rem", "alignItems": "center", "position": "relative"}}>
                    <h1 style={{"margin": "0"}}>{userData?.display_name}</h1>
                    <Image 
                        src={userData?.oauth_type === "discord" ? "/icon_clyde_white.png" : "/github_mark_white.png"} 
                        alt="OAuth Type"
                        width={0}
                        height={0}
                        sizes="100%"
                        style={{"width": "auto", "height": "2rem", "filter": "invert(1)", "opacity": "0.5"}}
                    />
                </section>
                <h3 style={{"margin": "2rem 0 0 0", "opacity": "0.5"}}>Username: @{userData?.username}</h3>
                <h3 style={{"margin": "0", "opacity": "0.5"}}>Public ID: {userData?.public_id}</h3>
            </main>
        </>
    );        
}

export default UserServer;
