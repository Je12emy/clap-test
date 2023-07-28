use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    resource: Resource,
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
    username: String,
    /// The email
    email: String,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
    match args.resource {
        Resource::User(user) => match user {
            UserCommand::Create(user_create) => {
                println!("{:?}", user_create)
            }
        },
    }
}
