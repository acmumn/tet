table! {
    task_assignments (id) {
        id -> Integer,
        user_id -> Integer,
        task_id -> Integer,
    }
}

table! {
    task_completions (id) {
        id -> Integer,
        user_id -> Integer,
        task_id -> Integer,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        creator -> Integer,
        reminder_frequency -> Nullable<Integer>,
        create_date -> Date,
        due_date -> Nullable<Date>,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        discord_id -> Text,
        nickname -> Nullable<Text>,
    }
}

joinable!(task_assignments -> tasks (task_id));
joinable!(task_assignments -> users (user_id));
joinable!(task_completions -> tasks (task_id));
joinable!(task_completions -> users (user_id));
joinable!(tasks -> users (creator));

allow_tables_to_appear_in_same_query!(
    task_assignments,
    task_completions,
    tasks,
    users,
);
