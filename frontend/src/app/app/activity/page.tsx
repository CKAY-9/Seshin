import ClientActivity from "./client";
import ServerActivity from "./server";

const ActivityPage = () => {
    return (
        <>
            <ClientActivity>
                {/* @ts-expect-error Async Server Component */}
                <ServerActivity></ServerActivity>
            </ClientActivity> 
        </>  
    );
}

export default ActivityPage;
