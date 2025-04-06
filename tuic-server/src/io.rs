use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BUFFER_SIZE: usize = 8 * 1024;

pub async fn exchange_tcp(
    a: &mut tuic_quinn::Connect,
    b: &mut tokio::net::TcpStream,
) -> (usize, usize, Option<eyre::Error>) {
    let mut a2b = [0u8; BUFFER_SIZE];
    let mut b2a = [0u8; BUFFER_SIZE];

    let mut a2b_num = 0;
    let mut b2a_num = 0;

    let mut last_err = None;

    loop {
        tokio::select! {
            a2b_res = a.recv.read(&mut a2b) => match a2b_res {
                Ok(Some(num)) => {
                    a2b_num += num;
                    if let Err(err) = b.write_all(&a2b[..num]).await {
                        last_err = Some(err.into());
                        break;
                    }
                },
                // EOF
                Ok(None) => {
                    break;
                },
                Err(err) => {
                    last_err = Some(err.into());
                    break;
                }
            },

            b2a_res = b.read(&mut b2a) => match b2a_res {
                Ok(num) => {
                    // EOF
                    if num == 0 {
                        break;
                    }
                    b2a_num += num;
                    if let Err(err) = a.send.write_all(&b2a[..num]).await {
                        last_err = Some(err.into());
                        break;
                    }
                },
                Err(err) => {
                    last_err = Some(err.into());
                    break;
                },
            }

        }
    }

    (a2b_num, b2a_num, last_err)
}
