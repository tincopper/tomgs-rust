use apollo_client::{Client, ClientConfig, IpValue};
use std::time::Duration;
use std::thread;

#[async_std::test]
async fn test_client_listen_0() {
    let config = ClientConfig {
        config_server_url: "http://172.20.183.155:8080",
        app_id: "panshi-agent-dev",
        cluster_name: "default",
        namespace_names: vec!["application"],
        ip: Some(IpValue::HostName),
        ..Default::default()
    };

    let mut client = Client::new(config);
    loop {
        match client.listen_and_request().await {
            Ok(result) => {
                let configuration = result
                    .into_first()?
                    .deserialize_configurations::<serde_json::Value>()?;
                let opt = configuration.get("panshi-agent.admin_host");
                match opt {
                    Some(v) => {
                        dbg!(v);
                    },
                    None => {},
                }
            }
            Err(err) => {
                println!("Listen apollo config change failed: {:?}", err);
                thread::sleep(Duration::from_secs(5));
            }
        };
    }
}