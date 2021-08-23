#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub pass: String,
}