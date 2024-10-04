use crate::test_app::base::TestApp;
use anchor_lang::solana_program::system_instruction;
use backend_lib::blockchain::solana::helpers::{
    get_recent_blockhash, send_and_confirm_transaction,
};
use backend_lib::configuration::database_settings::DatabaseSettings;
use backend_lib::database::config::update_config_value::update_config_value;
use backend_lib::startup::get_connection_pool::get_connection_pool;
use solana_program_test::{BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::account::Account;
use solana_sdk::clock::Clock;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::ops::DerefMut;
use std::sync::Arc;
use std::time;
use tokio::sync::Mutex;
use tokio::time::sleep;

// https://github.com/metaplex-foundation/metaplex-program-library/blob/aaa27392f217dc3451f99a68213f04c39d29d226/core/rust/testing-utils/src/utils/mod.rs#L19

pub fn brain_wars_program_test() -> ProgramTest {
    let mut program = ProgramTest::default();
    program.add_program("brain_wars", brain_wars::id(), None);
    program.set_compute_max_units(i64::MAX as u64);
    program
}

impl TestApp {
    pub async fn get_balance(&mut self, pubkey: &Pubkey) -> u64 {
        let mut context = self.context.lock().await;
        let context = context.deref_mut();
        context
            .banks_client
            .get_balance(*pubkey)
            .await
            .expect("account not found")
    }

    pub async fn get_account(&mut self, pubkey: &Pubkey) -> Account {
        let mut context = self.context.lock().await;
        let context = context.deref_mut();
        context
            .banks_client
            .get_account(*pubkey)
            .await
            .expect("account not found")
            .expect("account empty")
    }

    pub async fn airdrop(
        &mut self,
        receiver: &Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let context = self.context.lock().await;
        let recent_blockhash = get_recent_blockhash(&self.app_state.client.clone())
            .await
            .unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &context.payer.pubkey(),
                receiver,
                amount,
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            recent_blockhash,
        );
        send_and_confirm_transaction(&self.app_state.client.clone(), &tx)
            .await
            .unwrap();
        Ok(())
    }
}

pub async fn configure_database(settings: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&settings.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, settings.name).as_str())
        .await
        .expect("Failed to create database");

    let db_pool = get_connection_pool(&settings, None)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");

    db_pool
}

pub async fn move_clock(context: Arc<Mutex<ProgramTestContext>>) -> anyhow::Result<()> {
    let one_sec = time::Duration::from_millis(1_000);
    loop {
        let mut context = context.lock().await;
        let context = context.deref_mut();
        let clock = context.banks_client.get_sysvar::<Clock>().await?;
        context.warp_to_slot(clock.slot + 1000)?;
        sleep(one_sec).await;
    }
}

pub async fn set_test_config_values(db_pool: &PgPool) {
    let mut transaction = db_pool.begin().await.unwrap();
    update_config_value(
        &mut transaction,
        "eth_rpc",
        r#"{ "eth_rpc": "http://127.0.0.1:7654" }"#,
    )
    .await
    .unwrap();
    update_config_value(
        &mut transaction,
        "worker_interval",
        r#"{ "worker_interval": 1000 }"#,
    )
    .await
    .unwrap();
    update_config_value(
        &mut transaction,
        "min_game_length",
        r#"{ "min_game_length": 3 }"#,
    )
    .await
    .unwrap();
    update_config_value(
        &mut transaction,
        "max_game_length",
        r#"{ "max_game_length": 5 }"#,
    )
    .await
    .unwrap();
    update_config_value(
        &mut transaction,
        "countdown_for_others_to_join",
        r#"{ "countdown_for_others_to_join": 3 }"#,
    )
    .await
    .unwrap();
    update_config_value(
        &mut transaction,
        "game_entries",
        r#"{ "game_entries": [1000000000] }"#,
    )
    .await
    .unwrap();

    transaction.commit().await.unwrap()
}
