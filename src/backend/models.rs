use super::schema::*;

#[derive(Clone, Debug, Identifiable, Queryable)]
pub struct Task {
	id: u32,
}

#[derive(Clone, Debug, Identifiable, Queryable)]
pub struct User {
	id: u32,
}