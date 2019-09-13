extern crate haproxy_dataplane_client;
extern crate reqwest;
use haproxy_dataplane_client::requests::*;

#[test]
fn test_basic_request() {
    let b = Remote::new()
        .domain("localhost")
        .port(5555)
        .version(4)
        .credentials("dataplaneapi", "mypassword");
    let a = Frontend::new()
    .name(String::from("blablablabla"))
    .client_timeout(0)
    .clitcpka(State::Enabled)
    .contstats(State::Enabled)
    .default_backend("app".to_string())
    .dontlognull(State::Enabled)
    .http_use_htx(State::Disabled)
    .http_connection_mode(HttpConnectionMode::HttpKeepAlive)
    .http_keep_alive_timeout(0)
    .http_pretend_keepalive(State::Enabled)
    .http_request_timeout(0)
    .httplog(true);

    match a.add() {
        Ok(a) => { println!("{}", a.send(&b).unwrap().text().unwrap()) }
        Err(_) => {} 
    }

    let c = Global::new();
    match c.get() {
        Ok(c) => { println!("{}", c.send(&b).unwrap().text().unwrap()) }
        Err(_) => {}
    }
    let b = b.update_tracker();
    let (_header, vers) = b.get_tracker();
    println!("version is now {}", vers);
}