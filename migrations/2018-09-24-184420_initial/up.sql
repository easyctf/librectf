CREATE TABLE `chals` (
    `id` INTEGER PRIMARY KEY AUTO_INCREMENT,
    `title` VARCHAR(64) NOT NULL,
    `enabled` BOOLEAN NOT NULL DEFAULT FALSE,
    `description` TEXT NOT NULL,

    `correct_flag` TEXT NOT NULL,
    `regex` BOOLEAN NOT NULL DEFAULT FALSE,
    `value` INTEGER NOT NULL,

    -- constraints
    UNIQUE(`title`)
);
CREATE INDEX `chals_title_idx` ON `chals`(`title`);
CREATE INDEX `chals_value_idx` ON `chals`(`value`);

CREATE TABLE `teams` (
    `id` INTEGER PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(20) COLLATE utf8mb4_general_ci NOT NULL,
    `affiliation` VARCHAR(20),
    `banned` BOOLEAN NOT NULL,

    -- constraints
    UNIQUE (`name`)
);

CREATE TABLE `users` (
    `id` INTEGER PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(20) COLLATE utf8mb4_general_ci NOT NULL,
    `email` VARCHAR(128) NOT NULL,
    `email_verified` BOOLEAN NOT NULL DEFAULT FALSE,
    `password` VARCHAR(64) NOT NULL,
    `admin` BOOLEAN NOT NULL DEFAULT FALSE,

    -- foreign keys
    `team_id` INTEGER NULL,

    -- constraints
    UNIQUE (`name`),
    UNIQUE (`email`),
    CONSTRAINT `user_team_fk` FOREIGN KEY (`team_id`) REFERENCES `teams`(`id`)
);

CREATE TABLE `solves` (
    `id` INTEGER PRIMARY KEY AUTO_INCREMENT,
    `timestamp` DATETIME NOT NULL DEFAULT NOW(),
    `flag` TEXT NOT NULL,

    -- foreign keys
    `chal_id` INTEGER NOT NULL,
    `team_id` INTEGER NOT NULL,
    `user_id` INTEGER NOT NULL,

    -- constraints
    UNIQUE(`chal_id`, `team_id`),
    CONSTRAINT `solve_chal_fk` FOREIGN KEY (`chal_id`) REFERENCES `chals`(`id`),
    CONSTRAINT `solve_team_fk` FOREIGN KEY (`team_id`) REFERENCES `teams`(`id`),
    CONSTRAINT `solve_user_fk` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);

CREATE TABLE `tasks` (
    `id` INTEGER PRIMARY KEY AUTO_INCREMENT,
    `created` DATETIME NOT NULL DEFAULT NOW(),
    `claimed` DATETIME DEFAULT NULL,
    `completed` DATETIME DEFAULT NULL,

    `name` VARCHAR(16) NOT NULL,
    `payload` TEXT
);
