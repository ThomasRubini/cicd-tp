use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long, env, default_value = "cicd")]
    pub pg_user: String,

    #[arg(long, env, default_value = "cicd")]
    pub pg_passwd: String,

    #[arg(long, env, default_value = "cicd")]
    pub pg_db: String,

    #[arg(long, env, default_value = "localhost")]
    pub pg_host: String,

    #[arg(long, env, default_value = "5432")]
    pub pg_port: u16,
}

pub fn parse_args() -> Args {
    Args::parse()
}
