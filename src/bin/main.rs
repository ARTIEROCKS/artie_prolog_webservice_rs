use loco_rs::cli;
use artie_prolog_webservice_rs_test::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App>().await
}
