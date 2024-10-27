export type Metadata = {
    topics: Topic[]
}

export type Topic = {
    name: string,
    partitions_count: number
}