mod rustman;

use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    run(rustman::new::run, rustman::build::run, &args)
}

fn run(
    run_new: impl Fn(&str) -> Result<(), Box<dyn error::Error>>,
    run_build: impl Fn() -> Result<(), Box<dyn error::Error>>,
    args: &[String],
) -> Result<(), Box<dyn error::Error>> {
    if args.len() < 2 {
        return Err("Usage: rustman <command>".into());
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 3 {
                return Err("Usage: rustman new <project-name>".into());
            }
            run_new(args[2].as_str())?;
        }
        "build" => run_build()?,
        _ => return Err(format!("Unknown command: {}", args[1]).into()),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_run_new() {
        let new_called = Cell::new(false);
        let mock_new = |_: &str| {
            new_called.set(true);
            Ok(())
        };

        let args = vec![
            "rustman".to_string(),
            "new".to_string(),
            "hello".to_string(),
        ];

        run(mock_new, || Ok(()), &args).unwrap();
        assert!(new_called.get());
    }

    #[test]
    fn test_run_build() {
        let build_called = Cell::new(false);
        let mock_build = || {
            build_called.set(true);
            Ok(())
        };

        let args = vec!["rustman".to_string(), "build".to_string()];

        run(|_| Ok(()), mock_build, &args).unwrap();
        assert!(build_called.get());
    }

    #[test]
    fn test_run_args_1() {
        let args = vec!["rustman".to_string()];
        let result = run(|_| Ok(()), || Ok(()), &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Usage: rustman <command>");
    }

    #[test]
    fn test_run_args_2() {
        let args = vec!["rustman".to_string(), "new".to_string()];
        let result = run(|_| Ok(()), || Ok(()), &args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Usage: rustman new <project-name>"
        );
    }

    #[test]
    fn test_run_unknown() {
        let args = vec!["rustman".to_string(), "unknown".to_string()];
        let result = run(|_| Ok(()), || Ok(()), &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unknown command: unknown");
    }
}
