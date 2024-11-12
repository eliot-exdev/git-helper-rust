use clap::Parser;
use clap::ValueEnum;
use git::read_git_module;
use std::process::exit;

mod git;

#[derive(ValueEnum, Clone)]
enum Commands {
    LIST,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to git repo
    #[arg(short = 'p', long = "path", default_value = "./")]
    path: String,
    /// The command to execute
    #[arg(short = 'c', long = "command")]
    #[clap(value_enum, default_value_t=Commands::LIST)]
    command: Commands,
    /// Filter branches
    #[arg(short = 'f', long = "filter", default_value = "")]
    filter: String,
    /// Crawl all submodules
    #[arg(short = 'r', long = "recursive", default_value_t = false)]
    recursiv: bool,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::LIST => {
            let git_module = read_git_module(String::from("root"), args.path, args.recursiv);
            git_module.print(&args.filter);
        }
    }
    exit(0);
}
