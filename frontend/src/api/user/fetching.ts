import axios, { AxiosResponse } from "axios";
import {cookies} from "next/dist/client/components/headers";
import {PersonalUser, PublicUser} from "./interfaces";

export const fetchPublicProfile = async (manualToken: string = "", userID: string): Promise<PublicUser | undefined> => {
    let token;
    if (manualToken === "") {
        const cookieStore = cookies();
        token = cookieStore.get("token")?.value;
    } else {
        token = manualToken;
    }

    if (token === undefined) {
        return undefined;
    }

    try {
        const dataRequest: AxiosResponse<PublicUser> = await axios({
            url: process.env.NEXT_PUBLIC_API + "/api/user/" + userID,
            method: "GET",
            headers: {
                authorization: token
            }
        });

        return dataRequest.data;
    } catch (ex) {
        console.log(ex);
        return undefined;
    }
}

export const fetchPersonalData = async (manualToken: string = ""): Promise<PersonalUser | undefined> => {
    let token;
    if (manualToken === "") {
        const cookieStore = cookies();
        token = cookieStore.get("token")?.value;
    } else {
        token = manualToken;
    }

    if (token === undefined) {
        return undefined;
    }
    
    try {
        const dataRequest: AxiosResponse<PersonalUser> = await axios({
            url: process.env.NEXT_PUBLIC_API + "/api/user/me",
            method: "GET",
            headers: {
                authorization: token
            }
        });
        return dataRequest.data;
    } catch (ex) {
        console.log(ex);
        return undefined;
    }
}
