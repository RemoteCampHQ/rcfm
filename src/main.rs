use std::process::Command as Co;
use clap::{arg, Command};

fn fetch() {
    Co::new("mkdir")
        .arg("-p")
        .arg("~/.kfc/packages")
        .spawn()
        .expect("mkdir ~/.kfc/packages fail");

    println!("<======= mkdir ~/.kfc/packages success =======>");

    Co::new("cd")
        .arg("~/.kfc/packages")
        .spawn()
        .expect("<======= cd ~/.kfc/packages fail =======>");

    println!("<======= cd ~/.kfc/packages success =======>");

    let url = "https://nodejs.org/dist/v16.17.0/node-v16.17.0.tar.xz";

    Co::new("curl")
        .arg("-O")
        .arg(url)
        .spawn()
        .expect("curl command failed to start");

    println!("<======= curl -O https://nodejs.org/dist/v16.17.0/node-v16.17.0.tar.xz success =======>");
}

fn cli<'rcfm>() -> Command<'rcfm> {
    Command::new("rcfm")
        .about("A fictional versioning CLI")
        .subcommand(
            Command::new("install")
                .name("install")
                .about("pushes things")
                .alias("i")
                .arg(arg!(<Package> "The remote to Package"))
                .arg_required_else_help(true),
        )

}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let arg = sub_matches.get_one::<String>("");
            fetch();
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(),
    }
}