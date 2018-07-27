use super::schema::*;

#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[table_name = "tasks"]
pub struct Task {
    id: u32,
    /// How frequently users should be reminded of this task.
    reminder_frequency: u32,
}

#[derive(Clone, Debug, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    id: u32,
}

#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[table_name = "task_assignments"]
#[belongs_to(User)]
#[belongs_to(Task)]
pub struct TaskAssignment {
    id: u32,
    user_id: u32,
    task_id: u32,
}

#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[table_name = "task_completions"]
#[belongs_to(User)]
#[belongs_to(Task)]
pub struct TaskCompletion {
    id: u32,
    user_id: u32,
    task_id: u32,
}
