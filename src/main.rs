#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use rocket_cors::{Cors, AllowedOrigins,CorsOptions,AllowedHeaders,AllowedMethods,AllOrSome};
use rocket::ignite;
// use rocket_contrib::json::Json;
// use rocket_contrib::json;
// use rocket::response::content::Json;
// use rocket::http::hyper::Response;
use serde_json::json;

#[get("/hello")]
fn makehello() -> rocket::response::content::Json<String> {
    let user = serde_json::json!({
        "name": "Jon Snow".to_string(),
        "age": 21,
        "alive": true,
    });
    
    let json_string = serde_json::to_string(&user).unwrap();
    rocket::response::content::Json(json_string)
}


#[get("/play/<file_name>")]
fn play(file_name: String) -> Option<NamedFile> {
    let video_dir = "src/videos";
    let video_path = format!("{}/{}", video_dir, file_name);
    
    //  PathBuffer from the video_path
    let path = PathBuf::from(video_path);

   return NamedFile::open(path).ok()
}

// fn rocket() -> rocket::Rocket {
//     rocket::ignite().mount("/", routes![play])
// }

// // fn main() {
// //     rocket().launch();
// // }

// #[get("/hello")]
// fn makehello() -> Json<&'static str> {
//     return Json("Hello, world!");
//     // return Json(Response{message : "Hello, world!"})
// }


fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&["https://nicodex-angular.vercel.app/","http://nicodex-angular.vercel.app/"],);

    // Create CORS options
    let cors: CorsOptions = CorsOptions {
        allowed_origins: allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    };

    // Attach CORS to your Rocket application
    // rocket::ignite()
    //     .attach(cors)
    //     .mount("/", routes![play])
    //     .launch();
    // rocket::ignite().attach(cors.to_cors().unwrap())

    rocket::ignite()
    .attach(cors.to_cors().unwrap())
    .mount("/", routes![play])
    .mount("/", routes![makehello])
    .launch();
}