use clap::{command, Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about = "Added. For new features")]
    Add {
        #[arg(short, long, help = "Small description of the change")]
        text: String,

        #[arg(short, long, help = "Release version")]
        version: String,
    },

    #[clap(about = "Changed. For changes in existing functionality")]
    Change,

    #[clap(about = "Deprecated. For soon-to-be removed features")]
    Deprecate,

    #[clap(about = "Removed. For now removed features")]
    Remove,

    #[clap(about = "Fixed. For any bug fixes")]
    Fix,

    #[clap(about = "Security. In case of vulnerabilities")]
    Security,

    #[clap(about = "Generates simple template")]
    Create {
        #[arg(short, long, help = "Small description of the change")]
        path: Option<String>,
    },

    #[clap(about = "Yanked. Pulled because of a serious bug or security issue")]
    Yank,

    #[clap(about = "Unrelease. Pretty much self explanatory")]
    Unrelease,
}

#[derive(Parser, Debug)]
#[command(
    name = "keepchangelog",
    author = "Pedro Luz <pedromsluz@gmail.com>",
    version,
    about = "This is a wrapper around keepachangelog.com to allow to make changes to the CHANGELOG.md on the fly.",
    long_about = None,
)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

pub fn cli() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Add { text, version } => dbg!(text, version),
        Commands::Change => todo!(),
        Commands::Deprecate => todo!(),
        Commands::Remove => todo!(),
        Commands::Fix => todo!(),
        Commands::Security => todo!(),
        Commands::Create { path } => match keepchangelog::create(path.as_deref()) {
            Ok(_) => {
                println!("CHANGELOG.md created.");
                ("".to_owned(), "".to_owned())
            }
            Err(err) => {
                println!("UPSSS");
                ("ERROR: ".to_owned(), err.to_string())
            }
        },
        Commands::Yank => todo!(),
        Commands::Unrelease => todo!(),
    };
}
