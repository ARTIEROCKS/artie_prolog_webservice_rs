use serde::{Deserialize, Serialize};

impl PedagogicalProgramResponse {
    #[must_use]
    pub fn new(id: &str, institution_id: &str, program: &str) -> Self {
        Self {
            id: id.to_string(),
            institution_id: institution_id.to_string(),
            program: program.to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct PedagogicalProgramResponse{
    pub id: String,
    pub institution_id: String,
    pub program: String
}