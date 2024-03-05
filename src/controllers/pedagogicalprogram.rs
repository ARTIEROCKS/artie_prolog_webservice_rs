use loco_rs::prelude::*;

use crate::views::pedagogicalprogram::PedagogicalProgramResponse;

async fn get_pedagogical_program() -> Result<Json<PedagogicalProgramResponse>> {
    format::json(PedagogicalProgramResponse::new("test-id", "test-institution-id", "test-program"))
}

async fn query_pedagogical_program() -> Result<Json<PedagogicalProgramResponse>> {
    format::json(PedagogicalProgramResponse::new("test-id", "test-institution-id", "test-program"))
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/pedagogicalprogram", get(get_pedagogical_program))
        .add("/pedagogicalprogram/query", post(query_pedagogical_program))
}
