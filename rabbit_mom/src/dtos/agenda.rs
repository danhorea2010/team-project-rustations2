use serde::{Serialize, Deserialize};

use crate::models::agenda::{Agenda, NewAgenda};


#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct AgendaDTO {
    pub id: i32,
    pub title: String,
    pub deadline: chrono::NaiveDateTime
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct NewAgendaDTO {
    pub title: String,
    pub deadline: chrono::NaiveDateTime
}

impl From<Agenda> for AgendaDTO {
    fn from(agenda: Agenda) -> Self {
        AgendaDTO {
            id: agenda.id,
            title: agenda.title,
            deadline: agenda.deadline.clone()
        }
    }
}

impl From<NewAgendaDTO> for NewAgenda {
    fn from(dto: NewAgendaDTO) -> Self {
        NewAgenda { 
            title: dto.title, 
            deadline: dto.deadline.clone()
        }
    } 
}


