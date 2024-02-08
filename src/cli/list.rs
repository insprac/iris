use crate::result::Result;

pub fn list_scripts() -> Result<()> {
    let iris_dir = crate::env::iris_dir();
    let scripts = std::fs::read_dir(&iris_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|entry| entry.file_name())
        .filter_map(|name| name.into_string().ok())
        .collect::<Vec<String>>();
    for script in scripts {
        println!("{}", script);
    }
    Ok(())
}
