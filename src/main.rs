mod env;
mod result;
mod string;
mod gpt;
mod cli;
mod git;

#[tokio::main]
async fn main() {
    openai::set_key(env::openai_api_key());

    cli::call().await.unwrap();
}
