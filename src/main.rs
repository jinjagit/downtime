//use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use std::{thread, time::Duration};
use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
//use chrono::offset::LocalResult;

//use std::process;

pub struct PingEvent {
    url: String,
    status: String,
    stdout: String,
    stderr: String,
    ok: bool,
}

impl Default for PingEvent {
    fn default() -> PingEvent {
        PingEvent {
            url: String::from("unintialized"),
            status: String::from("unintialized"),
            stdout: String::from("unintialized"),
            stderr: String::from("unintialized"),
            ok: true,
        }
    }
}

impl PingEvent {
    fn ping(&mut self) -> bool {
        let result = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(&self.url)
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&result.stdout);
        let error = String::from_utf8_lossy(&result.stderr);

        if output.contains("1 packets transmitted, 1 received, 0% packet loss") {
            self.ok = true;
        } else {
            match result.status.code() {
                Some(code) => self.status = code.to_string(),
                None => self.status = String::from("Process terminated by signal"),
            }

            self.stdout = String::from(output);
            self.stderr = String::from(error);
            self.ok = false;
        }

        self.ok
    }
}

fn main() {
    let mut url1: PingEvent = PingEvent {
        url: String::from("google.com"),
        ..Default::default()
    };
    //let mut router: PingEvent = PingEvent {url: String::from("192.168.1.1"), ..Default::default()};

    write_line(&format!("START: {}", time_now()));

    println!("date / time: {:?}", time_now());

    let test: bool = url1.ping();

    // println!("url: {}", url1.url);
    // println!("status: {}", url1.status);
    // println!("stdout: {}", url1.stdout);
    // println!("stderr: {}", url1.stderr);
    // println!("ok: {}", url1.ok);
    println!("test: {}", test);

    // ctrlc::set_handler(move || {
    //     println!();
    //     println!("received Ctrl+C! (write end-time to log)");
    //     process::exit(1);
    // })
    // .expect("Error setting Ctrl-C handler");

    loop {
        println!("running...");

        thread::sleep(Duration::from_secs(1));
    }

}

fn time_now() -> String {
    let utc: String = format!("{}", Utc::now().to_rfc2822());

    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let sys_time: String = format!("{}", since_the_epoch.as_secs());

    format!("{} system time: {}", utc, sys_time)
}

fn write_line(input: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("downtime.log")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", input) {
        println!("Couldn't write to file: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // NOTE: This test will fail if router connection down/off/fails.
    fn ping_test() {
        let mut router: PingEvent = PingEvent {
            url: String::from("192.168.1.1"),
            ..Default::default()
        };
        let test: bool = router.ping();

        assert_eq!(test, true);
        assert_eq!(router.ok, true);

        let mut url1: PingEvent = PingEvent {
            url: String::from("not_a_valid_url.com"),
            ..Default::default()
        };
        let test: bool = url1.ping();

        assert_eq!(test, false);
        assert_eq!(url1.ok, false);
        assert_eq!(url1.status, "2");
        assert_eq!(
            url1.stderr,
            "ping: not_a_valid_url.com: Name or service not known\n"
        );
    }
}
