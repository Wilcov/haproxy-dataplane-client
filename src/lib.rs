/*
TODO:
- function aliassen aanmaken per struct
- send fn is 100% gekopieerd tussen endpoints, functionaliteit kan beter in root gestopt worden waarschijnlijk
*/

pub mod requests {
    extern crate reqwest;
    extern crate serde;
    extern crate serde_json;
    use reqwest::{Client, Response};
    use serde::{Serialize, Deserialize};

    pub type Result<T> = std::result::Result<T, String>;

    pub trait Endpoint {
        fn list(self) -> Result<Self> where Self: Sized;
        fn post(self) -> Result<Self> where Self: Sized;
        fn send<T: Query + ApiRoot>(self, remote: &T) -> Result<Response>;
        fn get(self) -> Result<Self> where Self: Sized;
        /*
        fn delete(&self) -> Result<Response>;
        fn replace(&self) -> Result<Response>;
        */
    }

    pub trait Query {
        fn has_tracker(&self) -> bool;
        fn get_tracker(&self) -> (String, String);
        fn update_tracker(self) -> Self;
        fn force_reload(&self) -> (QueryHeader, bool);
    }
    
    pub trait ApiRoot {
        fn domain(self, domain: &str) -> Self;
        fn port(self, port: usize) -> Self;
        fn credentials(self, username: &str, password: &str) -> Self;
        fn version(self, version: usize) -> Self;
        fn build_url(&self, endpoint: &str) -> String;
        fn get_credentials(&self) -> Option<(&str, &str)>;
    }

    pub enum QueryHeader {
        TransactionId,
        Version,
        ForceReload
    }

    #[derive(Copy, Clone)]
    pub enum Method {
        List,
        Get,
        Post,
        Put,
        Del,
    }

    pub struct Remote {
        domain: String,
        port: usize,
        version: Option<usize>,
        credentials: Option<(String, String)>
    }

    impl Remote {
        pub fn new() -> Self {
            Remote {
                domain: String::from(""),
                port: 0000,
                version: None,
                credentials: None
            }
        }

    }

    impl ApiRoot for Remote {
        fn domain(mut self, domain: &str) -> Self {
            self.domain = String::from(domain);
            self
        }

        fn port(mut self, port: usize) -> Self {
            self.port = port;
            self
        }

        fn version(mut self, version: usize) -> Self {
            self.version = Some(version);
            self
        }

        fn credentials(mut self, username: &str, password: &str) -> Self {
            self.credentials = Some((username.to_string(), password.to_string()));
            self
        }

        fn build_url(&self, endpoint: &str) -> String {
            String::from("") + "http://" + &self.domain + ":" + &self.port.to_string() + "/v1" + endpoint
        }

        fn get_credentials(&self) -> Option<(&str, &str)> {
            if let Some((username, password)) = &self.credentials {
                Some((username, password))
            } else {
                None
            }
        }
    }

    impl Query for Remote {
        fn has_tracker(&self) -> bool { 
            if self.version.is_none() {
                false
            } else {
                true
            }
        }

        fn get_tracker(&self) -> (String, String) {
            ("version".to_string(), self.version.unwrap().to_string())
        }

        fn update_tracker(mut self) -> Self {
            let g = Global::new();
            match g.get() {
                Ok(g) => {
                    let resp: Resp = g.send(&self).unwrap().json().unwrap();
                    self = self.version(resp._version);
                }
                Err(e) => print!("{}", e.to_string())
            }
            self
        }

        fn force_reload(&self) -> (QueryHeader, bool) {
            (QueryHeader::ForceReload, true)
        }
    }

    #[derive(Deserialize)]
    struct Resp {
        _version: usize,
        data : Global
    }

    pub struct Transaction<'r> {
        transaction_id: Option<String>,
        remote: &'r mut Remote
    }

    impl<'r> Query for Transaction<'r> {
        fn has_tracker(&self) -> bool {
            if self.transaction_id.is_none() {
                false
            } else {
                true
            }
        }

        fn get_tracker(&self) -> (String, String) {
            ("transaction_id".to_string(), self.transaction_id.clone().unwrap())
        }

        fn update_tracker(self) -> Self {
            self
        }

        fn force_reload(&self) -> (QueryHeader, bool) {
            (QueryHeader::ForceReload, false)
        }
    }

    impl<'r> ApiRoot for Transaction<'r> {
        fn domain(mut self, domain: &str) -> Self {
            self.remote.domain = String::from(domain);
            self
        }

        fn port(mut self, port: usize) -> Self {
            self.remote.port = port;
            self
        }

        fn version(mut self, version: usize) -> Self {
            self.remote.version = Some(version);
            self
        }

        fn credentials(mut self, username: &str, password: &str) -> Self {
            self.remote.credentials = Some((username.to_string(), password.to_string()));
            self
        }

        fn build_url(&self, endpoint: &str) -> String {
            String::from("") + "http://" + &self.remote.domain + ":" + &self.remote.port.to_string() + "/" + endpoint
        }

        fn get_credentials(&self) -> Option<(&str, &str)> {
            if let Some((username, password)) = &self.remote.credentials {
                Some((username, password))
            } else {
                None
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct Global {
        #[serde(skip)]
        url: Option<String>,
        #[serde(skip)]
        method: Option<Method>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cpu_maps: Option<Vec<CpuMap>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        daemon: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        external_check: Option<bool>,
        master_worker: Option<bool>,
        maxconn: Option<usize>,
        nbcproc: Option<isize>,
        nbthread: Option<isize>,
        pidfile: Option<String>,
        runtime_apis: Option<Vec<RuntimeApi>>,
        ssl_default_bind_ciphers: Option<String>,
        ssl_default_bind_options: Option<String>,
        stats_timeout: Option<isize>,
        tune_ssl_default_dh_param: Option<isize>
    }

    impl Global {
        pub fn new() -> Global {
            Global {
                url: None,
                method: None,
                cpu_maps: None,
                daemon: None,
                external_check: None,
                master_worker: None,
                maxconn: None,
                nbcproc: None,
                nbthread: None,
                pidfile: None,
                runtime_apis: None,
                ssl_default_bind_ciphers: None,
                ssl_default_bind_options: None,
                stats_timeout: None,
                tune_ssl_default_dh_param: None
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct CpuMap {
        #[serde(skip_serializing_if = "Option::is_none")]
        cpu_set: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        process: Option<String>
    }

    #[derive(Serialize, Deserialize)]
    pub struct RuntimeApi {
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "camelCase")]
        expose_fd_listeners: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename(serialize = "lowercase", deserialize = "PascalCase"))]
        level: Option<UserLevel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        process: Option<String>
    }

    #[derive(Serialize, Deserialize)]
    enum UserLevel {
        User,
        Operator,
        Admin
    }

    impl Endpoint for Global {
        fn list(self) -> Result<Global> {
            Err("The global endpoint does not implement the list function.".to_string())
        }

        fn get(mut self) -> Result<Global> {
            self.url = Some(String::from("/services/haproxy/configuration/global"));
            self.method = Some(Method::Get);
            Ok(self)
        }

        fn post(self) -> Result<Global> {
            Err("The global endpoint does not implement the post function.".to_string())
        }

        fn send<T: Query + ApiRoot>(self, remote: &T) -> Result<Response> {
            if let Some(method) = self.method {
                let client = Client::new();
                let mut resp;
                match method {
                    Method::Get => { 
                        resp = client.get(&remote.build_url(&self.url.unwrap())); 
                    },
                    Method::Post => {
                        let url = &self.url.clone().unwrap();
                        resp = client.post(&remote.build_url(url))
                            .json(&self);
                    },
                    _ => {
                        resp = client.post(&remote.build_url(&self.url.unwrap())); 
                    }
                }
                if let Some((username, password)) = remote.get_credentials() {
                    resp = resp.basic_auth(username, Some(password));
                }
                if remote.has_tracker() {
                    resp = resp.query(&[(remote.get_tracker())]);
                }
                match resp.send() {
                    Ok(response) => Ok(response),
                    Err(e) => Err(e.to_string())
                }
            } else {
                Err(String::from("Method hasn't been set yet"))
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum State {
        Enabled,
        Disabled
    }
    
    #[derive(Serialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum HttpConnectionMode {
        HttpTunnel,
        Httpclose,
        HttpServerClose,
        HttpKeepAlive
    }

    #[derive(Serialize)]
    pub enum Mode {
        Http,
        Tcp,
        Health
    }

    #[derive(Serialize)]
    pub struct Frontend { 
        #[serde(skip)]
        url: Option<String>,
        #[serde(skip)]
        method: Option<Method>,
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        clflog: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_timeout: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        clitcpka: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        contstats: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default_backend: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dontlognull: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "kebab-case")]
        http_use_htx: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        http_connection_mode: Option<HttpConnectionMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        http_keep_alive_timeout: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        http_pretend_keepalive: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        http_request_timeout: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        httplog: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        log_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        log_format_sd: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        log_seperate_errors: Option<State>,
        #[serde(skip_serializing_if = "Option::is_none")]
        log_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maxconn: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<Mode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tcplog: Option<bool>,
    }

    impl Frontend {
        pub fn name(mut self, b: String) -> Frontend {
            self.name = Some(b);
            self
        }

        pub fn clflog(mut self, b: bool) -> Frontend {
            self.clflog = Some(b);
            self
        }

        pub fn client_timeout(mut self, t: usize) -> Frontend {
            self.client_timeout = Some(t);
            self
        }

        pub fn clitcpka(mut self, b: State) -> Frontend {
            self.clitcpka = Some(b);
            self
        }

        pub fn contstats(mut self, b: State) -> Frontend {
            self.contstats = Some(b);
            self
        }

        pub fn default_backend(mut self, b: String) -> Frontend {
            self.default_backend = Some(b);
            self
        }

        pub fn dontlognull(mut self, b: State) -> Frontend {
            self.dontlognull = Some(b);
            self
        }

        pub fn http_use_htx(mut self, b: State) -> Frontend {
            self.http_use_htx = Some(b);
            self
        }

        pub fn http_connection_mode(mut self, b: HttpConnectionMode) -> Frontend {
            self.http_connection_mode = Some(b);
            self
        }

        pub fn http_keep_alive_timeout(mut self, t: usize) -> Frontend {
            self.http_keep_alive_timeout = Some(t);
            self
        }

        pub fn http_pretend_keepalive(mut self, b: State) -> Frontend {
            self.http_pretend_keepalive = Some(b);
            self
        }

        pub fn http_request_timeout(mut self, t: usize) -> Frontend {
            self.http_request_timeout = Some(t);
            self
        }

        pub fn httplog(mut self, b: bool) -> Frontend {
            self.httplog = Some(b);
            self
        }

        pub fn log_format(mut self, b: String) -> Frontend {
            self.log_format = Some(b);
            self
        }

        pub fn log_format_sd(mut self, b: String) -> Frontend {
            self.log_format_sd = Some(b);
            self
        }

        pub fn log_seperate_errors(mut self, b: State) -> Frontend {
            self.log_seperate_errors = Some(b);
            self
        }

        pub fn log_tag(mut self, b: String) -> Frontend {
            self.log_tag = Some(b);
            self
        }

        pub fn maxconn(mut self, t: usize) -> Frontend {
            self.maxconn = Some(t);
            self
        }

        pub fn mode(mut self, m: Mode) -> Frontend {
            self.mode = Some(m);
            self
        }

        pub fn tcplog(mut self, b: bool) -> Frontend {
            self.tcplog = Some(b);
            self
        }

        pub fn get() {

        }

        pub fn add(self) -> Result<Frontend> {
            self.post()
        }
        pub fn new() -> Frontend {
            Frontend { 
                url: None,
                method: None,
                name: None,
                clflog: None,
                client_timeout: None,
                clitcpka: None,
                contstats: None,
                dontlognull: None,
                default_backend: None,
                http_use_htx: None,
                http_connection_mode: None,
                http_keep_alive_timeout: None,
                http_pretend_keepalive: None,
                http_request_timeout: None,
                httplog: None,
                log_format: None,
                log_format_sd: None,
                log_seperate_errors: None,
                log_tag: None,
                maxconn: None,
                mode: None,
                tcplog: None
            }
        }
    }

    impl Endpoint for Frontend {
        fn list(mut self) -> Result<Frontend> { 
            self.url = Some(String::from("/services/haproxy/configuration/frontends"));
            self.method = Some(Method::List);
            Ok(self)
        }

        fn get(mut self) -> Result<Frontend> { 
            self.url = Some(String::from("/services/haproxy/configuration/frontends"));
            self.method = Some(Method::Get);
            Ok(self)
        }

        fn post(mut self) -> Result<Frontend> {
            let url = String::from("/services/haproxy/configuration/frontends/");
            if let Some(_name) = self.name.clone() {
                self.method = Some(Method::Post);
                self.url = Some(url);
                Ok(self)
            } else {
                Err(String::from("You forgot to define a name"))
            }
        }

        fn send<T: Query + ApiRoot>(self, remote: &T) -> Result<Response> {
            if let Some(method) = self.method {
                let client = Client::new();
                let mut resp;
                match method {
                    Method::List => { 
                        resp = client.get(&remote.build_url(&self.url.unwrap())); 
                    },
                    Method::Post => {
                        let url = &self.url.clone().unwrap();
                        resp = client.post(&remote.build_url(url))
                            .json(&self);
                    },
                    _ => {
                        resp = client.post(&remote.build_url(&self.url.unwrap())); 
                    }
                }
                if let Some((username, password)) = remote.get_credentials() {
                    resp = resp.basic_auth(username, Some(password));
                }
                if remote.has_tracker() {
                    resp = resp.query(&[(remote.get_tracker())]);
                }
                match resp.send() {
                    Ok(response) => Ok(response),
                    Err(e) => Err(e.to_string())
                }
            } else {
                Err(String::from("Method hasn't been set yet"))
            }
        }
    }
}