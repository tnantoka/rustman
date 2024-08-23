use std::error;
use std::fs;
use std::path::Path;

const HELLO_MD: &str = include_str!("../templates/contents/posts/2024-08-01-hello.md");
const INDEX_MD: &str = include_str!("../templates/contents/index.md");

pub fn run(name: &str) -> Result<(), Box<dyn error::Error>> {
    let root = Path::new(name);
    let contents_dir = root.join("contents");
    let posts_dir = contents_dir.join("posts");

    if root.exists() {
        return Err("Directory already exists".into());
    }

    fs::create_dir_all(&posts_dir)?;

    fs::write(posts_dir.join("2024-08-01-hello.md"), HELLO_MD)?;
    fs::write(contents_dir.join("index.md"), INDEX_MD)?;
    fs::write(root.join(".gitignore"), "/build\n")?;

    Ok(())
}
