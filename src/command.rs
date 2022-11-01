pub struct Command {
    pub name: String,
    pub help: String,
    pub details: String,
    pub run: fn(Vec<String>)
}

pub fn mount(commands: Vec<Command>, args: Vec<String>) {
    for command in commands.iter() {
        println!("{}", command.name)
    }
}
