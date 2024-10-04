#![forbid(unsafe_code)]

use backend_lib::database::migrate::migrate;
use backend_lib::database::user::create_user::create_user;
use backend_lib::database::user::get_user_opt_by_wallet::get_user_opt_by_wallet;
use backend_lib::errors::error::Error;
use backend_lib::workers::tasks::process_jobs::job_loop;

#[cfg(feature = "my-test")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(not(feature = "my-test"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use backend_lib::blockchain::solana::helpers::{get_client, get_keypair};
    use backend_lib::configuration::get_configuration::get_configuration;
    use backend_lib::domain::secret::Secret;
    use backend_lib::envars::app_env_var::AppEnvVar;
    use backend_lib::envars::env_var::EnvVar;
    use backend_lib::envars::get_env_var_or_panic::get_env_var_or_panic;
    use backend_lib::envars::load_dotenv::load_dotenv;
    use backend_lib::startup::application::{AppState, Application};
    use backend_lib::startup::get_connection_pool::get_connection_pool;
    use backend_lib::startup::report_exit::report_exit;
    use backend_lib::telemetry::subscriber::{get_subscriber, init_subscriber};
    use backend_lib::workers::game_worker::worker_loop;
    use solana_sdk::signature::Signer;
    use std::sync::Arc;
    use tokio::sync::broadcast;

    load_dotenv();
    let configuration = get_configuration().expect("Failed to read configuration");
    let subscriber = get_subscriber(
        "brain-wars",
        if <EnvVar as AsRef<String>>::as_ref(&get_env_var_or_panic(AppEnvVar::AppEnvironment))
            == "local"
        {
            "info"
        } else {
            "debug"
        },
        std::io::stdout,
        false,
    );
    init_subscriber(subscriber);
    tracing::info!("Starting with configuration {:#?}", configuration);
    let keypair = Arc::new(get_keypair(None)?);
    tracing::error!("wallet address {}", keypair.pubkey().to_string());
    let client = get_client();
    let database_url = get_env_var_or_panic(AppEnvVar::DatabaseUrl);
    let database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);
    let db_pool = get_connection_pool(&configuration.database, Option::from(database_url)).await?;
    migrate(&db_pool).await?;
    let (game_tx, _rx) = broadcast::channel(100);
    let (chat_tx, _chat_rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        game_tx,
        chat_tx,
        client: Arc::new(client),
        keypair,
        pool: db_pool.clone(),
    });
    let mut transaction = db_pool.begin().await.map_err(Error::from)?;
    let crank_user =
        get_user_opt_by_wallet(&mut transaction, &app_state.keypair.pubkey().to_string())
            .await
            .map_err(Error::from)?;
    if crank_user.is_none() {
        create_user(
            &mut transaction,
            &app_state.keypair.pubkey().to_string(),
            "The House",
        )
        .await
        .map_err(Error::from)?;
    }
    transaction.commit().await.map_err(Error::from)?;
    let worker_task = tokio::spawn(worker_loop(db_pool.clone(), app_state.clone()));
    let job_task = tokio::spawn(job_loop(db_pool.clone(), app_state.clone()));
    let application = Application::build(configuration, app_state, db_pool).await;
    let application_task = tokio::spawn(application.run());
    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task =>  report_exit("Background worker", o),
        o = job_task => report_exit("Job worker", o),
    };
    Ok(())
}
