#[allow(dead_code)]
#[allow(unused_variables)]
mod generated_cli;

use convert_case::{Case, Casing};
use generated_cli::*;

trait PoorEstr: std::fmt::Debug {
    fn poor_str(&self) -> String {
        let dbg_str = format!("{:?}", self);
        let poor = dbg_str
            .rsplit_once(':')
            .map_or(dbg_str.as_str(), |(_a, b)| b);
        poor.to_case(Case::Kebab)
    }
}

impl PoorEstr for CliCommand {}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // to use, set e.g. RUST_LOG=reqwest=trace,hyper=trace
    tracing_subscriber::fmt::init();

    // auth
    let openapi_key = std::env::var("OPENAI_API_KEY").expect("set OPENAI_API_KEY");
    let mut bearer =
        reqwest::header::HeaderValue::from_str(&format!("Bearer {openapi_key}")).unwrap();
    bearer.set_sensitive(true);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", bearer);
    let client_builder = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    // exec
    let client = sdk::Client::new_with_client("https://api.openai.com/v1", client_builder);
    let cli = Cli::new(client);
    let mut clap = clap::Command::new("sdk").arg_required_else_help(true);
    clap = clap.subcommands(CliCommand::iter().map(|c| Cli::get_command(c).name(c.poor_str())));
    let matches = clap.get_matches();
    for c in CliCommand::iter() {
        if let Some(cmd) = matches.subcommand_matches(&c.poor_str()) {
            cli.execute(c, &cmd).await;
            return;
        }
    }
}
