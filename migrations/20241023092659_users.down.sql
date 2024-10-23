-- Add down migration script here
drop table users;
drop type user_role
drop extension if exists "uuid-ossp";