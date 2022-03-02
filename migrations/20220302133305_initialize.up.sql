create table if not exists contents (
    id bigint unsigned not null auto_increment primary key,
    title varchar(240) not null,
    keywords varchar(500) not null default '',
    description varchar(500) not null default '',
    content_name varchar(240) default null,
    content_type integer unsigned not null default 0,
    is_published tinyint(1) not null default 0,
    published_at timestamp null,
    modified_at timestamp null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp on update current_timestamp,
    constraint unique unique_content_name (content_name)
);

create table if not exists content_entities (
    id bigint unsigned not null primary key,
    entity mediumtext not null,
    updated_at timestamp not null default current_timestamp on update current_timestamp
);