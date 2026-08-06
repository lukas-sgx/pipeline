use crate::schema::download_template;

mod args;
mod schema;
mod status;
mod variables;
mod help;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project = args::collect();

    match project.type_gen.as_deref().unwrap_or_default() {
        "init" => download_template().await?,
        "version" => println!("pipeline-gen v{}", env!("CARGO_PKG_VERSION")),
        _ => {
            help::help();
        }
    }
    Ok(())
}
