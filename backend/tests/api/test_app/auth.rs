use crate::test_app::base::TestApp;
use backend_lib::routes::auth::login::{LoginRequest, LoginResponse};
use backend_lib::routes::auth::logout::{LogoutRequest, LogoutResponse};

impl TestApp {
    pub async fn post_login(
        &self,
        wallet_address: &String,
        signed_message: &String,
        nonce: &String,
    ) -> LoginResponse {
        let body: LoginRequest = LoginRequest {
            wallet_address: wallet_address.clone(),
            signed_message: signed_message.clone(),
            nonce: nonce.clone(),
        };
        let response = self
            .api_client
            .post(&format!("{}/auth/login", &self.address))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");
        let response: LoginResponse = response.json::<LoginResponse>().await.unwrap();
        response
    }

    pub async fn post_logout(&self, wallet_address: &String) -> LogoutResponse {
        let body: LogoutRequest = LogoutRequest {
            wallet_address: wallet_address.clone(),
        };
        let response = self
            .api_client
            .post(&format!("{}/auth/logout", &self.address))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");
        let response: LogoutResponse = response.json::<LogoutResponse>().await.unwrap();
        response
    }

    pub async fn get_protected(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/auth/protected", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
