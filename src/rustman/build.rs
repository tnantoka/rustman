use pulldown_cmark::{html, Event, Options, Parser};
use std::error;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

const CONTENTS_DIR: &str = "contents";
const BUILD_DIR: &str = "build";

struct File {
    path: PathBuf,
    title: String,
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let contents_dir = Path::new(CONTENTS_DIR);
    let build_dir = Path::new(BUILD_DIR);

    let mut files = Vec::new();
    list_files(contents_dir, &mut files)?;
    files.sort_by(|a, b| b.path.cmp(&a.path));

    fs::remove_dir_all(build_dir).ok();
    fs::create_dir_all(build_dir)?;

    for file in &files {
        let src_path = &file.path;
        let relative_path = src_path.strip_prefix(contents_dir)?;
        let is_md = src_path.extension() == Some("md".as_ref());
        let dst_path = if is_md {
            build_dir.join(relative_path).with_extension("html")
        } else {
            build_dir.join(relative_path)
        };

        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        if is_md {
            let markdown = if src_path.ends_with("index.md") {
                build_index(fs::read_to_string(src_path)?, &files)
            } else {
                fs::read_to_string(src_path)?
            };

            let html = render_markdown(&markdown);
            fs::write(&dst_path, render_layout(&file.title, html))?;
        } else {
            fs::copy(src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn list_files(dir: &Path, files: &mut Vec<File>) -> Result<(), Box<dyn error::Error>> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.file_name().unwrap().to_string_lossy().starts_with('.')
            || path.ends_with("layout.html")
        {
            continue;
        }

        if path.is_dir() {
            list_files(&path, files)?;
        } else {
            files.push(File {
                path: path.clone(),
                title: read_title(&path),
            });
        }
    }

    Ok(())
}

fn render_markdown(src: &str) -> String {
    let parser = Parser::new_ext(src, Options::all()).map(|event| match event {
        Event::SoftBreak => Event::HardBreak,
        _ => event,
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn render_layout(title: &str, content: String) -> String {
    let layout = fs::read_to_string("contents/layout.html").unwrap();
    layout
        .replace("{{title}}", title)
        .replace("{{content}}", &content)
}

fn build_index(content: String, files: &[File]) -> String {
    let list = files.iter().fold(String::new(), |mut acc, file| {
        let path = &file.path;

        if !path.parent().unwrap().ends_with("posts") || path.extension() != Some("md".as_ref()) {
            return acc;
        }

        let file_stem = path.file_stem().unwrap().to_string_lossy();
        let date = file_stem
            .split('-')
            .take(3)
            .collect::<Vec<&str>>()
            .join("-");

        writeln!(acc, "- [{}](posts/{}.html) {}", file.title, file_stem, date).unwrap();
        acc
    });

    content.replace("{{posts}}", &list)
}

fn read_title(path: &PathBuf) -> String {
    if path.extension() != Some("md".as_ref()) {
        return String::new();
    }

    fs::read_to_string(path)
        .unwrap()
        .lines()
        .find(|line| line.starts_with("# "))
        .unwrap()
        .replace("# ", "")
}

mod tests {
    #[test]
    fn test_run() {
        let dir = std::env::temp_dir().join("build_test_run");
        std::fs::remove_dir_all(&dir).ok();

        super::super::new::run(dir.as_path().to_str().unwrap()).ok();

        std::env::set_current_dir(&dir).ok();
        let result = super::run();

        assert!(result.is_ok());
        assert!(std::path::Path::new(&dir.join("build")).exists());
    }
}
