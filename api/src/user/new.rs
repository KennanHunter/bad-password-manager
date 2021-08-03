use rocket::{post, request::Form, FromForm};

#[derive(FromForm)]
pub struct MyForm {
    field: String,
}

#[post("/new", data = "<new_user>")]
pub fn add_user(new_user: Form<MyForm>) -> String {
    let res = new_user.into_inner().field;
    println!("{}", res);
    res
}
