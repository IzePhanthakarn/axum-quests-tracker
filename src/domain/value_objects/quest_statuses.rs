use serde::{ Deserialize, Serialize };
use strum_macros::Display;

#[derive(Display, Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestStatus {
    #[default]
    #[strum(serialize = "Open")]
    Open,

    #[strum(serialize = "InJourney")]
    InJourney,

    #[strum(serialize = "Completed")]
    Completed,

    #[strum(serialize = "Failed")]
    Failed,
}
