use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};
use rocket::fairing::AdHoc;


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(Db::init())
            .mount("/db", routes![entry,read])
    })
}

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::SqlitePool);


#[get("/read/<id>")]
async fn read(mut db: Connection<Db>, id: i64) -> String {
   let reg = sqlx::query(&format!("SELECT Nombre FROM Registro WHERE entry_id = {};", id)).bind(id)
       .fetch_one(&mut *db).await
       .and_then(|r| Ok(r.try_get_unchecked(0)?))
       .ok();

   match reg {
       Some(x) => x,
       None => String::from("Not Found")
   }


}

#[get("/entry/<person>")]
async fn entry(person : String, mut db : Connection<Db>) ->  String {
    let entry_id = sqlx::query(&format!("INSERT INTO Registro (Nombre) VALUES ('{}') RETURNING entry_id;", person))
    .bind(person)
    .fetch_one(&mut *db).await
    .and_then(|r| Ok(r.try_get_unchecked(0)?))
    .ok();

    match entry_id {
        Some(x) => x,
        None => String::from("Some error")
    }
}


// #[get("/all")]
// async fn all(mut db : Connection<Db>) -> Result<Json<Vec<String>>> {
//     let x = sqlx::query("SELECT * FROM Registro;")
//     .fetch_all(&mut *db)
//     .map_ok(|r| r)
//     .try_collect::<Vec<String>>()
//     .await?;

//     Ok(Json(x))

// }
