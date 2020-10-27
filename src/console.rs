#![allow(dead_code)]
mod auth;
mod network;
mod protocol;
mod service;
mod util;

use auth::User;
use sqlx::SqlitePool;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(StructOpt)]
pub(crate) struct Query {
    /// Unique id of student card
    #[structopt(long)]
    card: Option<String>,
    /// Student id
    #[structopt(long)]
    id: Option<String>,
}

#[derive(StructOpt)]
pub(crate) struct Add {
    /// Student id
    id: String,
    /// Student name
    name: String,
    /// Unique id of student card
    card: String,
}

#[derive(StructOpt)]
#[structopt(name = "Lock console")]
enum Command {
    Query(Query),
    Add(Add),
}

async fn do_add(manager: auth::UserManager<'_>, new_user: Add) {
    let card_bytes = hex::decode(new_user.card).expect("Invalid card id, hex is needed.");
    let card = util::bytes_to_u32(<&[u8; 4]>::try_from(card_bytes.as_slice()).unwrap()) as i64;

    let new = User::new(new_user.id, new_user.name, card);
    manager.add(new).await.unwrap();
}

async fn do_query(manager: auth::UserManager<'_>, query: Query) {
    let u: User;

    match query {
        Query { card: Some(card), .. } => {
            let card_bytes = hex::decode(card).expect("Invalid card id, hex is needed.");
            let card = util::bytes_to_u32(<&[u8; 4]>::try_from(card_bytes.as_slice()).unwrap()) as i64;

            u = manager
                .query_by_card(card)
                .await
                .expect("Failed to query")
                .expect("No such user");
        }
        Query {
            id: Some(student), ..
        } => {
            u = manager
                .query_by_student_id(&student)
                .await
                .expect("Failed to query")
                .expect("No such user")
        }
        _ => {
            println!("Please specify a card id or student id to query.");
            return;
        }
    }
    println!("{:?}", u);
}

#[async_std::main]
async fn main() {
    let opt = Command::from_args();

    /* Connect (Open) database */
    let pool = SqlitePool::new("sqlite:lock.db").await.unwrap();
    let manager = auth::UserManager::new(&pool);

    match opt {
        Command::Add(new_user) => do_add(manager, new_user).await,
        Command::Query(query) => do_query(manager, query).await,
    }
}
