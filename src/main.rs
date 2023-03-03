
#[macro_use] extern crate rocket;
use rocket::State;
use std::sync::Mutex;
use rocket_db_pools::{Database};


#[path="./db/db.rs"]
mod db;

use db::{Db, stage};


#[get("/")]
fn hello()  -> &'static str{
    "Hello Rocket 🚀"
}


#[get("/all")]
fn all_vector(vector : &State<Mutex<Vec<i32>>>) -> String {
    let inner = vector.inner();
    let binding = inner.lock().unwrap();
    let mut string_vec = String::from("");
    for x in binding.to_vec() {
        if string_vec.is_empty() {
            string_vec.push_str(&x.to_string());
            continue
        }
        string_vec.push_str(", ");
        string_vec.push_str(&x.to_string())
    }
    return string_vec;
}

#[get("/<index>")]
fn vec(index : usize, elvector : &State<Mutex<Vec<i32>>>) -> String {
    let inner = elvector.inner();
    let mut binding = inner.lock().unwrap();
    binding.push(index.try_into().unwrap());
    print!("Added {} in pos {}.🌊", index, binding.len());
    let num = binding.get(index);
    match num {
        Some(x) => {format!("Ok {}", x)},
        None => String::from("Invalid Index 😈")
    }


}

#[launch]
fn rocket()  -> _ {


    rocket::build()
        .manage(Mutex::new(vec![0,1,2,3,4,10]))
        .manage(Db::init())
        .mount("/hello", routes![hello])
        .mount("/vec", routes![vec, all_vector])
        .attach(stage())
}
