use std::process::Command;
use std::process;

//use std::{thread, time::Duration};

fn main() {
    let output = Command::new("ping")
                     .arg("-c")
                     .arg("1")
                     .arg("192.168.1.1")
                     .output()
                     .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    if String::from_utf8_lossy(&output.stdout).contains("1 packets transmitted, 1 received, 0% packet loss") {
        println!("OK");
    } else {
        println!("BAD ping!");
    }

    ctrlc::set_handler(move || {
        println!();
        println!("received Ctrl+C! (write end-time to log)");
        process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    loop{}

    //assert!(output.status.success());
}

// router IP: 192.168.1.1
