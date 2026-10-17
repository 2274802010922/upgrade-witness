use serde_json::json;
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use upgrade_witness::{capture::capture_scenarios, config::parse_config};

struct FixtureServer {
    url: String,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl FixtureServer {
    fn new(html: &'static str) -> Self {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        listener.set_nonblocking(true).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let worker = thread::spawn(move || {
            while !flag.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));
                        let mut request = [0; 4096];
                        let _ = stream.read(&mut request);
                        let _ = write!(
                            stream,
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{html}",
                            html.len()
                        );
                    }
                    Err(_) => thread::sleep(Duration::from_millis(10)),
                }
            }
        });
        Self {
            url: format!("http://127.0.0.1:{port}"),
            stop,
            worker: Some(worker),
        }
    }
}

impl Drop for FixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take() {
            worker.join().unwrap();
        }
    }
}

#[test]
fn zero_area_elements_are_not_reported_visible() {
    let server = FixtureServer::new(
        r#"<!doctype html><body><div id="target" style="display:block;width:0;height:0;overflow:hidden"></div></body>"#,
    );
    let output = tempfile::tempdir().unwrap();
    let config = parse_config(&json!({
        "schemaVersion":1,"timeoutMs":1000,
        "scenarios":[{"id":"zero-area","route":"/","screenshot":false,
            "assertions":[{"id":"visibility","selector":"#target","kind":"visible","expected":false}]}]
    }).to_string()).unwrap();
    let captures = capture_scenarios(&server.url, &config, output.path(), "head");
    assert!(captures[0].error.is_none(), "{:?}", captures[0].error);
    assert_eq!(
        captures[0].checks[0].actual,
        json!(false),
        "a zero-area element has no visible box; treating it as visible can falsely pass a collapsed UI"
    );
}

#[test]
fn automatic_input_masks_cover_modal_top_layer() {
    let server = FixtureServer::new(
        r#"<!doctype html><body style="margin:0"><dialog id="modal" style="position:fixed;left:20px;top:20px;margin:0;padding:0;border:0;width:200px;height:100px;background:white"><input id="private" value="synthetic-private-value" style="position:absolute;left:10px;top:10px;width:100px;height:30px;border:0;padding:0;background:rgb(255,0,0)"></dialog><script>document.querySelector('#modal').showModal();document.activeElement.blur()</script></body>"#,
    );
    let output = tempfile::tempdir().unwrap();
    let config = parse_config(
        &json!({
            "schemaVersion":1,"timeoutMs":1000,
            "scenarios":[{"id":"modal","route":"/","viewport":{"width":640,"height":480},
                "assertions":[{"id":"open","selector":"#modal","kind":"visible","expected":true}]}]
        })
        .to_string(),
    )
    .unwrap();
    let captures = capture_scenarios(&server.url, &config, output.path(), "base");
    assert!(captures[0].error.is_none(), "{:?}", captures[0].error);
    let screenshot = image::open(captures[0].screenshot_path.as_ref().unwrap())
        .unwrap()
        .to_rgba8();
    assert_eq!(
        screenshot.get_pixel(120, 50).0,
        [0, 0, 0, 255],
        "a document-level overlay must not leave a top-layer input unmasked"
    );
}
