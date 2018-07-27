CREATE TABLE `tasks` (
	`id` INTEGER NOT NULL PRIMARY KEY,
	`creator` INTEGER NOT NULL,
	`reminder_frequency` INTEGER,
	`create_date` DATE NOT NULL DEFAULT CURRENT_DATE,
	`due_date` DATE,
	`name` TEXT NOT NULL,

	FOREIGN KEY (`creator`) REFERENCES `users`(`id`)
);

CREATE TABLE `users` (
	`id` INTEGER NOT NULL PRIMARY KEY,
	`discord_id` TEXT NOT NULL UNIQUE,
	`nickname` TEXT
);

CREATE TABLE `task_assignments` (
	`id` INTEGER NOT NULL PRIMARY KEY,
	`user_id` INTEGER NOT NULL,
	`task_id` INTEGER NOT NULL,

	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	FOREIGN KEY (`task_id`) REFERENCES `tasks`(`id`)
);

CREATE TABLE `task_completions` (
	`id` INTEGER NOT NULL PRIMARY KEY,
	`user_id` INTEGER NOT NULL,
	`task_id` INTEGER NOT NULL,

	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	FOREIGN KEY (`task_id`) REFERENCES `tasks`(`id`)
);