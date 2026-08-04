use std::env;

pub struct Arg {
    pub name: Option<String>,
    pub repo: Option<String>,
    pub type_gen: Option<String>,
}

pub fn collect() -> Arg {
    let mut gen_arg = Arg {
        name: None,
        repo: None,
        type_gen: None,
    };
    let mut iter = env::args().peekable();
    iter.next();

    let arg = iter.next();

    match arg.as_deref().unwrap_or_default() {
        "init" => gen_arg.type_gen = arg,
        "create" => gen_arg.type_gen = arg,
        _ => {}
    }

    loop {
        let arg = iter.next();

        match arg.as_deref().unwrap_or_default() {
            "--project" | "-p" => gen_arg.name = iter.next(),
            "--repo" | "-r" => gen_arg.repo = iter.next(),
            _ => {}
        }

        if arg.is_none() {
            break;
        }
    }
    gen_arg
}
