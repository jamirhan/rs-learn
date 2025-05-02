use tokio::io::AsyncReadExt;
use tokio::runtime;
use tokio::net;

fn main() {
    let r = runtime::Builder::new_multi_thread().enable_io().worker_threads(2).build().unwrap();
    r.block_on(
        async {
            let socket = net::TcpListener::bind("127.0.0.1:5555").await.unwrap();
            loop {
                let mut buf: [u8; 128] = [0; 128];
                let (mut stream, _) = socket.accept().await.unwrap();
                loop {
                    let read = stream.read(&mut buf).await.unwrap();
                    if read == 0 {
                        break;
                    }
                    let read_str: String = String::from_utf8_lossy(&buf).parse().unwrap();
                    print!("{read_str}")
               }
            }
        }
    );
}
