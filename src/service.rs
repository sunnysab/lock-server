use crate::auth::{User, UserManager};
use crate::EnvData;
use tide::prelude::*;
use tide::Request;

pub async fn serve(env: EnvData) -> anyhow::Result<()> {
    let mut app = tide::with_state(env);

    app.at("/user").post(create_user);
    app.listen("0.0.0.0:10291").await?;
    Ok(())
}

// async fn get_user_detail(req: Request<EnvData>) -> tide::Result<serde_json::Value> {
//     req.
// }

async fn create_user(mut req: Request<EnvData>) -> tide::Result<serde_json::Value> {
    let user: User = req.body_json().await.unwrap();
    let pool = req.state();
    let manager = UserManager::new(pool);

    manager.add(user).await.unwrap();
    Ok(json!({
        "code": 0,
    }))
}
