use std::any::Any;
use std::time::Duration;

use futures::StreamExt;
use futures::stream::FuturesUnordered;
use tokio::runtime::Runtime;
use tokio::signal::unix::SignalKind;
use toolbox::shutdown::Shutdown;
use toolbox::tokio::NamedTask;
use tracing::{error, info};

use crate::config::Config;
use crate::worker_thread::WorkerThread;

pub(crate) struct ControlThread {
    shutdown: Shutdown,
    threads: FuturesUnordered<NamedTask<Result<(), Box<dyn Any + Send>>>>,
}

impl ControlThread {
    pub(crate) fn run_in_place(config: Config) -> Result<(), Box<dyn Any + Send>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let server = runtime.block_on(ControlThread::setup(&runtime, config));

        runtime.block_on(server.run())
    }

    async fn setup(runtime: &Runtime, config: Config) -> Self {
        let shutdown = Shutdown::new();
        let mut threads = Vec::default();

        // Setup metrics.
        let nats_client = Box::leak(Box::new(
            metrics_nats_exporter::async_nats::connect(config.nats_servers)
                .await
                .expect("NATS Client Connect"),
        ));
        threads.push(
            metrics_nats_exporter::install(
                shutdown.token.clone(),
                metrics_nats_exporter::Config {
                    interval_min: Duration::from_millis(50),
                    interval_max: Duration::from_millis(1000),
                    metric_prefix: Some(format!("metric.rust-template.{}", config.host)),
                },
                nats_client,
            )
            .unwrap(),
        );

        // Setup app threads.
        threads.push(WorkerThread::spawn(shutdown.clone()));

        // Use tokio to listen on all thread exits concurrently.
        let threads = threads
            .into_iter()
            .map(|thread| {
                let name = thread.thread().name().unwrap().to_string();
                info!(name, "Thread spawned");

                NamedTask::new(runtime.spawn_blocking(move || thread.join()), name)
            })
            .collect();

        ControlThread { shutdown, threads }
    }

    async fn run(mut self) -> Result<(), Box<dyn Any + Send>> {
        let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
        let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt()).unwrap();

        let mut exit = tokio::select! {
            () = self.shutdown.cancelled() => Ok(()),

            _ = sigterm.recv() => {
                info!("SIGTERM caught, stopping server");

                Ok(())
            },
            _ = sigint.recv() => {
                info!("SIGINT caught, stopping server");

                Ok(())
            },
            opt = self.threads.next() => {
                let (name, res) = opt.unwrap();
                error!(%name, ?res, "Thread exited unexpectedly");

                res.unwrap().and_then(|()| Err(Box::new("Thread exited unexpectedly")))
            }
        };

        // Trigger shutdown.
        self.shutdown.shutdown();

        // Wait for all threads to exit, reporting the first error as the ultimate
        // error.
        while let Some((name, res)) = self.threads.next().await {
            info!(%name, ?res, "Thread exited");
            exit = exit.and(res.unwrap());
        }

        exit
    }
}
