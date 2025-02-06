mod args;

#[tokio::main]
async fn main() {
    use clap::{CommandFactory, Parser};
    use tracing::{error, info};

    // Parse command-line arguments.
    let opts = crate::args::Args::parse();

    // If user is requesting completions, return them and exit.
    if let Some(shell) = opts.completions {
        clap_complete::generate(
            shell,
            &mut crate::args::Args::command(),
            "rust-template",
            &mut std::io::stdout(),
        );

        return;
    }

    // Setup tracing.
    let _log_guard = toolbox::tracing::setup_tracing("rust-template", opts.logs.as_deref());

    // Setup Continuum standard panic handling.
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!(?panic_info, "Application panic");

        default_panic(panic_info);
    }));

    // Parse .env if it exists.
    match dotenvy::dotenv() {
        Ok(_) | Err(dotenvy::Error::Io(_)) => {}
        Err(err) => panic!("Failed to parse .env file; err={err}"),
    }

    // Log build information.
    toolbox::log_build_info!();

    // Start server.
    let cxl = tokio_util::sync::CancellationToken::new();
    let cxl_child = cxl.clone();
    let mut handle = tokio::spawn(async move { cxl_child.cancelled().await });

    // Wait for server exit or SIGINT.
    tokio::select! {
        res = tokio::signal::ctrl_c() => {
            res.expect("Failed to register SIGINT hook");

            info!("SIGINT caught, stopping server");
            cxl.cancel();

            handle.await.unwrap();
        }
        res = &mut handle => {
            res.unwrap();
        }
    }
}
