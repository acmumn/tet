table! {
    tasks(id) {
        id -> Integer,
    }
}

table! {
    users(id) {
        id -> Integer,
    }
}

table! {
    task_assignments(id) {
        id -> Integer,
        user_id -> Integer,
        task_id -> Integer,
    }
}

table! {
    task_completions(id) {
        id -> Integer,
        user_id -> Integer,
        task_id -> Integer,
    }
}
