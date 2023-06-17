import SearchClient from "./client";
import SearchServer from "./server";

const SearchPage = () => {
    return (
        <>
            {/* @ts-expect-error Async Server Component */}
            <SearchServer></SearchServer>
        </>
    );
}

export default SearchPage;
