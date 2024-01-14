use std::io;
use std::net::{SocketAddr, TcpListener};

pub fn start( addr: SocketAddr){
    let listener = TcpListener::bind(addr).unwrap();
    loop {
        if let Ok((stream,peer_addr)) = listener.accept(){
            println!("accept {}", peer_addr);
            let _ = handle_conn(stream);
        }
    }
}

fn handle_conn(mut stream: TcpListener) -> io::Result<()>{
    let mut buffer:[u8; 1024] = [1, 1024];
    let n = stream.read(&mut buffer)?;
    println!("{}", String::from_utf8(buffer[..n].to_vec()).unwrap());
    Ok(())
}

