use crate::schema::download_template;

mod args;
mod schema;
mod status;
mod variables;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project = args::collect();

    match project.type_gen.as_deref().unwrap_or_default() {
        "init" => download_template().await?,
        _ => {
            download_template().await?;
        }
    }
    Ok(())
}
