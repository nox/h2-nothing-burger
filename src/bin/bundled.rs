use futures::future::join;
use h2_support::frames;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    // let use_colors = atty::is(atty::Stream::Stdout);
    // let layer = tracing_tree::HierarchicalLayer::default()
    //     .with_writer(tracing_subscriber::fmt::writer::TestWriter::default())
    //     .with_indent_lines(true)
    //     .with_ansi(use_colors)
    //     .with_targets(true)
    //     .with_indent_amount(2);

    // let _guard = tracing_subscriber::registry().with(layer).set_default();

    // tracing::info!("Starting repro");

    let (io, mut client) = h2_support::mock::new();
    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel(1);

    let srv = async move {
        let mut h2_conn = h2::server::Builder::new()
            .max_concurrent_streams(2)
            .handshake::<_, bytes::Bytes>(io)
            .await
            .unwrap();

        while let Some(res) = h2_conn.accept().await {
            let (_req, _send_response) = res.unwrap();

            done_rx.recv().await.unwrap();
        }
    };

    let client = async move {
        client.assert_server_handshake().await;

        for i in 0..4 {
            client.send_frame(frames::headers(i * 2 + 1).request("GET", "http://localhost.local/")).await;
            client.send_frame(frames::reset(i * 2 + 1).protocol_error()).await;

            done_tx.send(()).await.unwrap();
        }
    };

    join(srv, client).await;
}
