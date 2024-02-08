use crate::result::Result;
use git2::Repository;

pub fn commit(message: &str) -> Result<()> {
    let iris_dir = crate::env::iris_dir();
    let repo = Repository::open(&iris_dir)?;
    let mut index = repo.index()?;
    index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let signature = git2::Signature::now("iris", "iris@iris")?;
    let parent_commit = repo.head()?.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}

pub fn commit_file(message: &str, file_path: &str) -> Result<()> {
    let iris_dir = crate::env::iris_dir();
    let repo = Repository::open(&iris_dir)?;
    let mut index = repo.index()?;
    index.add_path(std::path::Path::new(file_path))?;
    index.write()?;

    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let signature = git2::Signature::now("iris", "iris@iris")?;
    let parent_commit = repo.head()?.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}
