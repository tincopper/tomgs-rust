use apollo_client::{Client, ClientConfig, ClientResult, IpValue};

#[async_std::main]
/*async */fn main() -> ClientResult<()> {
    let config = ClientConfig {
        config_server_url: "http://172.20.183.155:8080",
        app_id: "panshi-agent-dev",
        cluster_name: "default",
        namespace_names: vec!["application"],
        ip: Some(IpValue::HostName),
        ..Default::default()
    };

    // Request response once.
    let responses = Client::new(config).request().await?;
    dbg!(&responses);

    let configuration = responses
        .into_first()?
        .deserialize_configurations::<serde_json::Value>()?;
    let opt = configuration.get("panshi-agent.admin_host");
    match opt {
        Some(v) => {
            dbg!(v);
        },
        None => {},
    }
    dbg!(&configuration);

    Ok(())
}

fn get_config(key:&str) -> Option<&str> {
    match configuration.get(key) {
        Some(v) => Some(v.as_str()),
        None => None,
    }
}