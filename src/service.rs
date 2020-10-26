use tide::prelude::*;
use tide::Request;

pub async fn serve() -> anyhow::Result<()> {
    let mut app = tide::new();

    app.at("/door/unlock").get(unlock_door);
    app.listen("0.0.0.0:10291").await?;
    Ok(())
}

async fn unlock_door(req: Request<()>) -> tide::Result<serde_json::Value> {
    Ok(json!({
        "code": 0,
        "msg": String::from("已开门"),
    }))
}
