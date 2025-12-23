use std::io::{self, BufRead};
// BufRead gives
// # read line, lines() and efficient internal buffering

struct Stats {
    total_requests: u64,
    total_latency: u64,
    error_count: u64,
}

impl Stats {
    fn new() -> Self {
        Stats {
            total_latency: 0,
            total_requests: 0,
            error_count: 0,
        }
    }

    fn update(&mut self, status: u16, latency_ms: u64) {
        self.total_requests += 1;
        self.total_latency += latency_ms;

        if status >= 500 {
            self.error_count += 1;
        }
    }

    fn print(&self) {
        if self.total_requests == 0 {
            println!("No valid requests processed!");
            return;
        }

        let avg_latency = self.total_latency as f64 / self.total_requests as f64;

        println!("Requests: {}", self.total_requests);
        println!("Errors: {}", self.error_count);
        println!("Average Latency (ms): {:.2}", avg_latency);
    }
}

fn parse_line(line: &str) -> Option<(u16, u64)> {
    let mut parts = line.split_whitespace();

    let _timestamp = parts.next()?;
    let _ip        = parts.next()?;
    let _method    = parts.next()?;
    let _path      = parts.next()?;

    let status: u16 = parts.next()?.parse().ok()?;
    let latency: u64 = parts.next()?.parse().ok()?;

    Some((status, latency))
}

fn main() {
    let stdin = io::stdin(); // handle to standard input
    let reader = stdin.lock(); // Reader object 
    // Reader gives a buffered reader with exclusive access to stdin
    // Avoids syscalls for every byte

    let mut stats = Stats::new();

    for line in reader.lines() {
        
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if let Some((status, latency)) = parse_line(&line) {
            stats.update(status, latency);
        }
    }
    stats.print();
}

// internal buffer is a chunk of memory where rust temporarily stores incoming bytes soo it can process them efficiently

// reader.lines() reads raw bytes from input
        // Collects them internally
        // looks for \n
        // when it finds it, it cuts out everything before \n
        // turns it into a string
        // returns it

// ctrl + D -> end of file