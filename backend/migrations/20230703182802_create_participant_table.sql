CREATE TABLE participants
(
    id         uuid        NOT NULL,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    PRIMARY KEY (id),
    created_at timestamptz NOT NULL,
    game_id    uuid        NOT NULL,
    user_id    uuid        NOT NULL,
    exit_time  timestamptz,
    status     text        NOT NULL,
    UNIQUE (game_id, user_id),
    CONSTRAINT fk_game FOREIGN KEY (game_id) REFERENCES games (id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX participants_created_at ON participants (created_at);
CREATE INDEX participants_exit_time ON participants (exit_time);
CREATE INDEX participants_game_id ON participants (game_id);
CREATE INDEX participants_user_id ON participants (user_id);
CREATE INDEX participants_status ON participants (status);

CREATE TRIGGER trg_make_archive_of_changes_for_participants
    AFTER INSERT OR DELETE OR UPDATE
    ON participants
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Participant');