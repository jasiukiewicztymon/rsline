pub struct Command {
    pub name: String,
    pub help: String,
    pub details: String,
    pub run: fn(Vec<String>)
}

pub fn mount(commands: Vec<Command>, args: Vec<String>) {
    if args[2] == "help" {

    } else {
        for command in commands.iter() {
            if command.name == args[2] {
                let fncommand = command.run;
                
                fncommand((&args[3..]).to_vec());
            }
        }
    }
}
