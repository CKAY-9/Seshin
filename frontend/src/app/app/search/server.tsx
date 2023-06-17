import {fetchPersonalData} from "@/api/user/fetching";
import Header from "@/components/header/header";
import SearchClient from "./client";

const SearchServer = async () => {
    const personalData = await fetchPersonalData();
    return (
        <>
            <Header user={personalData}></Header>
            <SearchClient user={personalData}></SearchClient>
        </>
    );     
}

export default SearchServer;
