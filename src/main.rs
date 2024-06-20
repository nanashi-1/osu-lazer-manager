use clap::{Parser, Subcommand};
use colored::Colorize;
use get_default::get_default;
use install::install;
use list::list;
use run::run;
use set_default::set_default;
use uninstall::uninstall;
use update::update;

mod get_default;
mod install;
mod list;
mod run;
mod set_default;
mod uninstall;
mod update;

#[derive(Parser)]
#[command(version = "1", about = "Installs and Manages osu!lazer")]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[command(about = "Install osu!lazer")]
    Install {
        #[clap(short, long, help = "Replace existing files from last install if any")]
        force: bool,

        #[clap(short, long, help = "Set the target version to install", default_value_t = String::from("latest"))]
        version: String,

        #[clap(short, long, help = "Make the target version the default version")]
        make_default_version: bool,
    },

    #[command(about = "Run osu!lazer")]
    Run {
        #[clap(short, long, help = "Set the target version to run", default_value_t = String::from("default"))]
        version: String,
    },

    #[command(about = "Update osu!lazer")]
    Update {
        #[clap(
            short,
            long,
            help = "Do not make the target version the default version"
        )]
        do_not_make_default_version: bool,
        #[clap(short, long, help = "Force install the latest version")]
        force: bool,
    },

    #[command(about = "List all installed version")]
    List,

    #[command(about = "Set the default version")]
    SetDefault {
        #[arg(help = "Set the target version to install")]
        version: String,
    },

    #[command(about = "Get the default version")]
    GetDefault,

    #[command(about = "Uninstall osu!lazer")]
    Uninstall {
        #[arg(help = "Set the target version to uninstall", default_value_t = String::from("latest"))]
        version: String,
    },
}

fn print_and_exit_if_err(result: anyhow::Result<()>) {
    if let Err(e) = result {
        eprintln!("{}", e.to_string().as_str().red());
        std::process::exit(1);
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Install {
            force,
            version,
            make_default_version,
        } => print_and_exit_if_err(install(force, &version, make_default_version).await),
        SubCommand::Run { version } => print_and_exit_if_err(run(&version)),
        SubCommand::Update {
            do_not_make_default_version,
            force,
        } => print_and_exit_if_err(update(do_not_make_default_version, force).await),
        SubCommand::List => print_and_exit_if_err(list()),
        SubCommand::SetDefault { version } => print_and_exit_if_err(set_default(&version).await),
        SubCommand::GetDefault => print_and_exit_if_err(get_default()),
        SubCommand::Uninstall { version } => print_and_exit_if_err(uninstall(&version).await),
    };
}
