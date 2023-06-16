import SearchClient from "./client";
import SearchServer from "./server";

const SearchPage = () => {
    return (
        <>
            <SearchClient>
                {/* @ts-expect-error Async Server Component */}
                <SearchServer></SearchServer>
            </SearchClient>
        </>
    );
}

export default SearchPage;
