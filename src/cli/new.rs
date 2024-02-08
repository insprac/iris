use std::{env, fs::File, io::Write, path::Path};

const PROMPT: &str = r#"
You are creating a new bash script.
You will be provided with a description of the script.
Do not provide any description, just output the new bash script.
"#;

pub async fn new_script(name: Option<String>, description: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let iris_dir = env::var("IRIS_DIR")?;
    let name = name.unwrap_or(super::input::read("Script name"));
    let file_path = Path::new(&iris_dir).join(&name);
    if file_path.exists() {
        println!("[error] Script already exists");
        return Ok(());
    }

    let description = description.unwrap_or(super::input::read("Description"));

    let mut messages = vec![
        crate::gpt::system_msg(PROMPT),
        crate::gpt::system_msg(&format!("Creating {}", &name)),
        crate::gpt::user_msg(&description),
    ];

    let file_contents = crate::gpt::chat(messages.clone()).await?;
    let file_contents = crate::string::trim_markdown_code_block(&file_contents);

    let mut file = File::create(&file_path)?;
    writeln!(file, "{}", &file_contents)?;

    println!("[created] {}", &name);

    messages.push(crate::gpt::system_msg("Now write a commit message for the new script."));
    let commit_msg = crate::gpt::chat(messages).await?;
    let commit_msg = crate::string::trim_quotes(&commit_msg);

    println!("[commit] {}", &name);

    crate::git::commit_file(&commit_msg, &name)?;

    Ok(())
}
