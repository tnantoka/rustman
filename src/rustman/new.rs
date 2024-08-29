use std::error;
use std::fs;
use std::path::Path;

const HELLO_MD: &str = include_str!("../templates/contents/posts/2024-08-01-hello.md");
const INDEX_MD: &str = include_str!("../templates/contents/index.md");
const LAYOUT_HTML: &str = include_str!("../templates/contents/layout.html");
const CSS: &str = include_str!("../templates/contents/assets/pico.classless.min.css");

pub fn run(name: &str) -> Result<(), Box<dyn error::Error>> {
    let root = Path::new(name);
    let contents_dir = root.join("contents");
    let posts_dir = contents_dir.join("posts");
    let assets_dir = contents_dir.join("assets");

    if root.exists() {
        return Err("Directory already exists".into());
    }

    fs::create_dir_all(&posts_dir)?;
    fs::create_dir_all(&assets_dir)?;

    fs::write(posts_dir.join("2024-08-01-hello.md"), HELLO_MD)?;
    fs::write(contents_dir.join("index.md"), INDEX_MD)?;
    fs::write(contents_dir.join("layout.html"), LAYOUT_HTML)?;
    fs::write(assets_dir.join("pico.classless.min.css"), CSS)?;
    fs::write(root.join(".gitignore"), "/build\n")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run() {
        let dir = std::env::temp_dir().join("new_test_run");
        std::fs::remove_dir_all(&dir).ok();

        let result = super::run(dir.as_path().to_str().unwrap());

        assert!(result.is_ok());
        assert!(std::path::Path::new(&dir).exists());
    }

    #[test]
    fn test_run_already_exists() {
        let dir = std::env::temp_dir().join("new_test_run_already_exists");
        std::fs::create_dir(&dir).ok();

        let result = super::run(dir.as_path().to_str().unwrap());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Directory already exists");
        println!("{:?}", dir);
        assert!(!std::path::Path::new(&dir.join("contents")).exists());
    }
}
