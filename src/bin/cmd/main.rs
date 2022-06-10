use clap::{Parser, Subcommand};
use sqlstuff::{DbError, MyDB, CRUD};

#[tokio::main]
async fn main() -> Result<(), DbError> {
    let conn = MyDB::new().await?;
    match Args::parse().cmd {
        DbAction::CreateUser {username, email} => {
            let created_user = conn.create_user(&username, &email).await?;
            dbg!(created_user);
        },
        _ => Err(String::from("unimplemented"))?,
    }
    Ok(())
}

// Simple app to interact with database
#[derive(Parser)]
struct Args {
    /// Database action to perform
    #[clap(subcommand)]
    cmd: DbAction
}

#[derive(Subcommand)]
enum DbAction {
    /// create a new user
    CreateUser {
        #[clap(forbid_empty_values = true)]
        /// name for the user
        username: String,

        #[clap(forbid_empty_values=true,validator=validate_email)]
        /// email for the user. must be unique.
        email: String,
    },
    /// update existing user
    UpdateUser {
        /// id of user
        id: i64,

        #[clap(forbid_empty_values = true)]
        // optional username to update
        username: Option<String>,

        #[clap(forbid_empty_values=true,validator=validate_email)]
        /// optional email to update. must be unique
        email: Option<String>,
    },
    /// find existing user
    FindUser {
        /// email of user
        #[clap(forbid_empty_values=true,validator=validate_email)]
        email: Option<String>,
    },
    /// delete existing user
    DeleteUser {
        /// id to delete
        id: i64,
    },
}

fn validate_email(email: &str) -> Result<(), String> {
    if email.contains(" ") {
        Err(String::from("email cannot contain spaces"))
    } else if email.contains("@") {
        Ok(())
    } else {
        Err(String::from("email is not valid"))
    }
}
