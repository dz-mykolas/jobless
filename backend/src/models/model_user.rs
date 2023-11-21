pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub tags: Option<Vec<String>>,
    pub resume: Option<String>,
}

pub struct UserForCreate {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub struct UserForUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub tags: Option<Vec<String>>,
    pub resume: Option<String>,
}
