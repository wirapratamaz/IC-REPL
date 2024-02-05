use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main(){
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout);
    
    loop {
        stdout.write_all("IC REPL> ".as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();

        match reader.next_line().await {
            Ok(Some(line)) => {
                if line.eq("exit") {
                    break;
                }

                // example of evaluating the command
                let result = process_command(line).await;
                println!("Command received: {}", result);
            },
            Ok(None) => {},
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

async fn process_command(line: String) -> String {
    // evaluate the command
    format!("You entered: {}", line)
}