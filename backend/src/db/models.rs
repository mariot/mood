use super::schema::mood;

#[derive(Insertable)]
#[table_name = "mood"]
pub struct NewMood<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Mood {
    pub id: i32,
    pub title: String,
}
