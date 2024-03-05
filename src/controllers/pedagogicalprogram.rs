use loco_rs::prelude::*;

use crate::{common, views::pedagogicalprogram::PedagogicalProgramResponse};
use loco_rs::app::AppContext;

async fn get_pedagogical_program(State(ctx): State<AppContext>) -> Result<Json<PedagogicalProgramResponse>> {
    if let Some(settings) = &ctx.config.settings {
        let settings = common::settings::Settings::from_json(settings)?;
        println!("mongodb: {:?}", settings.mongodb.unwrap().username);
    }
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
