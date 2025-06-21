create table tb_roles
(
    id          bigint unsigned auto_increment
        primary key,
    name        varchar(256) not null,
    description varchar(256) null,
    create_date datetime     not null,
    constraint tb_roles_unique
        unique (name)
);

create table tb_user_roles
(
    user_id     bigint unsigned not null,
    role_id     bigint unsigned not null,
    create_date datetime        null,
    primary key (role_id, user_id)
);

create table tb_users
(
    username    varchar(256) not null,
    password    varchar(256) not null,
    create_date datetime     not null,
    id          bigint unsigned auto_increment
        primary key,
    constraint tb_users_unique
        unique (username)
);

