use std::env;

mod command;
mod stylus;

fn getcommand(args: Vec<String>) {
    println!("{:?}", &args)
}

fn main() {
    let commands = vec![
        command::Command {
            name: String::from("get"),
            help: String::from("get <url> <selector>"),
            details: String::from("..."),
            argslen: vec![4],
            argserror: String::from("ARG ERROR"),
            run: getcommand
        },
        command::Command {
            name: String::from("observe"),
            help: String::from("observe <url> <selector> <timeout>"),
            details: String::from("..."),
            argslen: vec![],
            argserror: String::from("ARG ERROR"),
            run: getcommand
        }
    ];

    stylus::styledprint("test", "color: red", true);

    let args: Vec<String> = env::args().collect();
    command::mount(commands, args, "error: This command doesn't exists".to_string());
}
