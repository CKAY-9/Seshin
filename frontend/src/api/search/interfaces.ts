export interface UserPreview {
    display_name: string,
    username: string,
    avatar_url: string,
    public_id: string
}

export interface TopicPreview {
    name: string,
    bio: string,
    subscribers: number
}

export interface EventPreview {
    name: string,
    bio: string,
    members: string,
    datePosted: number
}

export interface SearchResponse {
    users: UserPreview[],
    topics: TopicPreview[],
    events: EventPreview[]
}
