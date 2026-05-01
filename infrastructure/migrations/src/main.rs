use app_config::Settings;
use migrations::Migrator;
use migrations::sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::process::ExitCode;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Up(Option<u32>),
    Down(Option<u32>),
    Fresh,
    Refresh,
    Reset,
    Status,
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(CommandError::Message(message)) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), CommandError> {
    let settings = Settings::load().expect("Failed to load settings");
    let db = Database::connect(settings.database_url())
        .await
        .expect("Failed to connect to a database.");

    match parse_args(std::env::args()) {
        Ok(Command::Up(steps)) => Migrator::up(&db, steps).await.expect("Migration failed!"),
        Ok(Command::Down(steps)) => Migrator::down(&db, steps).await.expect("Rollback failed!"),
        Ok(Command::Fresh) => Migrator::fresh(&db).await.expect("Fresh failed!"),
        Ok(Command::Refresh) => Migrator::refresh(&db).await.expect("Refresh failed!"),
        Ok(Command::Reset) => Migrator::reset(&db).await.expect("Reset failed!"),
        Ok(Command::Status) => {
            let statuses = Migrator::get_pending_migrations(&db)
                .await
                .expect("Failed to get status");
            for m in statuses {
                println!("Pending: {}", m.name());
            }
        }
        Err(message) => return Err(CommandError::Message(message)),
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum CommandError {
    Message(String),
}

fn parse_args<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let _bin = args.next();
    let command = args.next().unwrap_or_else(|| "up".to_string());
    let rest: Vec<String> = args.collect();

    match command.as_str() {
        "up" => Ok(Command::Up(parse_steps(&rest)?)),
        "down" => Ok(Command::Down(parse_steps(&rest)?)),
        "fresh" => Ok(Command::Fresh),
        "refresh" => Ok(Command::Refresh),
        "reset" => Ok(Command::Reset),
        "status" => Ok(Command::Status),
        _ => Err(format!("Unknown command: {command}.")),
    }
}

fn parse_steps(args: &[String]) -> Result<Option<u32>, String> {
    match args {
        [] => Ok(None),
        [steps] => steps
            .parse::<u32>()
            .map(Some)
            .map_err(|_| format!("Invalid steps value: {steps}.")),
        [flag, steps] if flag == "-n" || flag == "--num" => steps
            .parse::<u32>()
            .map(Some)
            .map_err(|_| format!("Invalid steps value: {steps}.")),
        _ => Err(format!("Unexpected arguments: {}.", args.join(" "))),
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, parse_args, parse_steps};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn defaults_to_up_when_no_command_is_provided() {
        let parsed = parse_args(args(&["migrations"])).expect("command should parse");

        assert_eq!(parsed, Command::Up(None));
    }

    #[test]
    fn parses_step_count_from_short_flag() {
        let parsed =
            parse_args(args(&["migrations", "up", "-n", "10"])).expect("command should parse");

        assert_eq!(parsed, Command::Up(Some(10)));
    }

    #[test]
    fn parses_step_count_from_plain_numeric_argument() {
        let parsed = parse_args(args(&["migrations", "down", "2"])).expect("command should parse");

        assert_eq!(parsed, Command::Down(Some(2)));
    }

    #[test]
    fn parses_refresh_command() {
        let parsed = parse_args(args(&["migrations", "refresh"])).expect("command should parse");

        assert_eq!(parsed, Command::Refresh);
    }

    #[test]
    fn rejects_unknown_command() {
        let error =
            parse_args(args(&["migrations", "generate"])).expect_err("command should be rejected");

        assert_eq!(error, "Unknown command: generate.");
    }

    #[test]
    fn rejects_non_numeric_step_count() {
        let error = parse_steps(&args(&["-n", "many"])).expect_err("steps should be rejected");

        assert_eq!(error, "Invalid steps value: many.");
    }

    #[test]
    fn rejects_unexpected_extra_arguments() {
        let error = parse_steps(&args(&["1", "2"])).expect_err("steps should be rejected");

        assert_eq!(error, "Unexpected arguments: 1 2.");
    }
}
