table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        publishingcompany -> Varchar,
        pages -> Nullable<Int4>,
        evaluation -> Nullable<Int4>,
        cover -> Nullable<Varchar>,
        read -> Nullable<Varchar>,
        writer -> Nullable<Varchar>,
        createdat -> Nullable<Timestamp>,
        updatedat -> Nullable<Timestamp>,
    }
}

table! {
    comics (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        serie -> Nullable<Varchar>,
        publishingcompany -> Varchar,
        pages -> Nullable<Int4>,
        evaluation -> Nullable<Int4>,
        cover -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        read -> Nullable<Varchar>,
        writer -> Nullable<Varchar>,
        artist -> Nullable<Varchar>,
        createdat -> Nullable<Timestamp>,
        updatedat -> Nullable<Timestamp>,
    }
}

table! {
    mangas (id) {
        id -> Int4,
        title -> Varchar,
        publishingcompany -> Varchar,
        pages -> Nullable<Int4>,
        evaluation -> Nullable<Int4>,
        cover -> Nullable<Varchar>,
        read -> Nullable<Varchar>,
        writer -> Nullable<Varchar>,
        artist -> Nullable<Varchar>,
        createdat -> Nullable<Timestamp>,
        updatedat -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    comics,
    mangas,
);
