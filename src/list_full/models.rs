use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateEntryDate{
     pub title: String,
     pub description: String,
     pub code: String,
     pub hour: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryDate{
    pub title: String,
    pub description: String,
    pub code: String,
    pub hour: String,
}
