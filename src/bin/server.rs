use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let use_colors = atty::is(atty::Stream::Stdout);
    let layer = tracing_tree::HierarchicalLayer::default()
        .with_writer(tracing_subscriber::fmt::writer::TestWriter::default())
        .with_indent_lines(true)
        .with_ansi(use_colors)
        .with_targets(true)
        .with_indent_amount(2);

    let _guard = tracing_subscriber::registry().with(layer).set_default();

    tracing::info!("Starting server");

    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::new(127, 0, 0, 1), 0))
        .await
        .unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

    let (tcp_stream, _) = listener.accept().await.unwrap();
    let mut h2_conn = h2::server::Builder::new()
        .max_concurrent_streams(20_000)
        .handshake::<_, bytes::Bytes>(tcp_stream)
        .await
        .unwrap();

    while let Some(res) = h2_conn.accept().await {
        let (_req, mut send_response) = res.unwrap();

        send_response.send_response(http::Response::default(), true).unwrap();
    }
}
