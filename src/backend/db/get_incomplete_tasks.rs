use diesel::prelude::*;
use failure::{Error,err_msg};

use backend::{models::{User, Task}, schema::tasks};
use Tet;

impl Tet {
	// pub fn get_incomplete_tasks_for(&self, user: User) -> Result<Vec<Task>, Error> {
	// 	let db = self.db.lock().unwrap();
	// 	tasks::table.order(tasks::due_date.desc()).load(&*db)
	// }
}