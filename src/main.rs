use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use std::str;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let socket = UnixStream::connect("/tmp/bspwm_0_0-socket").await?;
    let (mut rd, mut wr) = io::split(socket);

    let _write_task = tokio::spawn(async move {
        wr.write_all(b"query\x00-M\x00--names\x00").await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];
    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        print!("{}",  match str::from_utf8(&buf[..n]) {
            Ok(s) => s,
            _ => "Error"
        });
    }

    Ok(())
}
