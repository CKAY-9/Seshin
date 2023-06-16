"use client";

import { setCookie } from "@/utils/cookie";
import { redirect, useSearchParams } from "next/navigation";
import { useEffect } from "react";

const ClientLogin = () => {
    const searchParams = useSearchParams();
    const token = searchParams.get("token");
    const docReady = typeof(document);

    useEffect(() => {
        if (docReady === undefined) return;
        if (token !== null) {
            setCookie("token", token, 365);
            redirect("/app");
        }
    }, [docReady, token]);

    return (
        <>
            <title>Login</title>
        </>
    );
}

export default ClientLogin;
