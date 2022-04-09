use std::borrow::Borrow;
use actix_web::{Error, HttpResponse, web};
use actix_web::web::Header;
use log::info;

use bean::Rst;

use crate::bean;
use crate::bean::*;

pub(crate) async fn login(user: web::Json<User>) -> HttpResponse {
    info!("user: {:?}", &user);
    let a = &user.username;
    info!("{}", a);

    HttpResponse::Ok().json(Rst::ok(None))
}

pub(crate) async fn login_some_user(user: web::Json<SomeUser>) -> Result<HttpResponse, Error> {
    // https://riptutorial.com/rust/example/9458/unwrapping-a-reference-to-an-option-owning-its-contents
    let _ = &user.is_valid()?;

    let uname = &user.username.as_ref().unwrap();
    println!("{}", uname);

    let password = &user.password()?;
    println!("password : {}", password);

    let u = &user.username.as_ref().unwrap();
    println!("{}", u);
    // info!("some user: {:?}", &name);
    Ok(HttpResponse::Ok().json(Rst::ok(None)))
}
