-- Add migration script here
CREATE TABLE configs
(
    id         uuid PRIMARY KEY,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    key        TEXT        NOT NULL UNIQUE,
    value      TEXT        NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX config_created_at ON configs (created_at);
-- CREATE INDEX users_verified_email ON users (verified_email);
CREATE INDEX config_key ON configs (key);

CREATE TRIGGER trg_make_archive_of_changes_for_configs
    AFTER INSERT OR DELETE OR UPDATE
    ON configs
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Configs');

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'min_game_length', '{ "min_game_length": 3 }', current_timestamp);

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'max_game_length', '{ "max_game_length": 45 }', current_timestamp);

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'game_entries', '{ "game_entries": [10.0, 20.0] }', current_timestamp);

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'countdown_for_others_to_join', '{ "countdown_for_others_to_join": 5000 }',
        current_timestamp);

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'worker_interval', '{ "worker_interval": 1000 }', current_timestamp);

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'eth_rpc',
        '{ "eth_rpc": "https://dawn-evocative-haze.discover.quiknode.pro/13f8d0312edcd65ea706b693e4f52dae14bf538b/" }',
        current_timestamp);

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'eth_scan_api_key', '{ "eth_scan_api_key": "37X7TJFTZMQ6MC4M97DPGQ9ZXCTE8HG9RE" }',
        current_timestamp);

INSERT
INTO configs(id, key, value, created_at)
VALUES (gen_random_uuid(), 'eth_moralis_api_key',
        '{ "eth_moralis_api_key": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJub25jZSI6IjRiYzJmNTI0LTU5ZTYtNGZmNy05MjdmLTdiOTJkNzI5OTUxOCIsIm9yZ0lkIjoiMzUyOTU1IiwidXNlcklkIjoiMzYyNzcwIiwidHlwZUlkIjoiY2FiNjNlMGUtZGY3YS00NjgyLWEzOGUtNzJhMWJkYWEwOWUzIiwidHlwZSI6IlBST0pFQ1QiLCJpYXQiOjE2OTE5NTgyNDksImV4cCI6NDg0NzcxODI0OX0.YNXNp9u37qv3XHe3267aCihGpDeBPfkSJM_rooqsKZA" }',
        current_timestamp);







