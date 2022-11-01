use std::env;

mod command;

fn getcommand(args: Vec<String>) {

}

fn main() {
    let commands = vec![
        command::Command {
            name: String::from("get"),
            help: String::from("get <url> <selector>"),
            details: String::from("..."),
            run: getcommand
        },
        command::Command {
            name: String::from("observe"),
            help: String::from("observe <url> <selector> <timeout>"),
            details: String::from("..."),
            run: getcommand
        }
    ];

    let args: Vec<String> = env::args().collect();
    command::mount(commands, args);
}
