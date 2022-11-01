use std::env;

struct Command {
    name: String,
    help: String,
    details: String
}
impl Command {
    fn run(args: &[&str], func: &dyn Fn(&[&str])) {
        func(&args);
    }
}

fn main() {
    let commands = [
        Command {
            name: String::from("get"),
            help: String::from("get <url> <selector>"),
            details: String::from("...")
        },
        Command {
            name: String::from("observe"),
            help: String::from("observe <url> <selector> <timeout>"),
            details: String::from("...")
        }
    ];

    println!("{}", commands[1].name);

    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
