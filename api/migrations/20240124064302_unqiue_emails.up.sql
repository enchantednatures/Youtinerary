-- Add up migration script here
alter table users
add constraint users_unique_emails
unique (email);
