begin transaction;

drop table if exists users cascade;

drop table if exists itineraries cascade;
drop table if exists itinerary_start_date cascade;
drop table if exists itinerary_end_date cascade;
drop table if exists itinerary_shares cascade;
drop table if exists itinerary_items cascade;
drop table if exists stays cascade;
drop table if exists activities cascade;
drop table if exists travel_legs cascade;
drop table if exists flights cascade;
drop table if exists itinerary_flights cascade;

drop type itinerary_status;
drop type itinerary_share_type;
drop type travel_leg_type;

commit transaction;
