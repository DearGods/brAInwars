use crate::blockchain::solana::pdas::{get_game_address, get_players_actions_address};
#[cfg(not(feature = "my-test"))]
use crate::blockchain::solana::pdas::{
    get_player_profile_address, get_player_profile_token_address,
};
use crate::envars::app_env_var::AppEnvVar;
use crate::envars::env_var::EnvVar;
use crate::envars::get_env_var_or_panic::get_env_var_or_panic;
use crate::startup::application::AppState;
use anchor_client::{Client, Cluster};
use anchor_lang::solana_program::message::Message;
use anchor_lang::AccountDeserialize;
#[cfg(not(feature = "my-test"))]
use anchor_spl::token::spl_token;
use anyhow::anyhow;
use brain_wars::state::game::Game as OnChainGame;
use brain_wars::state::players_actions::PlayersActions as OnChainPlayerActions;
use solana_client::nonblocking::rpc_client::RpcClient;
#[cfg(feature = "my-test")]
use solana_program_test::BanksClient;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
#[cfg(feature = "my-test")]
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair, Signature, Signer};
use solana_sdk::transaction::Transaction;
#[cfg(feature = "my-test")]
use solana_sdk::transaction::VersionedTransaction;
#[cfg(feature = "my-test")]
use std::ops::DerefMut;
use std::str;
use std::str::FromStr;
use std::sync::Arc;
#[cfg(feature = "my-test")]
use std::time::Duration;
#[cfg(feature = "my-test")]
use tokio::{sync::Mutex, time::sleep};
use uuid::Uuid;

const DEV_NET_HTTP: &str = "https://api.devnet.solana.com";
// const DEV_NET_HTTP: &str = "https://staging-rpc.dev2.eclipsenetwork.xyz";

const DEV_NET_WSS: &str = "wss://api.devnet.solana.com/";
// const DEV_NET_WSS: &str = "wss://staging-rpc.dev2.eclipsenetwork.xyz";
pub const COMMITMENT: CommitmentConfig = CommitmentConfig::processed();

pub fn get_anchor_client(payer: Option<Arc<Keypair>>) -> Client<Arc<Keypair>> {
    let url = Cluster::Custom(DEV_NET_HTTP.to_string(), DEV_NET_WSS.to_string());
    Client::new_with_options(
        url.clone(),
        match payer {
            Some(payer) => payer.clone(),
            None => Arc::new(Keypair::new()),
        },
        COMMITMENT,
    )
}

pub fn get_client() -> RpcClient {
    let url = DEV_NET_HTTP.to_string();
    RpcClient::new_with_commitment(url.clone(), COMMITMENT)
}

pub fn get_keypair(path: Option<String>) -> anyhow::Result<Keypair> {
    if let Some(path) = path {
        let wallet: Keypair = read_keypair_file::<&str>(path.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
        return Ok(wallet);
    };
    let mnemonic_path = get_env_var_or_panic(AppEnvVar::WalletMnemonic);
    match mnemonic_path {
        EnvVar::Secret(secret) => {
            let wallet: Keypair = read_keypair_file::<&str>(secret.as_ref())
                .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
            Ok(wallet)
        }
        EnvVar::Public(_) => Err(anyhow::anyhow!("WalletMnemonic is not a secret")),
    }
}

pub fn create_transaction(
    instructions: Vec<Instruction>,
    payer: &Pubkey,
    signer: &Keypair,
    latest_blockhash: Hash,
) -> anyhow::Result<Transaction> {
    let signing_keypairs = vec![&signer];
    let mut transaction = Transaction::new_unsigned(Message::new(&instructions, Some(payer)));
    transaction
        .try_sign(&signing_keypairs, latest_blockhash)
        .map_err(|err| anyhow!("error: failed to sign transaction: {}", err))?;
    Ok(transaction)
}

#[cfg(not(feature = "my-test"))]
pub async fn get_account(
    client: &Arc<RpcClient>,
    pubkey: &Pubkey,
) -> anyhow::Result<Option<Account>> {
    let account = client
        .get_account_with_commitment(pubkey, COMMITMENT)
        .await
        .map_err(|e| anyhow!("error: Solana: get_account failed: {}", e))?;
    Ok(account.value)
}

#[cfg(feature = "my-test")]
pub async fn get_account(
    client: &Arc<Mutex<BanksClient>>,
    pubkey: &Pubkey,
) -> anyhow::Result<Option<Account>> {
    let mut client = client.lock().await;
    let client = client.deref_mut();
    client
        .get_account(*pubkey)
        .await
        .map_err(|e| anyhow!("error: get_account failed: {}", e))
}

#[cfg(not(feature = "my-test"))]
pub async fn get_recent_blockhash(client: &Arc<RpcClient>) -> anyhow::Result<Hash> {
    client
        .get_latest_blockhash()
        .await
        .map_err(|e| anyhow!("error: Solana: get_recent_blockhash failed: {}", e))
}

#[cfg(feature = "my-test")]
pub async fn get_recent_blockhash(client: &Arc<Mutex<BanksClient>>) -> anyhow::Result<Hash> {
    let mut client = client.lock().await;
    let client = client.deref_mut();
    client
        .get_latest_blockhash()
        .await
        .map_err(|e| anyhow!("error: get_recent_blockhash failed: {}", e))
}

#[cfg(not(feature = "my-test"))]
pub async fn send_and_confirm_transaction(
    client: &Arc<RpcClient>,
    txn: &Transaction,
) -> anyhow::Result<Signature> {
    match client
        .send_and_confirm_transaction(txn)
        // .send_transaction_with_config(
        //     txn,
        //     RpcSendTransactionConfig {
        //         skip_preflight: true,
        //         preflight_commitment: None,
        //         max_retries: Some(3),
        //         ..RpcSendTransactionConfig::default()
        //     },
        // )
        .await
    {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig).await {
                if confirmed {
                    return Ok(sig);
                }
            }
        },
        Err(e) => {
            let msg = format!("error: send_and_confirm_transaction failed: {}", e);
            tracing::error!("{}", msg);
            Err(anyhow!(msg))
        }
    }
}

#[cfg(feature = "my-test")]
pub async fn send_and_confirm_transaction(
    client: &Arc<Mutex<BanksClient>>,
    txn: &Transaction,
) -> anyhow::Result<Signature> {
    let mut client = client.lock().await;
    let test_client = client.deref_mut();

    let vtx: VersionedTransaction = txn.clone().into();
    let signature = vtx.signatures[0];
    let metadata = test_client
        .process_transaction_with_metadata(vtx)
        .await
        .map_err(|e| anyhow!("error: send_and_confirm_transaction failed: {}", e))?;
    tracing::info!("send_and_confirm_transaction: metadata:  {:?}", metadata);
    let mut status = test_client.get_transaction_status(signature).await?;
    while status.is_none() {
        sleep(Duration::from_millis(100)).await;
        status = test_client.get_transaction_status(signature).await?;
    }
    if status.clone().unwrap().err.is_some() {
        tracing::error!("send_and_confirm_transaction: status:  {:?}", status);
        return Err(anyhow!(
            "error: send_and_confirm_transaction failed: {:?}",
            status.clone().unwrap().err
        ));
    }
    Ok(signature)
}

#[tracing::instrument(
    name = "get_game_from_blockchain",
    level = "trace",
    skip(app_state),
    err
)]
pub async fn get_game_from_blockchain(
    app_state: Arc<AppState>,
    game_id: u64,
    id: Uuid,
) -> anyhow::Result<OnChainGame> {
    let address = get_game_address(game_id).0;
    let account = get_account(&app_state.client, &address).await?;
    if account.is_none() {
        return Err(anyhow!(
            "get_game_from_blockchain: game.id = '{}' game.game_id='{}' account is none '{}'",
            id,
            game_id,
            address.to_string()
        ));
    }
    let on_chain_game: OnChainGame =
        OnChainGame::try_deserialize(&mut account.unwrap().data.as_slice())?;
    Ok(on_chain_game)
}

#[cfg(not(feature = "my-test"))]
#[tracing::instrument(name = "get_player_balance", level = "trace", skip(app_state), err)]
pub async fn get_player_balance(
    app_state: Arc<AppState>,
    player: &Pubkey,
    mint: &Pubkey,
) -> anyhow::Result<u64> {
    if mint.to_string() == spl_token::native_mint::id().to_string() {
        let (player_profile, _) = get_player_profile_address(player);
        let account_data = get_account(&app_state.client, &player_profile).await?;
        match account_data {
            Some(account) => Ok(account.lamports),
            None => Err(anyhow!(
                "get_player_balance: player = '{}' mint='{}' account is none",
                player.to_string(),
                mint.to_string()
            )),
        }
    } else {
        let (player_token_account, _) = get_player_profile_token_address(player, mint);
        let account_data = &app_state
            .client
            .get_token_account_balance(&player_token_account)
            .await;
        match account_data {
            Ok(account_data) => Ok(account_data.ui_amount.unwrap() as u64),
            Err(e) => Err(anyhow!(
                "get_player_balance: player = '{}' mint='{}' account is none: {}",
                player.to_string(),
                mint.to_string(),
                e
            )),
        }
    }
}

#[cfg(feature = "my-test")]
#[tracing::instrument(name = "get_player_balance", level = "trace", skip(_app_state), err)]
pub async fn get_player_balance(
    _app_state: Arc<AppState>,
    _player: &Pubkey,
    _mint: &Pubkey,
) -> anyhow::Result<u64> {
    Ok(LAMPORTS_PER_SOL)
}

#[tracing::instrument(
    name = "get_players_actions_from_blockchain",
    level = "trace",
    skip(app_state),
    err
)]
pub async fn get_players_actions_from_blockchain(
    app_state: Arc<AppState>,
    game_id: u64,
    id: Uuid,
) -> anyhow::Result<OnChainPlayerActions> {
    let address = get_players_actions_address(game_id).0;
    let account = get_account(&app_state.client, &address).await?;
    if account.is_none() {
        return Err(anyhow!(
            "get_game_from_blockchain: game.id = '{}' game.game_id='{}' account is none '{}'",
            id,
            game_id,
            address.to_string()
        ));
    }
    let on_chain_player_action: OnChainPlayerActions =
        OnChainPlayerActions::try_deserialize(&mut account.unwrap().data.as_slice())?;
    Ok(on_chain_player_action)
}

pub fn sign_message(message: &str, keypair: &Keypair) -> anyhow::Result<String> {
    let signature = keypair.try_sign_message(message.as_bytes())?;
    Ok(signature.to_string())
}

pub fn validate_signature(
    message: &str,
    signature: &str,
    public_key: &Pubkey,
) -> anyhow::Result<bool> {
    let signature = Signature::from_str(signature)?;
    Ok(signature.verify(&public_key.to_bytes(), message.as_bytes()))
}

pub static USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_keypair() {
        let keypair = get_keypair(None).unwrap();
        assert_eq!(
            keypair.pubkey().to_string(),
            "J6z5oFcRfpvtsGX2zARjQ5FpE1khXDuyTBtVNF7bArcu"
        );
    }

    #[test]
    pub fn test_sign_message() {
        let keypair = get_keypair(None).unwrap();
        let message = "hello world";
        let signature = sign_message(message, &keypair).unwrap();
        assert!(validate_signature(message, &signature, &keypair.pubkey()).unwrap());
    }
}
