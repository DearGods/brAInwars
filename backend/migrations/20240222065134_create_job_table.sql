-- Add migration script here
CREATE TABLE jobs
(
    id             uuid PRIMARY KEY,
    sid            BIGSERIAL   NOT NULL UNIQUE,
    status         TEXT        NOT NULL,
    job_type       TEXT        NOT NULL,
    wallet_address TEXT,
    game_uuid      uuid        NOT NULL,
    created_at     timestamptz NOT NULL,
    error          TEXT,
    txn            TEXT,
    CONSTRAINT fk_game FOREIGN KEY (game_uuid) REFERENCES games (id)
);

CREATE INDEX jobs_created_at ON jobs (created_at);
CREATE INDEX jobs_status ON jobs (status);
CREATE INDEX jobs_game_uuid ON jobs (game_uuid);
CREATE INDEX jobs_wallet_address ON jobs (wallet_address);
CREATE INDEX jobs_job_type ON jobs (job_type);
CREATE INDEX jobs_txn ON jobs (txn);

CREATE TRIGGER trg_make_archive_of_changes_for_jobs
    AFTER INSERT OR DELETE OR UPDATE
    ON jobs
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Job');
