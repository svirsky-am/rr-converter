use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::net::{TcpStream, UdpSocket};

#[test]
fn test_tcp_client_server() {
    let port = 12345;
    let server_addr = format!("127.0.0.1:{}", port);

    // Start the server as a child process
    let mut run_server = Command::new("cargo")
        .args(["run", "-p", "streaming_quotes_project",
        // "--features",  "'sqlite random logging'",
        "--bin", "quote_server", "--", &port.to_string()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server");
    thread::sleep(Duration::from_millis(100));

        // Spawn a thread to read and print server stdout in real time
    let server_stdout = run_server.stdout.take().unwrap();
    let server_stderr = run_server.stderr.take().unwrap();

    // println!("[test SERVER STDOUT]");


    // std::thread::spawn(move || {
    //     let reader = std::io::BufReader::new(server_stdout);
    //     for line in std::io::BufRead::lines(reader) {
    //         match line {
    //             Ok(text) => println!("[SERVER STDOUT] {}", text),
    //             Err(e) => eprintln!("[SERVER STDOUT READ ERROR] {:?}", e),
    //         }
    //     }
    // });

    std::thread::spawn(move || {
        let stderr_reader = std::io::BufReader::new(server_stderr);
        for line in std::io::BufRead::lines(stderr_reader) {
            if let Ok(line) = line {
                eprintln!("[SERVER STDERR] {}", line);
            }
        }
    });

    // let stdout = String::from_utf8_lossy(&run_server.stdout);
    // Give the server time to bind
    thread::sleep(Duration::from_millis(100));



    let socket = UdpSocket::bind(&server_addr).is_ok();

    println!("MetricsSender создан на адресе {}", server_addr);
    
    // assert!(
    //         TcpStream::connect(&server_addr).is_ok(),
    //         "Server did not start listening on {}",
    //         server_addr
    // );

    // // Verify server is actually listening
    // assert!(
    //     TcpStream::connect(&server_addr).is_ok(),
    //     "Server did not start listening on {}",
    //     server_addr
    // );


    // // Run the client
    // let output = Command::new("cargo")
    // .args([
    //     "run",
    //     "--bin",
    //     "tcp-client",
    //     "--",
    //     &port.to_string(),
    //     "PING",
    // ])
    // .output()
    // .expect("Failed to run client");
    
    // Run the client
    let output = Command::new("cargo")
        .args([
            "run", "-p", "streaming_quotes_project",
            "--features",  "'sqlite random logging'",
            "--bin",
            "quote_client",
            "--",
            "127.0.0.1:8080",
            "1000"
            // &port.to_string(),
            // "PING",
        ])
        .output()
        .expect("Failed to run client");

    // Check client output (adjust based on your client's behavior)
    thread::sleep(Duration::from_millis(8000));
    

    // let run_thread = std::thread::spawn(move || {
    //     let mut stdout_reader = std::io::BufReader::new(server_stdout);
    //     let mut output = String::new();
    //     // let result_out = reader.read_to_string(&mut output);
    //     // for line in std::io::BufRead::lines(reader) {
    //     println!("testest");
    //     let read_result = stdout_reader.read_to_string(&mut output);
    //     println!("[SERVER STDOUT2] {}", output.to_string());
    //         match read_result {
    //             Ok(_) => {
    //                                 println!("[SERVER STDOUT2] {}", &output);
    //                                 let check_contant = output.to_string();
    //                                 assert!(check_contant.contains("PONG") || check_contant.contains("Received"), "Client did not get expected response: {}", check_contant);
    //                                 },
    //             Err(e) => eprintln!("[SERVER STDOUT READ ERROR] {:?}", e),
    //         }
    //     // }
    // });

    let (sender, receiver) = mpsc::channel::<bool>();
    let run_thread  =     std::thread::spawn(move || {
        let reader = std::io::BufReader::new(server_stdout);
        for line in std::io::BufRead::lines(reader) {
            match line {
                Ok(text) => {
                    println!("[SERVER STDOUT] {}", text);
                    if text.contains("[#005]") {
                        let _ = sender.send(true); // Signal: found!
                        return; // Optional: exit early
                    }
                }
                Err(e) => {
                    eprintln!("[SERVER STDOUT READ ERROR] {:?}", e);
                    let _ = sender.send(false);
                    return;
                }
            }
        }
        let _ = sender.send(false);
    });

    thread::sleep(Duration::from_millis(1000));
    match receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(found) => {
            assert!(found, "Server stdout did not contain 'TEST_TEST'");
        }
        Err(_) => {
            // Timeout: assume not found
            panic!("Timeout waiting for server output containing 'TEST_TEST'");
        }
    }

    let _ = run_thread.join().is_ok();

    

    // Kill the server
    run_server.kill().ok();
}