use serde_json::json;
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use upgrade_witness::{capture::capture_scenarios, config::parse_config};

#[test]
fn real_browser_actions_assertions_masks_and_http_failures() {
    let html = r#"<!doctype html><body style="margin:0"><input id="secret" value="private" style="position:absolute;left:10px;top:10px;width:100px;height:30px"><main style="margin-top:100px"><input id="entry"><button id="open">Open</button><div id="panel" hidden data-state="closed" style="color:green">Panel</div><span id="output"></span><span id="environment"></span></main><script>document.querySelector('#open').onclick=()=>{const p=document.querySelector('#panel');p.hidden=false;p.dataset.state='open'};document.querySelector('#entry').onkeydown=e=>{if(e.key==='Enter')document.querySelector('#output').textContent=e.target.value};document.querySelector('#environment').textContent=[navigator.language,Intl.DateTimeFormat().resolvedOptions().timeZone,matchMedia('(prefers-reduced-motion: reduce)').matches,matchMedia('(prefers-color-scheme: light)').matches].join('|');</script>"#;
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
                    let mut buf = [0; 4096];
                    let n = stream.read(&mut buf).unwrap_or(0);
                    let missing = String::from_utf8_lossy(&buf[..n]).starts_with("GET /missing");
                    let status = if missing { "404 Not Found" } else { "200 OK" };
                    let _ = write!(
                        stream,
                        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{html}",
                        html.len()
                    );
                }
                Err(_) => thread::sleep(Duration::from_millis(10)),
            }
        }
    });
    let out = tempfile::tempdir().unwrap();
    let config=parse_config(&json!({"schemaVersion":1,"timeoutMs":5000,"scenarios":[{"id":"flow","route":"/","viewport":{"width":640,"height":480},"actions":[{"type":"fill","selector":"#entry","value":"hello"},{"type":"press","selector":"#entry","value":"Enter"},{"type":"click","selector":"#open"}],"assertions":[{"id":"output","selector":"#output","kind":"text","expected":"hello"},{"id":"panel","selector":"#panel","kind":"visible","expected":true},{"id":"state","selector":"#panel","kind":"attribute","name":"data-state","expected":"open"},{"id":"color","selector":"#panel","kind":"style","name":"color","expected":"rgb(0, 128, 0)"},{"id":"env","selector":"#environment","kind":"text","expected":"en-US|UTC|true|true"}]},{"id":"http","route":"/missing","screenshot":false,"assertions":[{"id":"no","selector":"body","kind":"text","expected":"none"}]}]}).to_string()).unwrap();
    let captures = capture_scenarios(
        &format!("http://127.0.0.1:{port}"),
        &config,
        out.path(),
        "base",
    );
    stop.store(true, Ordering::Relaxed);
    worker.join().unwrap();
    assert!(captures[0].error.is_none(), "{:?}", captures[0].error);
    assert!(
        captures[0].checks.iter().all(|c| c.passed),
        "{:?}",
        captures[0].checks
    );
    let image = image::open(captures[0].screenshot_path.as_ref().unwrap())
        .unwrap()
        .to_rgba8();
    assert_eq!(image.get_pixel(40, 25).0, [0, 0, 0, 255]);
    assert!(
        captures[1]
            .error
            .as_ref()
            .unwrap()
            .contains("Page navigation")
    );
    assert!(captures[1].checks.is_empty());
    assert!(captures[1].screenshot_path.is_none());
}
