create table network
(
    id            serial primary key,
    name          text        not null,
    address_range text        not null,
    setting       jsonb       not null,
    status        smallint    not null default 0,
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now()
);
-- alter table network add column status smallint not null default 0;
-- drop table node;
create table if not exists node
(
    id         serial primary key,
    name       text        not null,
    network_id integer     not null,
    ip         text        not null,
    public_key text        not null default '',
    setting    jsonb       not null,
    node_type  smallint    not null default 0,
    status     smallint    not null default 0,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
-- alter table node alter column public_key set default '';
create index if not exists node_network_id_index on node (network_id);
create index if not exists node_public_key_index on node (public_key);

-- create table node_graph
-- (
--     network_id integer     not null,
--     node_id    integer     not null,
--     link_ids   integer[]   not null default Array []::integer[],
--     created_at timestamptz not null default now(),
--     updated_at timestamptz not null default now()
-- );
-- create index if not exists node_graph_network_id_index on node_graph (network_id);

-- create table if not exists session(
--   node_id integer not null,
--   network_id integer not null,
--   session_id text not null
-- );
-- create unique index if not exists  session_session_id_index on session(session_id);


-- drop table node_graph;
-- create table node_graph(
--     network_id integer not null,
--     node_id integer not null,
--     ref_id integer not null,
--     link_type smallint not null,
--     created_at timestamptz not null default now()
-- );
-- comment on column node_graph.link_type is '0: parent_to_child,1: peer';
-- 
-- create index if not exists node_graph_network_id_index on node_graph (network_id);
-- 
-- create unique index if not exists node_graph_node_id_ref_id on node_graph(node_id, ref_id);