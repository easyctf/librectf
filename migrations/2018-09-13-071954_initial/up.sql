CREATE TABLE IF NOT EXISTS `challenges` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(64) NOT NULL,
    `description` TEXT NOT NULL,
    `hint` TEXT,
    `value` INTEGER NOT NULL,

    PRIMARY KEY (`id`)
);
CREATE INDEX challenges_value_idx ON `challenges`(`value`);

CREATE TABLE IF NOT EXISTS `config` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,

    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `teams` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,
    `teamname` VARCHAR(32) NOT NULL UNIQUE,
    `affiliation` VARCHAR(32),
    `banned` BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY (`id`)
);
CREATE INDEX teams_teamname_idx ON `teams`(`teamname`);

CREATE TABLE IF NOT EXISTS `users` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,
    `tid` INTEGER,
    `admin` BOOLEAN NOT NULL DEFAULT FALSE,
    `email` VARCHAR(128) NOT NULL UNIQUE,
    `password` VARCHAR(128) NOT NULL UNIQUE,

    `date_created` DATETIME NOT NULL DEFAULT NOW(),

    PRIMARY KEY (`id`),
    FOREIGN KEY (`tid`) REFERENCES `teams`(`id`)
);
CREATE INDEX users_email_idx ON `users`(`email`);
