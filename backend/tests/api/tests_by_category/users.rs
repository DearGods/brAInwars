use crate::test_app::base::spawn_app;
use backend_lib::blockchain::solana::helpers::sign_message;
use backend_lib::routes::basic_response::ResponseStatus;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn create_user() {
    let app = spawn_app().await;
    let wallet = Keypair::new();
    let response = app.create_user(&wallet).await;
    assert_eq!(response.status, ResponseStatus::Success);
    assert_eq!(
        response.wallet_address.unwrap(),
        wallet.pubkey().to_string()
    );
}

#[tokio::test]
async fn get_user() {
    let app = spawn_app().await;
    let wallet = Keypair::new();
    let response = app.create_user(&wallet).await;
    assert_eq!(response.status, ResponseStatus::Success);
    assert_eq!(
        response.wallet_address.unwrap(),
        wallet.pubkey().to_string()
    );
    let signed_message = sign_message(&response.nonce.clone().unwrap(), &wallet).unwrap();
    let response = app
        .post_login(
            &wallet.pubkey().to_string(),
            &signed_message,
            &response.nonce.unwrap(),
        )
        .await;

    assert_eq!(
        response.wallet_address.unwrap(),
        wallet.pubkey().to_string()
    );
    let response = app.get_user(wallet.pubkey().to_string()).await;
    assert_eq!(
        response.wallet_address.unwrap(),
        wallet.pubkey().to_string()
    );
}
