import UserClient from "./client";
import UserServer from "./server";

const AppUser = ({ params }: { params: { slug : string } }) => {
    return (
        <UserClient>
            {/* @ts-expect-error Async Server Component */}
            <UserServer params={params}></UserServer>
        </UserClient> 
    );
}

export default AppUser;
