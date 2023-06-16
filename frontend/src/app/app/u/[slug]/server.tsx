import {fetchPersonalData, fetchPublicProfile} from "@/api/user/fetching";
import Header from "@/components/header/header";
import Image from "next/image";
import {FollowersButton} from "./client";

const UserServer = async (props: { params: { slug: string } }) => {
    let personalData = await fetchPersonalData();
    let userData;
    if (props.params.slug === "me") {
        userData = personalData;
    } else {
        userData = await fetchPublicProfile("", props.params.slug);
    }

    if (userData === undefined) {
        return (
            <>
                <main className="container">
                    <h1>Couldn't find user "{props.params.slug}"</h1>
                </main>
            </>
        );     
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
                <h3 style={{"margin": "2rem 0 0 0", "opacity": "0.5", "fontSize": "1rem"}}>Username: @{userData?.username}</h3>
                <h3 style={{"margin": "0 0 2rem 0", "opacity": "0.5", "fontSize": "1rem"}}>Public ID: {userData?.public_id}</h3>
                <FollowersButton followers={userData?.followers}></FollowersButton>    
            </main>
        </>
    );        
}

export default UserServer;
