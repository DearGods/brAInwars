use crate::test_app::base::spawn_app;
use backend_lib::blockchain::solana::helpers::sign_message;
use backend_lib::routes::basic_response::ResponseStatus;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn create_user_and_login_and_logout() {
    let app = spawn_app().await;
    let wallet = Keypair::new();
    let protected = app.get_protected().await;
    assert_eq!(false, protected.status().is_success());
    assert!(protected.status().is_redirection());
    let response = app.create_user(&wallet).await;
    assert_eq!(response.status, ResponseStatus::Success);
    assert_eq!(
        response.wallet_address.unwrap(),
        wallet.pubkey().to_string(),
    );
    let protected = app.get_protected().await;
    assert_eq!(false, protected.status().is_success());
    assert!(protected.status().is_redirection());
    let signed_message = sign_message(&response.nonce.clone().unwrap(), &wallet).unwrap();
    let response = app
        .post_login(
            &wallet.pubkey().to_string(),
            &signed_message,
            &response.nonce.unwrap(),
        )
        .await;
    assert_eq!(response.status, ResponseStatus::Success);
    assert_eq!(
        response.wallet_address.unwrap(),
        wallet.pubkey().to_string()
    );
    let protected = app.get_protected().await;
    assert!(protected.status().is_success());
    let response = app.post_logout(&wallet.pubkey().to_string()).await;
    assert_eq!(response.status, ResponseStatus::Success);
    let protected = app.get_protected().await;
    assert!(protected.status().is_redirection());
}

#[tokio::test]
async fn create_user_and_login_via_shortcut() {
    let app = spawn_app().await;
    let wallet = Keypair::new();
    let protected = app.get_protected().await;
    assert!(protected.status().is_redirection());
    app.get_logged_in_user(&wallet).await;
    let protected = app.get_protected().await;
    assert!(protected.status().is_success());
}
