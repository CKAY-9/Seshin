import {fetchPersonalData} from "@/api/user/fetching";
import Header from "@/components/header/header";

const ServerActivity = async () => {
    const personalData = await fetchPersonalData();

    return (
        <>
            <Header user={personalData}></Header>
        </>
    );
}

export default ServerActivity;
