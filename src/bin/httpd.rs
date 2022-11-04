use std::path::PathBuf;

use clipstash::{data::AppDatabase, web::renderer::Renderer};
use dotenv::dotenv;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn runtime");

    let handle = rt.handle().clone();

    rt.block_on(async move {
        let renderer = Renderer::new(opt.template_directory);
        let database = AppDatabase::new(&opt.connection_string).await;

        let config = clipstash::RocketConfig { renderer, database };
        clipstash::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server");
    });
}
