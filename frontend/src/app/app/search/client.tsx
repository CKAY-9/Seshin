"use client";

import {fetchSearchData} from "@/api/search/fetch";
import {EventPreview, TopicPreview, UserPreview} from "@/api/search/interfaces";
import {PersonalUser} from "@/api/user/interfaces";
import Footer from "@/components/footer/footer";
import Image from "next/image";
import Link from "next/link";
import {BaseSyntheticEvent, useState} from "react";
import style from "./search.module.scss";

interface SubProps {
    data: UserPreview[] | EventPreview[] | TopicPreview[],
    user: PersonalUser | undefined
}

const Users = (props: SubProps) => {
    console.log(props);

    return (
        <>
            {(props.data as UserPreview[]).map((preview: UserPreview, index: number) => {
                const isMe: boolean = (props.user !== undefined) && (props.user.public_id === preview.public_id);
                return (
                    <Link href={`/app/u/${isMe ? "me" : "preview.public_id"}`} className={style.preview} key={index}>
                        <div style={{"position": "relative"}}>
                            <Image src={preview.avatar_url || ""} alt="User Icon" width={0} height={0} sizes="100%" style={{
                                "width": "4rem",
                                "height": "4rem",
                                "borderRadius": "50%"
                            }}></Image>
                        </div>
                        <div className={style.info}>
                            <span className={style.displayName}>{preview.display_name}{isMe && 
                                <span style={{"opacity": "0.5", "marginLeft": "0.5rem"}}>(You)</span>
                            }</span>
                            <span className={style.username}>{preview.username}</span>
                        </div>
                    </Link>
                );
            })}
        </>
    );
}

const SearchClient = (props: {user: PersonalUser | undefined}, {children}: any) => {
    const [searchIndex, setSearchIndex] = useState<number>(0); // This is increased by 10 on the server to fetch further items
    const [searchQuery, setSearchQuery] = useState<string>("");
    
    const [view, setView] = useState<number>(0);
    const [users, setUsers] = useState<UserPreview[]>([]);

    const search = async (e: BaseSyntheticEvent) => {
        e.preventDefault();
        const fetchSearch = await fetchSearchData("", searchIndex, searchQuery); 
        setUsers(fetchSearch?.users as UserPreview[]);
    }

    return (
        <>
            {children}
            <main className="container">
                <section style={{
                    "textAlign": "center"        
                }}>
                    <input 
                        type="text"
                        name="search" 
                        onChange={(e: BaseSyntheticEvent) => setSearchQuery(e.target.value)} 
                        placeholder="Search Seshin..."></input>
                    <input type="submit" onClick={search} value="Go"></input>
                </section>

                <nav className={style.nav}>
                    <button>Users</button>
                    <button>Topics</button>
                    <button>Events</button>
                </nav>
            
                <section className={style.results}>
                    {view === 0 && <Users data={users} user={props.user}></Users>}
                </section>
            </main>
            <Footer></Footer>
        </>
    );
}

export default SearchClient;
