alter table network add column token text not null default 'network_token';
comment on column network.token is 'special token to protect from brute force';
alter table node add column device_id integer not null default 0;
alter table node drop column public_key;

create table device (
    id serial primary key,
    public_key text not null,
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now()
);
