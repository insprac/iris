use std::io::Write;

const PROMPT: &str = r#"
You are editing a bash script.
The file will be provided then a description of the changes will be requested.
Do not provide any description, just output the modified bash script.
"#;

pub async fn edit_script(name: Option<String>, description: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let iris_dir = crate::env::iris_dir();
    let name = name.unwrap_or(super::input::read("Script name"));
    let file_path = std::path::Path::new(&iris_dir).join(&name);
    if !file_path.exists() {
        println!("[error] Script does not exist");
        return Ok(());
    }

    let description = description.unwrap_or(super::input::read("Description"));

    let file_content = std::fs::read_to_string(&file_path)?;

    let mut messages = vec![
        crate::gpt::system_msg(&format!("Editing {}", &name)),
        crate::gpt::system_msg(PROMPT),
        crate::gpt::system_msg(&format!("```bash\n{}\n```", &file_content)),
        crate::gpt::user_msg(&description),
    ];
    let result = crate::gpt::chat(messages.clone()).await?;
    let file_content = crate::string::trim_markdown_code_block(&result);

    let mut file = std::fs::File::options().write(true).truncate(true).open(&file_path)?;
    write!(file, "{}", &file_content)?;

    println!("[updated] {}", &name);

    messages.push(crate::gpt::system_msg("Now write a commit message for the changes."));
    let commit_msg = crate::gpt::chat(messages).await?;
    let commit_msg = crate::string::trim_quotes(&commit_msg);

    println!("[commit] {}", &commit_msg);

    crate::git::commit_file(&commit_msg, &name)?;

    Ok(())
}
