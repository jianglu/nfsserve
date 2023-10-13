use nfsserve::mirrorfs::MirrorFS;
use nfsserve::tcp::*;

#[tokio::main]
async fn main() {
    let mut args = pico_args::Arguments::from_env();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(std::io::stderr)
        .init();

    let ip: String = args.opt_value_from_str("--ip").unwrap().unwrap();
    let dir: String = args.opt_value_from_str("--dir").unwrap().unwrap();

    let listener = NFSTcpListener::bind(
        &ip, //
        MirrorFS::new(dir.into()),
    )
    .await
    .unwrap();

    listener.handle_forever().await.unwrap();
}

// Test with

// mount -t nfs -o nolocks,vers=3,tcp,port=12000,mountport=12000,soft 127.0.0.1:/ mnt/
