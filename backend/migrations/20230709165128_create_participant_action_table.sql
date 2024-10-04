CREATE TABLE participant_actions
(
    id         uuid        NOT NULL,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    PRIMARY KEY (id),
    created_at timestamptz NOT NULL,
    game_id    uuid        NOT NULL,
    user_id    uuid        NOT NULL,
    action     text        NOT NULL,
    signature  text        NOT NULL,
    UNIQUE (game_id, user_id, action),
    CONSTRAINT fk_game FOREIGN KEY (game_id) REFERENCES games (id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX participant_actions_action ON participant_actions (action);
CREATE INDEX participant_actions_user_id ON participant_actions (user_id);
CREATE INDEX participant_actions_game_id ON participant_actions (game_id);
CREATE INDEX participant_actions_created_at ON participant_actions (created_at);
CREATE INDEX participant_actions_signature ON participant_actions (signature);

CREATE TRIGGER trg_make_archive_of_changes_for_participant_actions
    AFTER INSERT OR DELETE OR UPDATE
    ON participant_actions
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('ParticipantAaction');