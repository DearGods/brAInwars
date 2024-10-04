use crate::test_app::base::TestApp;
use backend_lib::blockchain::solana::helpers::sign_message;
use backend_lib::routes::basic_response::ResponseStatus;
use backend_lib::routes::users::create::{CreateUserRequest, CreateUserResponse};
use backend_lib::routes::users::get::{GetUserRequest, GetUserResponse};
use solana_sdk::signature::{Keypair, Signer};

impl TestApp {
    pub async fn create_user(&self, wallet: &Keypair) -> CreateUserResponse {
        let wallet_address = wallet.pubkey().to_string();
        let body: CreateUserRequest = CreateUserRequest {
            wallet_address: wallet_address.clone(),
        };
        let response = self
            .api_client
            .post(&format!("{}/users/create", &self.address))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(response.headers().get("sid").is_none());
        let response: CreateUserResponse = response.json::<CreateUserResponse>().await.unwrap();
        response
    }

    pub async fn get_user(&self, wallet_address: String) -> GetUserResponse {
        let body: GetUserRequest = GetUserRequest { wallet_address };
        let response = self
            .api_client
            .post(&format!("{}/users/get", &self.address))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");
        let response: GetUserResponse = response.json::<GetUserResponse>().await.unwrap();
        response
    }

    pub async fn get_logged_in_user(&self, wallet: &Keypair) -> GetUserResponse {
        let response = self.create_user(wallet).await;
        assert_eq!(response.status, ResponseStatus::Success);
        let signed_message = sign_message(&response.nonce.clone().unwrap(), &wallet).unwrap();
        self.post_login(
            &wallet.pubkey().to_string(),
            &signed_message,
            &response.nonce.unwrap(),
        )
        .await;
        self.get_user(wallet.pubkey().to_string()).await
    }
}
