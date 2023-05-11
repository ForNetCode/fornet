-- this is for easy develop

insert into network(name, address_range, setting)  values ('test_network', '10.0.0.0/16', '{"mtu": 1420, "port": 51820, "keepAlive": 30}');
-- client
insert into node(name, network_id,ip, public_key, setting, node_type, status) values ('client',1,'10.0.0.2','GjRhJIjhkMBaWyQ59Pu/vHyNCmCcFwKjef5xy06BIT3lGQx+dYw9GDLCxWEshed/pARu4YnzFrDS+k73D1vkfw==','{"keepAlive":25, "mtu":1420}',0,2);
--mock-server
insert into node(name, network_id,ip, public_key, setting, node_type, status) values ('dev_fake_server', 1, '10.0.0.1', 'lpgpJqleWa1zqrk/O/jRThnqK1dGDzogKKicoefQrFv2PkzMj6DTE/dPLgzcaof3+0CaLroYph0bPuLq9Kwu3A==',
                                                                                      '{"keepAlive":25, "endpoint": "124.71.208.98", "port": 51820}', 1, 2);


-- insert into node_graph(network_id, node_id, ref_id, link_type) values (1, 2, 1, 0);

