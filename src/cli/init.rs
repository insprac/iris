use crate::result::Result;

pub fn init() -> Result<()> {
    let iris_dir = crate::env::iris_dir();
    std::fs::create_dir_all(&iris_dir)?;
    crate::git::commit("Initial commit")?;

    Ok(())
}

