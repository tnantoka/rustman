use pulldown_cmark::{html, Options, Parser};
use std::error;
use std::fs;
use std::path::{Path, PathBuf};

const CONTENTS_DIR: &str = "contents";
const BUILD_DIR: &str = "build";

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let contents_dir = Path::new(CONTENTS_DIR);
    let build_dir = Path::new(BUILD_DIR);

    let mut files = Vec::new();
    list_files(contents_dir, &mut files)?;

    fs::remove_dir_all(build_dir).ok();
    fs::create_dir_all(build_dir)?;

    for src_path in files {
        if src_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with('.')
        {
            continue;
        }

        let relative_path = src_path.strip_prefix(contents_dir).unwrap();
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
            fs::write(&dst_path, render_markdown(&fs::read_to_string(&src_path)?))?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn list_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn error::Error>> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            list_files(&path, files)?;
        } else {
            files.push(path);
        }
    }

    Ok(())
}

pub fn render_markdown(src: &str) -> String {
    let parser = Parser::new_ext(src, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
