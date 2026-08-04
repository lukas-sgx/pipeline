use crate::schema::download_template;

mod args;
mod schema;
mod variables;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project = args::collect();
    
    if project.type_gen.is_none() {
        // init();
    } else {
        if project.type_gen.unwrap_or_default() == "init" {
            download_template().await?;
        }
    }
    Ok(())
}
