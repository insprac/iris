pub fn openai_api_key() -> String {
    std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
}

pub fn iris_dir() -> String {
    std::env::var("IRIS_DIR").expect("IRIS_DIR not set")
}
