use clap::Command;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use crate::book::Book;

mod book;

fn cli() -> Command<'static> {
    Command::new("pekin")
        .about("Simplified GNUCash for terminal.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Create the database structure to start.")
        )
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connect database
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

    // Start the CLI
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init(&pool).await,
        _ => unreachable!(),
    };

    Ok(())
}

async fn init(pool: &Pool<Sqlite>) {
    println!("🤖 Generating database");
    sqlx::migrate!()
        .run(pool)
        .await;

    // Insert a default book
    Book::new().save(pool).await;

    // Read all books
    let books = book::find_all(pool).await;
    for book in books {
        println!("{:?}", book);
    }
}
