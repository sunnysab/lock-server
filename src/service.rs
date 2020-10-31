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
    // Load user parameters
    let user_param: User = req.body_json().await?;
    let new_user = User::new(user_param.student_id, user_param.name, user_param.card);

    // Open database
    let pool = req.state();
    let manager = UserManager::new(pool);

    // Add a new user to db.
    manager.add(new_user).await?;

    Ok(json!({
        "code": 0,
    }))
}
