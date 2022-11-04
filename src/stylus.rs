pub fn styleprint(content: &str, css: &str) {
    let prop: Vec<&str> = css.split(";").collect();
    println!("{:?}", prop);
}
