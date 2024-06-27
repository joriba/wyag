use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No command specified!");
    }

    let command = &args[1];
    let result = match &command[..] {
        "add" =>            { cmd_add() },
        "cat-file" =>       { cmd_cat_file() },
        "check-ignore" =>   { cmd_check_ignore() },
        "checkout" =>       { cmd_checkout() },
        "commit" =>         { cmd_commit() },
        "hash-object" =>    { cmd_hash_object() },
        "init" =>           { cmd_init() },
        "log" =>            { cmd_log() },
        "ls-files" =>       { cmd_ls_files() },
        "ls-tree" =>        { cmd_ls_tree() },
        "rev-parse" =>      { cmd_rev_parse() },
        "rm" =>             { cmd_rm() },
        "show-ref" =>       { cmd_show_ref() },
        "status" =>         { cmd_status() },
        "tag" =>            { cmd_tag() },
        other => { panic!("{} is not a valid git command!", other)}
    };
}

fn cmd_add() -> Result<(), io::Error> {
    todo!()
}

fn cmd_cat_file() -> Result<(), io::Error> {
    todo!()
}

fn cmd_check_ignore() -> Result<(), io::Error> {
    todo!()
}

fn cmd_checkout() -> Result<(), io::Error> {
    todo!()
}

fn cmd_commit() -> Result<(), io::Error> {
    todo!()
}

fn cmd_hash_object() -> Result<(), io::Error> {
    todo!()
}

fn cmd_init() -> Result<(), io::Error> {
    todo!()
}

fn cmd_log() -> Result<(), io::Error> {
    todo!()
}

fn cmd_ls_files() -> Result<(), io::Error> {
    todo!()
}

fn cmd_ls_tree() -> Result<(), io::Error> {
    todo!()
}

fn cmd_rev_parse() -> Result<(), io::Error> {
    todo!()
}

fn cmd_rm() -> Result<(), io::Error> {
    todo!()
}

fn cmd_show_ref() -> Result<(), io::Error> {
    todo!()
}

fn cmd_status() -> Result<(), io::Error> {
    todo!()
}

fn cmd_tag() -> Result<(), io::Error> {
    todo!()
}