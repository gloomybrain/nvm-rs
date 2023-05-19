use clap::{Parser, Subcommand};

use shared::local::list_local_versions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// install the specified version
    Install,
    /// List all of the node versions cached locally
    Ls,
    /// List all of the available node versions
    LsRemote,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Ls) => {
            list_local_versions()
                .into_iter()
                .for_each(|version| println!("{:?}", version));
        }
        Some(Command::LsRemote) => {
            unimplemented!()
        }
        Some(Command::Install) => {
            unimplemented!()
        }
        None => {
            println!("")
        }
    }
}
