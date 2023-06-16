import Footer from "@/components/footer/footer";

const UserClient = ({children}: any) => {
    return (
        <>
            {children}
            <Footer></Footer>
        </>
    );
}

export default UserClient;
