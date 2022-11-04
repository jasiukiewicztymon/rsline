pub struct decoration {
    reset: String,
    bold: String,
    underline: String,
    inverse: String
}
pub struct color {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String
}
pub struct background {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String
}

pub fn styledprint(content: &str, css: &str, reset: bool) {
    let Color = color {
        black: "\x1b[30m".to_string(),
        red: "\x1b[31m".to_string(),
        green: "\x1b[32m".to_string(),
        yellow: "\x1b[33m".to_string(),
        blue: "\x1b[34m".to_string(),
        magenta: "\x1b[35m".to_string(),
        cyan: "\x1b[36m".to_string(),
        white: "\x1b[37m".to_string()
    };
    let Decoration = decoration {
        reset: "\x1b[0m".to_string(),
        bold: "\x1b[1m".to_string(),
        underline: "\x1b[4m".to_string(),
        inverse: "\x1b[7m".to_string()
    }
    let Background = background {
        black: "\x1b[40m".to_string(),
        red: "\x1b[41m".to_string(),
        green: "\x1b[42m".to_string(),
        yellow: "\x1b[43m".to_string(),
        blue: "\x1b[44m".to_string(),
        magenta: "\x1b[45m".to_string(),
        cyan: "\x1b[46m".to_string(),
        white: "\x1b[47m".to_string()
    }

    let mut output: String = content.to_string();

    let cssprops: Vec<&str> = css.split(";").collect();
    for cssprop in cssprops {
        let csspropstyle = cssprop.split(":").collect::<Vec<&str>>().to_vec();
    }

    println!("{}", output)
}
