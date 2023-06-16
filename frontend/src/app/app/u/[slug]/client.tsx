"use client";

import {UserPreview} from "@/api/search/interfaces";
import {fetchUserPreview} from "@/api/user/fetching";
import Footer from "@/components/footer/footer";
import {getCookie} from "@/utils/cookie";
import Image from "next/image";
import {BaseSyntheticEvent, useState} from "react";
import style from "./user.module.scss";

export const FollowersButton = (props: {followers: string[]}) => {
    const [loadedFollowers, setLoadedFollowers] = useState<UserPreview[]>([]);
    const [limiter, setLimiter] = useState<number>(10);
    const [visible, setVisible] = useState<boolean>(false);

    const fetchFollowerData = async (e: BaseSyntheticEvent) => {
        if (typeof(document) === undefined)
            return;

        let token = getCookie("token");
        if (token === null || token === undefined)
            return;

        if (limiter > props.followers.length) 
            setLimiter(props.followers.length);

        for (let i = 0; i < limiter; i++) {
            const req = await fetchUserPreview(token, props.followers[i]);
            if (req === undefined) 
                continue;

            setLoadedFollowers((old) => [
                ...old,
                req
            ]);
        }
    }

    return (
        <>
            {visible && 
                <div className={style.popup}>
                    <div className={style.followers}>
                        <h2>Followers</h2>
                        <div className={style.list}>
                            {loadedFollowers.map((follower: UserPreview, index: number) => {
                                return (
                                    <div className={style.follower}>
                                        <section className={style.avatar}>
                                            <Image
                                                alt="User Icon"
                                                src={follower.avatar_url || ""} 
                                                width={0} 
                                                height={0}
                                                sizes="100%" />
                                        </section> 
                                        <section className={style.info}>
                                            <span>{follower.display_name}</span>
                                            <span>@{follower.username}</span>
                                        </section>
                                    </div>
                                );
                            })}
                        </div>
                        <button className="noBG noPadding" onClick={() => setVisible(false)}>Close</button>
                    </div>  
                </div>
            }
            <button onClick={async (e) => {
                if (props.followers.length >= 1) {
                    await fetchFollowerData(e);
                    setVisible(true);
                }
                setVisible(true)
            }} className="noBG noPadding">{props.followers.length} Followers</button>
        </>
    );
}

const UserClient = ({children}: any) => {
    return (
        <>
            {children}
            <Footer></Footer>
        </>
    );
}

export default UserClient;
