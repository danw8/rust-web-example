use super::super::schema::card;

#[derive(Queryable, Deserialize, Serialize, Clone)]
pub struct Card {
	pub id: i32,
	pub user_id: i32,
	pub title: String,
	pub description: String,
    status: i32,
}

impl Card {
    pub fn set_status(&mut self, status: CardStatus){
        self.status = status as i32;
    }

    pub fn get_status(&self) -> CardStatus{
        match self.status {
            1 => CardStatus::Incomplete,
            2 => CardStatus::Complete,
            _ => CardStatus::Unknown
        }
    }
}

#[derive(Insertable)]
#[table_name="card"]
pub struct NewCard<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub description: &'a str,
    pub status: i32,
}

pub enum CardStatus{
    Incomplete = 1,
    Complete = 2,
    Unknown = 3,
}