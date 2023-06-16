import {getCookie} from "@/utils/cookie";
import axios, {AxiosResponse} from "axios";
import {cookies} from "next/dist/client/components/headers";
import {SearchResponse} from "./interfaces";

export const fetchSearchData = async (manualToken: string = "", index: number, query: string): Promise<SearchResponse | undefined> => {
    let token;
    if (manualToken === "") {
        if (typeof(document) === undefined) return;
        token = getCookie("token");
    } else {
        token = manualToken;
    }

    if (token === undefined) {
        return undefined;
    }

    console.log(token);
   
    const req: AxiosResponse<SearchResponse> = await axios({
        url: process.env.NEXT_PUBLIC_API + "/api/search/all",
        method: "GET",
        params: {
            "search_query": query
        },
        headers: {
            "authorization": token
        }
    });

    return req.data;
}
