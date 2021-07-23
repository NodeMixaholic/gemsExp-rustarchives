fn main() {
    let mut testvar = 0;
    testvar = 100;
    println!("Welcome to gemsExp RuShell test!
    This is a brand new rewrite of GEMS NT
    from the ground up, but this time in RUST!
    ");
    while true {
        let mut cmd = String::new();
        println!("RUSHELL>");
        let b1 = std::io::stdin().read_line(&mut cmd).unwrap();
        println!("{}", cmd);
    }
    println!("PANIC:
    BROKE OUTSIDE OF RUSHELL!")
}
