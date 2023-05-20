use diesel::{Insertable, Queryable};
use crate::schema::agendas;


#[derive(Queryable, Debug, Clone)]
pub struct Agenda {
    pub id: i32,
    pub title: String,
    pub deadline: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = agendas)]
pub struct NewAgenda {
    pub title: String,
    // Can't use TimeStamp directly because 
    // because diesel is rude
    pub deadline: chrono::NaiveDateTime,
}


