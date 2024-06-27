use std::env;
use wyag::{command, Repository};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No command specified!");
    }

    let command = &args[1];
    let _ = match &command[..] {
        "add" =>            { command::add() },
        "cat-file" =>       { command::cat_file() },
        "check-ignore" =>   { command::check_ignore() },
        "checkout" =>       { command::checkout() },
        "commit" =>         { command::commit() },
        "hash-object" =>    { command::hash_object() },
        "init" =>           { command::init() },
        "log" =>            { command::log() },
        "ls-files" =>       { command::ls_files() },
        "ls-tree" =>        { command::ls_tree() },
        "rev-parse" =>      { command::rev_parse() },
        "rm" =>             { command::rm() },
        "show-ref" =>       { command::show_ref() },
        "status" =>         { command::status() },
        "tag" =>            { command::tag() },

        other =>      { panic!("{} is not a valid git command!", other)}
    };

    let r = Repository::new(env::current_dir().unwrap());
}
