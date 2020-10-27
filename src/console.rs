mod auth;
mod network;
mod protocol;
mod service;
mod util;

use auth::{User, UserManager};
use sqlx::SqlitePool;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(StructOpt)]
pub(crate) struct Query {
    /// Unique id of student card
    #[structopt()]
    card: Option<String>,
    /// Student id
    #[structopt()]
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

async fn do_add(pool: SqlitePool, new_user: Add) {
    let manager = UserManager::new(&pool);
    let card_bytes = hex::decode(new_user.card).expect("Invalid card id, hex is needed.");
    let card = util::bytes_to_u32(<&[u8; 4]>::try_from(card_bytes.as_slice()).unwrap()) as i64;

    let new: User = User {
        student_id: new_user.id,
        name: new_user.name,
        card,
        created_at: None,
    };

    manager.add(new).await.unwrap();
}

async fn do_query() {}

#[async_std::main]
async fn main() {
    let opt = Command::from_args();

    /* Connect (Open) database */
    let pool = SqlitePool::new("sqlite:lock.db").await.unwrap();
    let manager = auth::UserManager::new(&pool);
    //
    // let u = manager.query_by_student_id("1812100505").await;
    // println!("{:?}", u);
    //
    // let card_id = crate::util::bytes_to_u32(&[0x13, 0xBB, 0xA6, 0x15]);
    // let u = manager.query_by_card(card_id as i64).await;
    // println!("{:?}", u);

    match opt {
        Command::Add(new_user) => do_add(pool, new_user).await,
        Command::Query(query) => do_query().await,
    }
}
