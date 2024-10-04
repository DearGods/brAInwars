-- Add migration script here
CREATE TABLE nonces
(
    id             uuid PRIMARY KEY,
    sid            BIGSERIAL   NOT NULL UNIQUE,
    nonce          TEXT        NOT NULL,
    wallet_address TEXT        NOT NULL,
    created_at     timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (wallet_address) REFERENCES users (wallet_address)
);

CREATE INDEX nonces_created_at ON nonces (created_at);
CREATE INDEX nonces_nonce ON nonces (nonce);
CREATE INDEX nonces_wallet_address ON nonces (wallet_address);

CREATE TRIGGER trg_make_archive_of_changes_for_nonces
    AFTER INSERT OR DELETE OR UPDATE
    ON nonces
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Nonce');

