use clap::Parser;
use tracing_subscriber::fmt::time::ChronoLocal;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub json_path: String,

    pub output_dir: Option<String>,
}

impl Args {
    pub fn output_path(&self) -> &str {

        self.output_dir.as_deref().unwrap_or("./client")
    }

    pub fn init_tracing(&self) {

        tracing_subscriber::fmt()
            .with_timer(ChronoLocal::rfc_3339())
            .init();
    }
}
