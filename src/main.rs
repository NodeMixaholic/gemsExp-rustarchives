//Basically just defining "gemsStdio.getCLIInput". Use "println!" for printing to CLI.
mod gemsStdio;
mod gemsCalc;
use vfs::{VfsPath, VfsError, MemoryFS};

fn main() {
    let gemsVer = "0.000000001a";
    let mut testvar = 0;
    testvar = 100;
    println!("Welcome to gemsExp RuShell test!
    This is a brand new rewrite of GEMS NT
    from the ground up, but this time in RUST!
    ");
    let root: VfsPath = MemoryFS::new().into();
    while true {
        println!("RUSHELL>");
        let cmd = gemsStdio::getCLIInput();
        let mut argz = cmd.split(" ");
        let args: Vec<&str> = argz.collect();
        if (args[0] == "calc") {
            gemsCalc::calc();
        //Wait until GitHub user manuel-woelker fixes rust-vfs for my use cases.
        /*
        } else if (args[0] == "echo") {
            let echoStr = cmd.replace(args[0], "");
        } else if (args[0] == "2file") {
            let contents = gemsStdio::getCLIInput();
            let f = root.join(args[1]);
            f.write_all(b"{}", contents) || f.create_file(args[1]).write_all(b"{}", contents);
        } else if (args[0] == "dog") {
            let mut content = String::new();
            let f = root.join(args[1]);
            f.open_file(args[1]).read_to_string(&mut content);
            println!("${}", content);
        */
        } else if (args[0] == "help") {
            println!("echo - echos stuff
            calc - a simple calculator
            dog - like cat
            del - deletes stuff
            2file - put text in a file
            ");
        } else if (args[0] == "about") {
            println!("GEMS Exp v{}", gemsVer)
        } else {
            println!("Command not found! (Perhaps try \"help\" or \"about\"?)");
        }
    }
    println!("PANIC:
    BROKE OUTSIDE OF RUSHELL!")
}

