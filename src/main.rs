use clap::Command;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use crate::account::Account;
use crate::book::Book;

mod book;
mod account;

fn cli() -> Command<'static> {
    Command::new(String::from(env!(CARGO_PKG_NAME)))
        .about("Simplified GNUCash for terminal.")
        .version(env!("CARGO_PKG_VERSION"))
        .author(String::from(env!(CARGO_PKG_AUTHORS)))
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

    // Insert default datas
    Book::new().save(pool).await;
    Account::new("Asset", None, account::Category::ASSET).save(pool).await;
    Account::new("Equity", None, account::Category::EQUITY).save(pool).await;
    Account::new("Income", None, account::Category::INCOME).save(pool).await;
    Account::new("Expense", None, account::Category::EXPENSE).save(pool).await;
    Account::new("Liability", None, account::Category::LIABILITY).save(pool).await;

    // Read all books
    let books = book::find_all(pool).await;
    for book in books {
        println!("{:?}", book);
    }

    let accounts = account::find_all(pool).await;
    for account in accounts {
        println!("{:?}", account);
    }
}
