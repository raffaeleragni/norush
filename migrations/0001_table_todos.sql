create table if not exists todos (
    id integer primary key autoincrement,
    title varchar(200) default 'todo',
    state varchar(50) default 'todo',
    tags varchar(200) default ''
);
