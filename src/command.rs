use std::convert::TryInto;

pub struct Command {
    pub name: String,
    pub help: String,
    pub details: String,
    pub argslen: Vec<i32>,   
    pub argserror: String,      // [num], [min, max], [] <- no limit
    pub run: fn(Vec<String>)
}

pub fn mount(commands: Vec<Command>, args: Vec<String>, error: String) {
    if args.len() > 1 {
        let mut found = false;
        for command in commands.iter() {
            if command.name == args[1] {
                if command.argslen.len() == 0 {
                    let fncommand = command.run;
                    fncommand((&args[2..]).to_vec());
                }
                else if command.argslen.len() == 1 && args.len() == (command.argslen[0] + 2).try_into().unwrap() {
                    let fncommand = command.run;
                    fncommand((&args[2..]).to_vec());
                }
                else if args.len() >= (command.argslen[0] + 2).try_into().unwrap() && args.len() <= (command.argslen[1] + 2).try_into().unwrap() {
                    let fncommand = command.run;
                    fncommand((&args[2..]).to_vec());
                }
                else {
                    println!("{}", command.argserror);
                }
                found = true;
            }
        }
        if !found {
            println!("{}", error);
        }
    }
}
