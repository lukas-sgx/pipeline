use crate::schema::download_template;

mod args;
mod help;
mod schema;
mod status;
mod variables;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project = args::collect();

    match project.type_gen.as_deref().unwrap_or_default() {
        "init" => download_template().await?,
        "version" => println!(
            "{} v{} by {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        ),
        _ => {
            help::help();
        }
    }
    Ok(())
}
