use crate::test_app::base::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(2));
    assert_eq!(response.text().await.unwrap(), "OK");
}
