use crate::test_app::base::spawn_app;
use anchor_lang::AccountDeserialize;
use anchor_spl::token::spl_token;
use backend_lib::blockchain::solana::helpers::sign_message;
use backend_lib::blockchain::solana::pdas::get_player_profile_address;
use backend_lib::database::game::get_game::get_game;
use backend_lib::domain::game::game_status::GameStatus;
use backend_lib::routes::basic_response::ResponseStatus;
use brain_wars::state::player_profile::PlayerProfile;
use brain_wars::StartGameStatusContextBumps;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::Arc;
use std::time;

#[tokio::test]
async fn create_game() {
    let mut app = spawn_app().await;
    assert!(app
        .airdrop(&app.keypair.pubkey(), 10 * LAMPORTS_PER_SOL)
        .await
        .is_ok());
    assert!(app
        .set_crank(&app.keypair.clone(), &app.keypair.clone().pubkey(), 100)
        .await
        .is_ok());
    let players = vec![
        Arc::new(Keypair::new()),
        Arc::new(Keypair::new()),
        Arc::new(Keypair::new()),
    ];
    let one_sec = time::Duration::from_millis(1_000);
    tokio::time::sleep(one_sec).await;
    let games_response = app.get_games().await;
    assert_eq!(games_response.games.len(), 1);
    let game_uuid = games_response.games[0].id.as_uuid();
    let mut players_nonces: Vec<String> = Vec::new();
    for player in players.iter() {
        app.airdrop(&player.pubkey(), 10 * LAMPORTS_PER_SOL)
            .await
            .unwrap();
        let response = app.create_user(&player).await;
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(
            response.wallet_address.unwrap(),
            player.pubkey().to_string()
        );
        players_nonces.push(response.nonce.unwrap());
        assert!(app
            .deposit(&player, &spl_token::native_mint::id(), 5 * LAMPORTS_PER_SOL)
            .await
            .is_ok());
        let account = app
            .get_account(&get_player_profile_address(&player.pubkey()).0)
            .await;
        let player_profile: PlayerProfile =
            PlayerProfile::try_deserialize(&mut account.data.as_slice()).unwrap();
        assert!(account.lamports > 5 * LAMPORTS_PER_SOL);
        assert_eq!(player_profile.player, player.pubkey());
    }
    let wallet = app.keypair.clone();
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
    assert_eq!(response.status, ResponseStatus::Success);
    assert_eq!(
        response.wallet_address.unwrap(),
        wallet.pubkey().to_string()
    );
    for (index, player) in players.iter().enumerate() {
        let nonce = &players_nonces[index];
        let signed_message = sign_message(nonce, &player).unwrap();
        let response = app
            .post_login(&player.pubkey().to_string(), &signed_message, &nonce)
            .await;
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(
            response.wallet_address.unwrap(),
            player.pubkey().to_string()
        );
        let user_response = app.get_user(player.pubkey().to_string()).await;
        assert_eq!(user_response.status, ResponseStatus::Success);
        assert_eq!(
            user_response.wallet_address.unwrap(),
            player.pubkey().to_string()
        );
        let response = app.join_game(&game_uuid).await;
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(response.game_id.unwrap(), game_uuid);
        assert_eq!(response.user_id.unwrap(), user_response.id.unwrap());
    }
    let game = app.get_game(game_uuid).await;
    assert_eq!(game.id.as_uuid(), game_uuid);
    assert_eq!(game.mint, spl_token::native_mint::id().to_string());
    assert_eq!(game.game_status, GameStatus::WaitingForPlayers);
    assert_eq!(game.reveled_limit, None);
    let mut count = 0;
    while count < 10 {
        tokio::time::sleep(one_sec).await;
        let game = app.get_game(game_uuid).await;
        if game.game_status == GameStatus::OnGoing {
            break;
        }
        count += 1;
    }

    let game = app.get_game(game_uuid).await;
    assert_eq!(game.id.as_uuid(), game_uuid);
    assert_eq!(game.mint, spl_token::native_mint::id().to_string());
    assert_eq!(game.game_status, GameStatus::OnGoing);
    assert_eq!(game.reveled_limit, None);

    let player = players[0].clone();
    let nonce = &players_nonces[0];
    let signed_message = sign_message(nonce, &player).unwrap();
    let response = app
        .post_login(&player.pubkey().to_string(), &signed_message, &nonce)
        .await;
    assert_eq!(response.status, ResponseStatus::Success);
    assert_eq!(
        response.wallet_address.unwrap(),
        player.pubkey().to_string()
    );
    app.leave_game(&game_uuid).await;

    let mut transaction = app.db_pool.begin().await.unwrap();
    let db_game = get_game(&mut transaction, game_uuid).await.unwrap();
    transaction.commit().await.unwrap();

    let diff = db_game.end_time.unwrap().get() - db_game.start_time.unwrap().get();
    tokio::time::sleep(time::Duration::from_secs(diff)).await;

    let mut count = 0;
    while count < 10 {
        tokio::time::sleep(one_sec).await;
        let game = app.get_game(game_uuid).await;
        if game.game_status == GameStatus::Settled {
            break;
        }
        count += 1;
    }

    let game = app.get_game(game_uuid).await;
    assert_eq!(game.id.as_uuid(), game_uuid);
    assert_eq!(game.mint, spl_token::native_mint::id().to_string());
    let mut transaction = app.db_pool.begin().await.unwrap();
    let db_game = get_game(&mut transaction, game_uuid).await.unwrap();
    transaction.commit().await.unwrap();
    assert_ne!(game.reveled_limit, None);
    assert_eq!(game.winner, Some(player.pubkey().to_string()));
    assert_eq!(db_game.winner, Some(player.pubkey().to_string()));
    assert_eq!(game.game_status, GameStatus::Settled);
}
