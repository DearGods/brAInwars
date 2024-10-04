use crate::test_app::base::TestApp;
use backend_lib::domain::id::Id;
use backend_lib::routes::basic_response::ResponseStatus;
use backend_lib::routes::games::get::GetGameResponse;
use backend_lib::routes::games::index::IndexGameResponse;
use backend_lib::routes::games::join_game::{JoinGameRequest, JoinGameResponse};
use backend_lib::routes::games::leave_game::{LeaveGameRequest, LeaveGameResponse};
use uuid::Uuid;

impl TestApp {
    pub async fn leave_game(&mut self, id: &Uuid) -> LeaveGameResponse {
        let body: LeaveGameRequest = LeaveGameRequest {
            game_id: Id::from(*id),
        };
        let response = self
            .api_client
            .post(&format!("{}/games/leave_game", &self.address))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");
        let response: LeaveGameResponse = response.json::<LeaveGameResponse>().await.unwrap();
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(response.message, None);
        response
    }

    pub async fn join_game(&mut self, id: &Uuid) -> JoinGameResponse {
        let body: JoinGameRequest = JoinGameRequest {
            game_id: Id::from(*id),
        };
        let response = self
            .api_client
            .post(&format!("{}/games/join_game", &self.address))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");
        let response: JoinGameResponse = response.json::<JoinGameResponse>().await.unwrap();
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(response.message, None);
        response
    }

    pub async fn get_game(&self, id: Uuid) -> GetGameResponse {
        let response = self
            .api_client
            .get(&format!("{}/games/get?game_id={}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.");
        let response: GetGameResponse = response.json::<GetGameResponse>().await.unwrap();
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(response.message, None);
        response
    }

    pub async fn get_games(&self) -> IndexGameResponse {
        let response = self
            .api_client
            .get(&format!("{}/games/index", &self.address))
            .send()
            .await
            .expect("Failed to execute request.");

        let response: IndexGameResponse = response.json::<IndexGameResponse>().await.unwrap();
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(response.message, None);
        response
    }
}
