use apollo_client::{Client, ClientConfig, IpValue};
use serde_json::Value;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use async_std::task;
use std::collections::HashMap;

//#[async_std::main]
/*async */fn main() {

    //let (tx, rx)= mpsc::channel::<Value>();

    let mut apollo = ApolloConfig::new();

    loop {
        let host = apollo.get_config("panshi-agent.admin_host");
        match host {
            Some(v) => println!("host: {}", v),
            None => println!("none change"),
        };

        let listen = apollo.get_config("panshi-agent.listen");
        println!("listen: {}", listen.unwrap());
        thread::sleep(Duration::from_secs(20));
    }
}

pub struct ApolloConfig {
    rx: Receiver<Value>,
    config: HashMap<String, String>,
}

impl ApolloConfig {
    pub fn new() -> ApolloConfig {
        let (tx_tmp, rx_tmp) = mpsc::channel::<Value>();

        thread::spawn(move || {
            task::block_on(config_apollo(tx_tmp));
        });

        let mut config_map: HashMap<String, String> = HashMap::new();
        let recv = rx_tmp.recv();

        match recv {
            Ok(mut v) => {
                let config = v.as_object_mut().unwrap();
                for (key, value) in config {
                    config_map.insert(key.to_string(), value.to_string());
                }
            }
            Err(e) => println!("receive error: {}", e),
        }

        ApolloConfig {
            rx: rx_tmp,
            config: config_map,
        }
    }

    pub fn get_config(&mut self, key: &str) -> Option<&str> {
        let recv = self.rx.try_recv();
        match recv {
            Ok(v) => {
                let config = v.as_object().unwrap();
                for (key, value) in config {
                    self.config.insert(key.to_string(), value.to_string());
                }
            }
            Err(_e) => {}
        }

        // Option<&String> ==> Option<&str>
        self.config.get(key).map(String::as_str)
    }
}

pub async fn config_apollo(sender: Sender<Value>) {
    let apollo_config = ClientConfig {
        config_server_url: "http://172.20.183.155:8080",
        app_id: "panshi-agent-dev",
        cluster_name: "default",
        namespace_names: vec!["application"],
        ip: Some(IpValue::HostName),
        ..Default::default()
    };

    let mut client = Client::new(apollo_config);
    loop {
        match client.listen_and_request().await {
            Ok(result) => {
                let configuration = result
                    .into_first().unwrap()
                    .deserialize_configurations::<serde_json::Value>().unwrap();

                sender.send(configuration.clone()).unwrap();
                println!("config update success, config: {}.", configuration)
            }
            Err(err) => {
                println!("Listen apollo config change failed: {:?}", err);
                thread::sleep(Duration::from_secs(3));
            }
        };
    }
}
