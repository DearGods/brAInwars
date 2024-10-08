CREATE TABLE users
(
    id             uuid PRIMARY KEY,
    sid            BIGSERIAL   NOT NULL UNIQUE,
    wallet_address TEXT        NOT NULL UNIQUE,
    name           TEXT        NOT NULL UNIQUE,
    created_at     timestamptz NOT NULL
);

CREATE INDEX user_created_at ON users (created_at);
CREATE INDEX users_wallet_address ON users (wallet_address);
CREATE INDEX users_name ON users (name);

CREATE TRIGGER trg_make_archive_of_changes_for_users
    AFTER INSERT OR DELETE OR UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('User');

