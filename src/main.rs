//Basically just defining "gemsStdio.getCLIInput". Use "println!" for printing to CLI.
mod gemsStdio;
mod gemsCalc;

fn main() {
    let gemsVer = "0.000000001a";
    let mut testvar = 0;
    testvar = 100;
    println!("Welcome to gemsExp RuShell test!
    This is a brand new rewrite of GEMS NT
    from the ground up, but this time in RUST!
    ");
    while true {
        println!("RUSHELL>");
        let cmd = gemsStdio::getCLIInput();
        let mut argz = cmd.split(" ");
        let args: Vec<&str> = argz.collect();
        if (args[0] == "calc") {
            gemsCalc::calc();
        } else if (args[0] == "echo") {
            let echoStr = cmd.replace(args[0], "");
        } else if (args[0] == "help") {
            println!("echo - echos stuff
            calc - a simple calculator");
        } else if (args[0] == "about") {
            println!("GEMS Exp v{}", gemsVer)
        } else {
            println!("Command not found! (Perhaps try \"help\" or \"about\"?)");
        }
    }
    println!("PANIC:
    BROKE OUTSIDE OF RUSHELL!")
}
