use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Default, Debug, Clone, PartialEq)]
pub enum Stage {
    #[strum(serialize = "Local")]
    Local,

    #[strum(serialize = "Development")]
    #[default]
    Development,

    #[strum(serialize = "Production")]
    Production,
}