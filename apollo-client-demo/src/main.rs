mod apollo_client;

use crate::apollo_client::ApolloClient;
use std::time::Duration;
use std::thread;
use std::sync::Mutex;

#[async_std::main]
async fn main() {

  let lock = Mutex::new(ApolloClient::new());
  let mut client = lock.lock().unwrap();
  let host = client.get_config("panshi-agent.admin_host");
  println!("host: {}", host.unwrap());

  let mut client = ApolloClient::new();

  loop {
     let host = client.get_config("panshi-agent.admin_host");
     match host {
         Some(v) => println!("host: {}", v),
         None => println!("none change"),
     };

     let listen = client.get_config("panshi-agent.listen");
     println!("listen: {}", listen.unwrap());
     thread::sleep(Duration::from_secs(3));
  }

}