use std::fs::File;
use std::path::Path;
use std::io::{ BufRead, BufReader };


// ---- FUNCTIONS ---- //

pub fn read_config(path: &str) {
    let path = Path::new(path);
    let file = File::open(path).expect("Config file not found");
    let content = BufReader::new(&file);

    let mut config = Config::new();
    let mut general_neighbours = Vec::new();
    let mut specific_neighbours = Vec::new();
    let mut rpc_allowed_ips = Vec::new();
    let mut rpc_details = RPCDetails::new();

    for line in content.lines() {
        let line = line.expect("Could not read the line");

        // Remove whitespaces at the beginning and end
        let line = line.trim();
 
        // Ignore comments and empty lines
        if line.starts_with("#") || line.starts_with(";") || line.is_empty() {
            continue;
        }

        let line_split: Vec<&str> = line.split("=").collect();
        let property = line_split[0];
        let value = line_split[1];

        match property {
            "testnet" => { 
                match value {
                    "0" => { config.testnet = false; },
                    _ => { config.testnet = true; }
                }
            },
            "server" => {
                match value {
                    "0" => { config.accept_json_rpc = false; },
                    _ => { config.accept_json_rpc = true; }
                }
            },
            "rpcssl" => {
                match value {
                    "1" => { config.rpc_ssl = true; },
                    _ => { config.rpc_ssl = false; }
                }
            },
            "allowreceivebyip" => {
                match value {
                    "1" => { config.allow_receive_by_ip = true; },
                    _ => { config.allow_receive_by_ip = false; }
                }
            },
            "proxy" => { config.proxy = Some(String::from(value)); },
            "addnode" => { general_neighbours.push(String::from(value)); },
            "connect" => { specific_neighbours.push(String::from(value)); },
            "maxconnections" => { config.max_connections = value.parse::<u8>().unwrap(); },
            "rpcuser" => { config.rpc_user = Some(String::from(value)); },
            "rpcpassword" => { config.rpc_password = Some(String::from(value)); },
            "rpctimeout" => { config.rpc_timeout = value.parse::<u8>().unwrap(); },
            "rpcallowip" => { rpc_allowed_ips.push(String::from(value)); },
            "rpcport" => { config.rpc_port = String::from(value); },
            "rpcsslciphers" => { rpc_details.ciphers = String::from(value); },
            "rpcsslcertificatechainfile" => { rpc_details.cert_file = String::from(value); },
            "rpcsslprivatekeyfile" => { rpc_details.key_file = String::from(value); },
            "keypool" => { config.keypool = value.parse::<u8>().unwrap(); },
            "paytxfee" => { config.pay_tx_fee = value.parse::<f32>().unwrap(); },
            _ => println!("Property not catered to")
        }

    }

    config.general_neighbours = general_neighbours;
    config.specific_neighbours = specific_neighbours;
    
    if config.rpc_ssl {
        config.rpc_details = Some(rpc_details);
    }
    
    println!("{:?}", config);
}


// ---- STRUCTS ---- //

#[derive(Clone, Debug)]
pub struct RPCDetails {
    ciphers: String,
    cert_file: String,
    key_file: String
}

impl RPCDetails {
    fn new() -> RPCDetails {
        RPCDetails {
            ciphers: String::from(""),
            cert_file: String::from(""),
            key_file: String::from("")
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    testnet: bool,
    proxy: Option<String>,
    general_neighbours: Vec<String>,
    specific_neighbours: Vec<String>,
    max_connections: u8,
    accept_json_rpc: bool,
    rpc_user: Option<String>,
    rpc_password: Option<String>,
    rpc_timeout: u8,
    rpc_allowed_ips: Option<Vec<String>>,
    rpc_port: String,
    rpc_ssl: bool,
    rpc_details: Option<RPCDetails>,
    keypool: u8,
    pay_tx_fee: f32,
    allow_receive_by_ip: bool
}

impl Config {
    fn new() -> Config {
        Config {
            testnet: false,
            proxy: None,
            general_neighbours: Vec::new(),
            specific_neighbours: Vec::new(),
            max_connections: 0,
            accept_json_rpc: true,
            rpc_user: None,
            rpc_password: None,
            rpc_timeout: 0,
            rpc_allowed_ips: None,
            rpc_port: String::from(""),
            rpc_ssl: false,
            rpc_details: None,
            keypool: 0,
            pay_tx_fee: 0.00,
            allow_receive_by_ip: false
        }
    }
}