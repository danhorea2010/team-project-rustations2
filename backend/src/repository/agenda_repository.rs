use diesel::{QueryResult, RunQueryDsl};

use crate::models::agenda::{Agenda, NewAgenda};
use diesel::prelude::*;

use super::connection::database_connection::establish_connection;
use crate::schema::agendas::dsl::*;
use log::error;

pub fn get_all_agendas() -> Vec<Agenda> {
    let connection = &mut establish_connection();

    let results = agendas.load::<Agenda>(connection);

    match results {
        QueryResult::Err(error) => {
            error!("{}", error);
            Vec::new()
        }
        QueryResult::Ok(query_results) => query_results,
    }
}

pub fn insert_agenda_repo(new_agenda: NewAgenda) -> Vec<Agenda> {
    let new_agendas = vec![new_agenda];
    let connection = &mut establish_connection();

    let results = diesel::insert_into(agendas)
        .values(&new_agendas)
        .get_results(connection);

    match results {
        QueryResult::Err(error) => {
            error!("{}", error);
            Vec::new()
        }
        QueryResult::Ok(query_result) => query_result,
    }
}

pub fn update_agenda_repo(update_agenda: Agenda) -> Agenda {
    let connection = &mut establish_connection();

    let agenda = diesel::update(agendas.filter(id.eq(update_agenda.id)))
        .set((
            title.eq(update_agenda.title.clone()),
            deadline.eq(update_agenda.deadline.clone()),
        ))
        .get_result(connection)
        .expect("Cannot update agenda");

    agenda
}

pub fn delete_agenda_repo(agenda_id: i32) -> bool {
    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(agendas.filter(id.eq(agenda_id)))
        .execute(connection)
        .expect("Error deleting agenda!");

    num_deleted != 0
}
