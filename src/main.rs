use std::env;

use clap::Parser;
use ignore::Walk;
use log::debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to scan dir
    #[arg(value_name = "DIR")]
    path: Option<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let path = if let Some(p) = args.path {
        p
    } else {
        debug!("chemin non passé en paramétre");
        env::current_dir()
            .unwrap()
            .to_str()
            .expect("Pas de chemin valide trouvé pour le répertoire courant")
            .to_string()
    };

    debug!("chemin à chercher {}", path);
    for r in Walk::new(path) {
        match r {
            Ok(entry) => {
                if entry.clone().path().is_dir() {
                    println!("{}", entry.path().display());
                }
            }
            Err(err) => debug!("{}", err),
        }
    }
}
