CREATE TABLE games
(
    created_at                     timestamptz NOT NULL,
    id                             uuid        NOT NULL,
    PRIMARY KEY (id),

    game_id                        BIGINT      NOT NULL UNIQUE,
    entry_fee                      BIGINT      NOT NULL,
    mint                           TEXT        NOT NULL,
    start_time                     BIGINT,
    waiting_for_players_start_time BIGINT,
    winner                         TEXT,
    game_status                    TEXT        NOT NULL,
    wait_for_players_limit         BIGINT      NOT NULL,
    players_actions                TEXT        NOT NULL,
    hashed_limit                   BIGINT      NOT NULL,
    reveled_limit                  BIGINT      NOT NULL,
    reveled_salt                   BIGINT      NOT NULL,

    end_time                       BIGINT,
    num_participants               BIGINT      NOT NULL,
    num_participants_left_game     BIGINT      NOT NULL,
    mev_lock                       BOOL        NOT NULL,
    CONSTRAINT entry_fee check (entry_fee >= 0),
    CONSTRAINT hashed_limit check (hashed_limit >= 0),
    CONSTRAINT reveled_limit check (reveled_limit >= 0),
    CONSTRAINT reveled_salt check (reveled_salt >= 0),

    CONSTRAINT num_participants check (num_participants >= 0),
    CONSTRAINT num_participants_left_game check (num_participants >= 0),
    CONSTRAINT fk_user FOREIGN KEY (winner) REFERENCES users (wallet_address)
);

CREATE INDEX games_created_at ON games (created_at);
CREATE INDEX games_entry_fee ON games (entry_fee);
CREATE INDEX games_mint ON games (mint);
CREATE INDEX games_start_time ON games (start_time);
CREATE INDEX games_waiting_for_players_start_time ON games (waiting_for_players_start_time);
CREATE INDEX games_winner ON games (winner);
CREATE INDEX games_game_status ON games (game_status);
CREATE INDEX games_players_actions ON games (players_actions);
CREATE INDEX games_hashed_limit ON games (hashed_limit);
CREATE INDEX games_reveled_limit ON games (reveled_limit);
CREATE INDEX games_reveled_salt ON games (reveled_salt);
CREATE INDEX games_num_participants ON games (num_participants);
CREATE INDEX games_num_participants_left_game ON games (num_participants_left_game);

CREATE TRIGGER trg_make_archive_of_changes_for_games
    AFTER INSERT OR DELETE OR UPDATE
    ON games
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Game');