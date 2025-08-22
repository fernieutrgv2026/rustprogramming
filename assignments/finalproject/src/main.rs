use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use reqwest;

// definition of configuration
struct Config {
    num_threads: usize,
    request_timeout: Duration,
    max_retries: usize,
}

// website info update
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn main() {
    // example of configuration
    let config = Config {
        num_threads: 10,
        request_timeout: Duration::from_secs(5),
        max_retries: 3,
    };

    // url list
    let websites = vec![
        "https://example.com".to_string(),
        "https://rust-lang.org".to_string(),
    ];

    let (tx, rx) = mpsc::channel::<String>();

    // collecting results with this
    let results: Arc<Mutex<Vec<WebsiteStatust>>> = Arc::new(Mutex::new(Vec::new()));

    // some threads
    let mut handles = Vec::new();
    for _ in 0..config.num_threads {
        let rx = rx.clone();
        let results = Arc::clone(&results);
        let request_timeout = config.request_timeout;
        let max_retries = config.max_retries;

        let handle = thread::spawn(move || {
            // creating reqwest
            let client = reqwest::blocking::Client::builder()
                .timeout(request_timeout)
                .build()
                .unwrap();

            while let Ok(url) = rx.recv() {
                let mut attempt = 0;
                let mut success = false;
                let mut last_error = None;
                let mut status_code = None;
                let mut response_time = Duration::from_secs(0);

                while attempt < max_retries {
                    let start = Instant::now();
                    match client.get(&url).send() {
                        Ok(resp) => {
                            response_time = start.elapsed();
                            status_code = Some(resp.status().as_u16());
                            success = true;
                            break;
                        }
                        Err(e) => {
                            last_error = Some(e.to_string());
                            attempt += 1;
                        }
                    }
                }

                // store result
                let result = CheckResult {
                    url: url.clone(),
                    status_code,
                    response_time,
                    error: if success { None } else { last_error },
                };

                results.lock().unwrap().push(result);
            }
        });
        handles.push(handle);
    }

    // sending urls to threads
    for url in &websites {
        tx.send(url.clone()).unwrap();
    }
    drop(tx); // closed the sender

    // after threads finish
    for handle in handles {
        handle.join().unwrap();
    }

    // print results
    let results = results.lock().unwrap();
    for res in results.iter() {
        println!(
            "URL: {}, Status: {:?}, Time: {:?}, Error: {:?}",
            res.url, res.status_code, res.response_time, res.error
        );
    }
}