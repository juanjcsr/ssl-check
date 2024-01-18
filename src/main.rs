use std::env;
use std::process::ExitCode;


mod sslcheck;

fn main() -> ExitCode {
    println!("Hello, world!");

    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() <= 1 {
        println!("Usage: {} <url>", env::args().next().unwrap());
        return ExitCode::from(1)
    }

    let url = &cli_args[1];

    println!("URL: {}", url);

    println!("SSL Expiry: {}", sslcheck::checker::ssl_expiry(url));
    return ExitCode::from(0)
}
