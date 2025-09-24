mod args;
mod config;
mod control_thread;
mod worker_thread;

fn main() -> Result<(), Box<dyn std::any::Any + Send>> {
    use clap::{CommandFactory, Parser};
    use control_thread::ControlThread;
    use tracing::error;

    // Parse .env if it exists (and before args in case args want to read
    // environment).
    match dotenvy::dotenv() {
        Ok(_) | Err(dotenvy::Error::Io(_)) => {}
        Err(err) => panic!("Failed to parse .env file; err={err}"),
    }

    // Parse command-line arguments.
    let args = crate::args::Args::parse();

    // If user is requesting completions, return them and exit.
    if let Some(shell) = args.completions {
        clap_complete::generate(
            shell,
            &mut crate::args::Args::command(),
            "rust-template",
            &mut std::io::stdout(),
        );

        return Ok(());
    }

    // Setup tracing.
    let _log_guard = toolbox::tracing::setup_tracing("rust-template", args.logs.as_deref());

    // Log build information (as soon as possible).
    toolbox::log_build_info!();

    // Setup standard panic handling.
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!(?panic_info, "Application panic");

        default_panic(panic_info);
    }));

    // Parse config.
    let config = serde_yaml::from_slice(&toolbox::fs::must_read(&args.config)).unwrap();

    // Start server.
    ControlThread::run_in_place(config)
}
