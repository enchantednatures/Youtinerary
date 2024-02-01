-- Add up migration script here
create type itinerary_status as enum ('draft', 'published', 'archived');
create type itinerary_share_type as enum ('editor', 'viewer');
create type travel_leg_type as enum (
    'flight', 'train', 'bus', 'car', 'ferry', 'other'
);

-- create type 
create table users
(
    user_id serial not null
    constraint users_pk primary key,
    email varchar(255) not null,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);


create table itineraries
(
    itinerary_id serial not null
    constraint itineraries_pk
    primary key,
    user_id integer not null
    constraint itineraries_users_id_fk
    references users
    on update cascade on delete cascade,
    name varchar(255) not null,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);

create table itinerary_start_date
(
    itinerary_id integer not null
    constraint itinerary_start_date_itineraries_id_fk
    references itineraries
    on update cascade on delete cascade,
    start_date date not null
);

create table itinerary_end_date
(
    itinerary_id integer not null
    constraint itinerary_end_date_itineraries_id_fk
    references itineraries
    on update cascade on delete cascade,
    end_date date not null
);


create table itinerary_shares
(
    itinerary_id integer not null
    constraint itinerary_shares_itineraries_id_fk
    references itineraries
    on update cascade on delete cascade,
    user_id integer not null
    constraint itinerary_shares_users_id_fk
    references users
    on update cascade on delete cascade,
    share_type itinerary_share_type not null,
    share_message varchar(255) not null
);


create table itinerary_items
(
    id serial not null
    constraint itinerary_items_pk
    primary key,
    itinerary_id integer not null
    constraint itinerary_items_itineraries_id_fk
    references itineraries
    on update cascade on delete cascade,
    name varchar(255) not null
);

create table stays
(
    id serial not null
    constraint stay_pk
    primary key,
    summary varchar(255) not null,
    start_date date not null,
    end_date date not null,
    location point not null,
    notes varchar(255) not null
);

create table activities
(
    id serial not null
    constraint activities_pk
    primary key,
    summary varchar(255) not null,
    start_date date not null,
    end_date date not null,
    location point not null,
    notes varchar(255) not null
);

create table travel_legs
(
    id serial not null
    constraint travel_legs_pk
    primary key,
    itinerary_item_id integer not null
    constraint travel_legs_itinerary_items_id_fk
    references itinerary_items
    on update cascade on delete cascade,
    travel_leg_type travel_leg_type not null,
    start_date date not null,
    end_date date not null,
    start_location point not null,
    end_location point not null,
    notes varchar(255) not null
);

create table itinerary_flights (
    itinerary_id integer not null constraint
    itinerary_flights_itineraries_id_fk
    references itineraries
    on update cascade
    on delete cascade,
    flight_id integer not null
    constraint flight_flights_itineraries_id_fk references itineraries
    on update cascade
    on delete cascade
);

create table flights
(
    id serial not null
    constraint flight_pk
    primary key,

    airline varchar(255) not null,
    confirmation_code varchar(50) not null,

    departure_time timestamp with time zone not null,
    arrival_time timestamp with time zone not null,

    notes varchar(255) not null
);
