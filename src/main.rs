

mod args;
mod commands;
mod status;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project = args::collect();

    match project.type_gen.as_deref().unwrap_or_default() {
        "init" => commands::init::download_template().await?,
        "version" => println!(
            "{} v{} by {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        ),
        _ => {
            commands::helper::help();
        }
    }
    Ok(())
}
