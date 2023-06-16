import {PersonalUser} from "@/api/user/interfaces";
import Image from "next/image";
import Link from "next/link";
import {LogoutButton} from "./client";
import style from "./header.module.scss";

const Header = (props: {user: PersonalUser | undefined}) => {
    return (
        <header className={style.header}>
            <section id="left">
                <Link href="/app">Seshin</Link>
                <Link href="/app/activity">Activity</Link>
                <Link href="/app/search">Search</Link>
            </section>
            <section id="right">
                {props.user === undefined 
                    ? <>
                        <Link href="/login">Login</Link> 
                    </>
                    : <>
                        <Link href="/app/u/me/events" style={{"position": "relative"}}>
                            <Image alt="User Icon" src={"/events.svg"} width={0} height={0} sizes="100%" style={{
                                "width": "2rem",
                                "height": "2rem"
                            }} />
                        </Link>
                        <LogoutButton></LogoutButton>
                        <Link href="/app/u/me" className={style.profile}>
                            <span>{props.user.display_name}</span>
                            <Image alt="User Icon" src={props.user.avatar_url || ""} width={0} height={0} sizes="100%" />
                        </Link>
                    </>
                }
            </section>
        </header>
    );     
}

export default Header;
