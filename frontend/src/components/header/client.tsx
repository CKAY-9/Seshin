"use client";

import {eraseCookie} from "@/utils/cookie";
import {BaseSyntheticEvent} from "react";

import style from "./header.module.scss";

export const LogoutButton = () => {
    const logout = (e: BaseSyntheticEvent) => {
        if (typeof(window) === undefined || typeof(document) === undefined) return;

        eraseCookie("token");
        window.location.href = "/login";
    }

    return (
        <button onClick={logout} className={style.logout}>
            LOGOUT
        </button>
    ); 
}
