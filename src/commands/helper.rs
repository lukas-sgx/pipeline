use colorized::{Colors, colorize_println};

fn display_section_title(name: &str) {
    colorize_println(name, Colors::GreenFg);
}

fn display_section_minus(content: &str) {
    colorize_println(content, Colors::CyanFg);
}

fn display_usage() {
    let usage_text = format!("{:>4}pipeline-gen [+command] [OPTIONS]...\n", "");

    display_section_title("Usage:");
    display_section_minus(&usage_text);
}

fn display_commands() {
    let cmd_init = format!("    {:<10} {}", "init", "setup repo properly");
    let cmd_update = format!("    {:<10} {}", "update", "upgrade to the latest version");
    let cmd_version = format!("    {:<10} {}", "version", "show version");
    let cmd_help = format!("    {:<10} {}", "help", "show this pannel");

    display_section_title("Commands:");
    display_section_minus(&cmd_init);
    display_section_minus(&cmd_update);
    display_section_minus(&cmd_version);
    display_section_minus(&cmd_help);
}

pub fn help() {
    println!("Setup simply new repo\n");

    display_usage();
    display_commands();
    println!();
}
