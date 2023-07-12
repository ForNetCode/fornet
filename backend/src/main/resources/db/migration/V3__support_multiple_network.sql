alter table network
    add column token text not null default 'network_token';
comment on column network.token is 'special token to protect from brute force';

alter table node
    add column device_id integer not null default 0;

create index if not exists node_device_id_index on node (device_id);

drop index node_public_key_index;

alter table node
    drop column public_key;

create table device
(
    id              serial primary key,
    token           text        not null,
    public_key      text        not null,
    mqtt_last_leave timestamptz, -- would use in future version.
    created_at      timestamptz not null default now()
);

create  unique index if not exists  device_public_key_index on device(public_key);