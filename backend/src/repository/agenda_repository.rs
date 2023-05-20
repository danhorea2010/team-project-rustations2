use diesel::{RunQueryDsl, QueryResult};

use crate::models::agenda::Agenda;

use crate::schema::agendas::dsl::*;
use super::connection::database_connection::establish_connection;


pub fn get_all_agendas() -> Vec<Agenda> {
    let connection = &mut establish_connection();

    let results = agendas.load::<Agenda>(connection);

    match results {
        QueryResult::Err(error) => {
            error!("{}", error);
            Vec::new()
        },
        QueryResult::Ok(query_results) => query_results
    }

}
