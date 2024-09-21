use tokio::net::{ TcpListener, TcpStream};
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use clap::Parser;
use std::io::{self, Write};
use std::process::exit;

#[derive(Parser)]
struct Local {
    #[arg(short = 'l', help = "address to listening")]
    local: String,

    #[arg(short = 'p', help = "port to listening")]
    port: String,
}

async fn handle_connection(mut socket: TcpStream) {
    loop {
        let mut command = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).expect("failed to read line");
        // let command = command as u8;
        if let Err(e) = socket.write_all(command.as_bytes()).await {
            eprintln!("Failed to send message: {}",e);
        }
    
        let mut buffer = [0;1024];
        let read_bytes = socket.read(&mut buffer).await.unwrap();

        if read_bytes == 0 {
            println!("connection closed by peer");
            exit(0);
            // break;
        }else {
            println!("Receive from nc(NetCat): {}",String::from_utf8_lossy(&buffer[..read_bytes]));   
        }

    }

}

async fn handle_opretion(local: String, port: String) {
    let addr = format!("{}:{}",local, port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}",addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }

}


#[tokio::main]
async fn main(){
    let arg = Local::parse();
    let local_value = arg.local;
    let port = arg.port;
    
    if !local_value.is_empty() {
        handle_opretion(local_value, port).await;
    }else {
        println!("Local value is empty");
    }

}