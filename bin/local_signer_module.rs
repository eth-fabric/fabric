use cb_common::{
    config::{LogsSettings, StartSignerConfig},
    utils::{initialize_tracing_log, wait_for_signal},
};
use cb_signer::service::SigningService;
use eyre::Result;
use tracing::{error, info};

pub const SIGNER_MODULE_NAME: &str = "signer";

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = initialize_tracing_log(SIGNER_MODULE_NAME, LogsSettings::from_env_config()?);

    let config = StartSignerConfig::load_from_env()?;
    let server = SigningService::run(config);

    tokio::select! {
        maybe_err = server => {
            if let Err(err) = maybe_err {
                error!(%err, "signing server unexpectedly stopped");
                eprintln!("signing server unexpectedly stopped: {err}");
            }
        },
        _ = wait_for_signal() => {
            info!("shutting down");
        }
    }

    Ok(())
}
