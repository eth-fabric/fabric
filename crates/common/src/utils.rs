use eyre::Result;
use tokio::{
    select,
    signal::unix::{SignalKind, signal},
};

#[cfg(unix)]
pub async fn wait_for_signal() -> Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    select! {
        _ = sigint.recv() => {}
        _ = sigterm.recv() => {}
    }

    Ok(())
}

#[cfg(windows)]
pub async fn wait_for_signal() -> eyre::Result<()> {
    signal::ctrl_c().await?;
    Ok(())
}
