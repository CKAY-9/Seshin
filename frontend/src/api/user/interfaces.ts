export interface PersonalUser {
    username: string,
    email: string,
    avatar_url: string,
    display_name: string,
    joined_groups: string[],
    joined_events: string[],
    token: string,
    public_id: string,
    oauth_type: string,
    followers: string[]
}

export interface PublicUser {
    username: string,
    avatar_url: string,
    public_id: string,
    oauth_type: string,
    display_name: string,
    followers: string[]
}
