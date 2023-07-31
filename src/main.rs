use clap::{Args, CommandFactory, Parser, Subcommand, error::ErrorKind};

pub const INVALID_TOKEN_DEFAULT_VALUE: &str = "INVALID";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    resource: Resource,
    // Enable verbose logging
    #[arg(default_value = "true", global = true, short, long)]
    verbose: bool,
    /// Personal access token
    #[arg(global = true, default_value = INVALID_TOKEN_DEFAULT_VALUE, short, long)]
    pub access_token: String,
}

#[derive(Debug, Subcommand)]
enum Resource {
    #[clap(subcommand)]
    /// Create, Read, Update or Delete a User entity
    User(UserCommand),
}

#[derive(Debug, Subcommand)]
enum UserCommand {
    /// Create a user
    Create(CreateUserCommand),
}

#[derive(Debug, Args)]
struct CreateUserCommand {
    /// The username
    #[arg(short, long)]
    username: String,
    /// The email
    #[arg(short, long)]
    email: String,
}

fn main() {
    let args = Cli::parse();
    let mut cmd = Cli::command();
    if args.access_token.eq(INVALID_TOKEN_DEFAULT_VALUE) {
        cmd.error(
            ErrorKind::InvalidValue,
            "Please provide a valid value for access token",
        )
        .exit();
    }
    println!("{:?}", args);
}
