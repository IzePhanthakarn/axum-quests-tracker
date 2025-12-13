use diesel::prelude::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::adventures::AdventurerEntity,
    domain::entities::quests::QuestEntity,
    infrastructure::postgres::schema::quest_adventurer_junction,
};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Associations)]
#[diesel(belongs_to(AdventurerEntity, foreign_key = adventurer_id))]
#[diesel(belongs_to(QuestEntity, foreign_key = quest_id))]
#[diesel(table_name = quest_adventurer_junction)]
pub struct QuestAdventurerJunction {
    pub adventurer_id: i32,
    pub quest_id: i32
}