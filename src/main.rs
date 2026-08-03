mod args;

pub fn download_template() {
}

fn main() {
    let project = args::collect();
    
    if project.type_gen.is_none() {
        // init();
    } else {
        if project.type_gen.unwrap_or_default() == "init" {
            download_template();
        }
    }
}
