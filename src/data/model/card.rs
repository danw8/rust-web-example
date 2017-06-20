use super::super::schema::card;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
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
            0 => CardStatus::Incomplete,
            1 => CardStatus::Complete,
            _ => CardStatus::Unknown
        }
    }

    pub fn get_status_int(&self) -> i32{
        self.status
    }
}

#[derive(Insertable, Deserialize, Serialize,)]
#[table_name="card"]
pub struct NewCard {
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub status: i32,
}

impl NewCard {
    pub fn get_status(&self) -> CardStatus{
        match self.status {
            0 => CardStatus::Incomplete,
            1 => CardStatus::Complete,
            _ => CardStatus::Unknown
        }
    }
}

#[derive(Debug)]
pub enum CardStatus{
    Incomplete = 0,
    Complete = 1,
    Unknown = 2,
}