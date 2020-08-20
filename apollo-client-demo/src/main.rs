use apollo_client::{Client, ClientConfig, IpValue};
use serde_json::Value;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use async_std::task;

#[async_std::main]
async fn main() {

    let (tx, rx)= mpsc::channel::<Value>();
    ApolloConfig::new();
    thread::spawn(move || {
        task::block_on(config_apollo(tx));
    });

    loop {
        let recver = rx.recv();
        match recver {
            Ok(v) => println!("recv: {}", v.as_str().unwrap()),
            Err(e) => println!("error: {}", e),
        };
        //let host = rx.recv().unwrap().as_str();
        //let host = apollo_config.get_config("panshi-agent.admin_host");
        //match host {
        //    Some(v) => println!("host: {}", v),
        //    None => println!("none change"),
        //};
        thread::sleep(Duration::from_secs(3));
    }
}

pub struct ApolloConfig {
    configuration: Value,
}

impl ApolloConfig {

    pub fn new() -> ApolloConfig {
        ApolloConfig {
            configuration: Value::default()
        }

    }

    pub fn update_config(&mut self, update_value: &Value) {
        self.configuration.clone_from(update_value);
    }

    pub fn get_config(&self, key: &str) -> Option<&str> {
        match self.configuration.get(key) {
            Some(v) => v.as_str(),
            None => None,
        }
    }
}

async fn config_apollo(sender: Sender<Value>) {
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

                println!("{}", configuration);
                //config_map = configuration;
                //apollo_client.update_config(&configuration);
                sender.send(configuration.clone()).unwrap();
            }
            Err(err) => {
                println!("Listen apollo config change failed: {:?}", err);
                thread::sleep(Duration::from_secs(3));
            }
        };
    }
}