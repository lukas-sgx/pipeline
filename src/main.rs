mod args;
mod commands;
mod models;
mod status;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project = args::collect();

    match project.type_gen.as_deref().unwrap_or_default() {
        "init" | "--init" => commands::init::download_template().await?,
        "version" | "--version" => commands::version::actual(),
        _ => {
            commands::helper::help();
        }
    }
    Ok(())
}
