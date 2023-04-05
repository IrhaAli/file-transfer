#[macro_use] extern crate rocket;
use rocket::fs::TempFile;
use uuid::Uuid;
use std::path::Path;

#[post("/upload", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    let id = Uuid::new_v4();
    let form = format!("./files/{}", id.to_string());
    let path = Path::new(&form);
    file.persist_to(path).await?;
    Ok(())
}

#[get("/download/<_identifier>")]
fn download(_identifier: &str) -> &'static str {
    "You did a hecking download fam!"
}

#[delete("/delete/<_identifier>")]
fn delete(_identifier: &str) -> &'static str {
    "OMG ITS GONEEEE!"
}

#[put("/replace/<_identifier>")]
fn replace(_identifier: &str) -> &'static str {
    "it's new wowza!"
}

#[post("/list")]
fn list() -> &'static str {
    "hey, here's all the files, they're cool!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1", routes![upload, download, delete, replace, list])
}