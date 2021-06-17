drop table if exists todo_list;
drop table if exists eshop_product;

create table eshop_product (
    id serial primary key,
    title varchar(150) not null,
    message varchar(100000) not null,
    stock integer not null
);

insert into eshop_product (title, message, stock) values
(
    'con co be be',
    'mo ta con co be be',
    100
),
(
    'con cun dang yeu',
    'mo ta con cun',
    1000
);