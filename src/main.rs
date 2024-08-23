mod rustman;

use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Usage: rustman <command>".into());
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 3 {
                return Err("Usage: rustman new <project-name>".into());
            }
            rustman::new::run(args[2].as_str())?;
        }
        "build" => rustman::build::run()?,
        _ => return Err(format!("Unknown command: {}", args[1]).into()),
    }

    Ok(())
}
