use crate::commands::add::add_handler;
use crate::commands::find::find_handler;
use crate::commands::list::list_handler;
use crate::commands::remove::remove_handler;
use crate::commands::update::update_handler;
use crate::commands::scripts::scripts_handler;
use crate::commands::sync::sync_handler;
use crate::schemas::Package;
use clap::{Parser, Subcommand};
use sea_orm::{ConnectionTrait, Database, Schema};

mod schemas;
mod commands;

#[derive(Parser)]
#[command(
    name = "PacWhy",
    version = "1.0",
    about = "A System Package Organization Manager"
)]
struct CLI {
    #[command(subcommand)]
    commands: Commands,

    #[arg(long = "bypass-root", help = "Run PacWhy as non-root")]
    bypass_root: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(long_flag = "add", short_flag = 'a', long_about = "Add Package")]
    Add {
        #[arg(long = "name")]
        name: String,

        #[arg(long = "description")]
        description: String,

        #[arg(long = "reason")]
        reason: String
    },

    #[command(long_flag = "remove", short_flag = 'r', long_about = "Remove Package")]
    Remove {
        #[arg(long = "name")]
        name: String,
    },

    #[command(long_flag = "update", short_flag = 'u', long_about = "Update Package")]
    Update {
        #[arg(long = "name")]
        name: String,

        #[arg(long = "description", default_value = "")]
        description: String,

        #[arg(long = "reason", default_value = "")]
        reason: String
    },

    #[command(long_flag = "list", short_flag = 'l', long_about = "List Package")]
    List {
        #[arg(long = "parseable", default_value = "false")]
        parseable: String,
    },

    #[command(long_flag = "find", short_flag = 'f', long_about = "Find Package")]
    Find {
        #[arg(long = "name", default_value = "")]
        name: String,

        #[arg(long = "description", default_value = "")]
        description: String,

        #[arg(long = "time", default_value = "", help = "Time the package was installed in ISO 8601")]
        time: String,

        #[arg(long = "parseable", default_value = "false")]
        parseable: String,
    },

    #[command(long_flag = "scripts", long_about = "Execute PacWhy scripts")]
    Scripts {
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            help = "First argument is the script name, the rest are the arguments"
        )]
        args: Vec<String>,
    },

    #[command(long_flag = "sync", short_flag = 's', long_about = "Sync deleted packages")]
    Sync {
    }
}

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let raw_args = std::env::args().collect::<Vec<String>>();
    // Check if user is root
    // Error when user is not root except:
    // - The user just wants the usage message
    // - The user specified --bypass-root
    // - The user wants the version
    if !raw_args.contains(&String::from("--bypass-root")) && !raw_args.contains(&String::from("--help")) && !raw_args.contains(&String::from("--version")) && raw_args.len() != 1 {
        match std::env::var("USER") {
            Ok(val) => if val != "root" {
                eprintln!("PacWhy must be run as root! This is because of security reasons");
                return Ok(());
            },
            Err(e) => println!("Failed to get USER env: {}", e),
        };
    }

    let cli = CLI::parse();
    let current_exe_path = std::env::current_exe().unwrap();
    let db = Database::connect("sqlite://".to_owned() + current_exe_path.parent().unwrap().to_str().unwrap() + "/pacwhy.db?mode=rwc").await?;
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    db.execute(backend.build(&schema.create_table_from_entity(Package)
        .if_not_exists()
        .to_owned())).await?;

    match cli.commands {
        Commands::Add { name, description, reason } => add_handler(
            name, description, reason, &db
        ).await,
        Commands::Remove { name } => remove_handler(
            name, &db
        ).await,
        Commands::Find { name, description, time, parseable } => find_handler(
            name, description, time, parseable, false, &db
        ).await,
        Commands::Update { name, description, reason } => update_handler(
            name, description, reason, &db
        ).await,
        Commands::List { /*is_dependency,*/ parseable } => list_handler(
            parseable, &db
        ).await,
        Commands::Scripts { args } => scripts_handler(
            args, &db
        ).await,
        Commands::Sync { } => sync_handler(
            &db
        ).await
    };

    db.close().await?;

    Ok(())
}
