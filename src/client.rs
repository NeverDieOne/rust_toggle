use crate::config::ToggleConfig;


pub fn _create_client(config: &ToggleConfig) -> reqwest::blocking::Client {
    let mut client_builder = reqwest::blocking::Client::builder();

    if config.proxy != "" {
        let proxy = reqwest::Proxy::http(&config.proxy).expect("Can not create proxy");
        client_builder = client_builder.proxy(proxy);
    }

    client_builder
        .build()
        .expect("Can not create client")
}