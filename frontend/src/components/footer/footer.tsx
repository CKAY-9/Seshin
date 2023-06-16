import Link from "next/link";
import style from "./footer.module.scss";

const Footer = () => {
    return (
        <footer className={style.footer}>
            <section>
                <ul>
                    <strong>Navigation</strong>
                    <Link href="/">Seshin Landing</Link>
                    <Link href="/app">Seshin Home</Link>
                    <Link href="/app/search">Search</Link>
                </ul>
            </section>
            <section style={{"gap": "0.5rem"}}>
                Made with ❤️ by <Link href="https://github.com/Camerxxn/Seshin" target="_blank">Camerxn</Link>
            </section>
        </footer>
    );
}

export default Footer;
