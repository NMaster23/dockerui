use std::{fs, io::BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use ssh2::Session;

slint::include_modules!();

#[derive(Serialize, Deserialize)]
struct DockerCred {
    username: String,
    password: String,
    ip_addr: String,
}

#[derive(Serialize, Deserialize)]
struct DockerInfo {
    #[serde(rename = "Command")]
    command: String,
    #[serde(rename = "CreatedAt")]
    createdat: String,
    #[serde(rename = "ID")]
    dockerid: String,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "Labels")]
    labels: String,
    #[serde(rename = "LocalVolumes")]
    localvolumes: String,
    #[serde(rename = "Mounts")]
    mounts: String,
    #[serde(rename = "Names")]
    names: String,
    #[serde(rename = "Networks")]
    networks: String,
    #[serde(rename = "Platform")]
    platform: Option<String>,
    #[serde(rename = "Ports")]
    ports: String,
    #[serde(rename = "RunningFor")]
    runningfor: String,
    #[serde(rename = "Size")]
    size: String,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Status")]
    status: String,
}
/*
enum DockerInfo {
    command(String),
    createdat(String),
    dockerid(String),
    image(String),
    labels(String),
    localvolumes(String),
    mounts(String),
    names(String),
    networks(String),
    platform(String),
    ports(String),
    runningfor(String),
    size(String),
    state(String),
    status(String),
}*/

fn dockercreds(username_input: String, password_input: String, ip_addr_input: String) {
    let dockercred = DockerCred {
        username: username_input,
        password: password_input,
        ip_addr: ip_addr_input,
    };
    let auth = serde_json::to_string(&dockercred).unwrap();
    let mut authentication = File::create("dockercred.crd").unwrap();
    authentication.write_all(auth.as_bytes()).unwrap();
}

fn docker_command(ip_addr_input: String, command: String) {
    let connectionaddress = format!("{}:22", ip_addr_input);
    let tcp = TcpStream::connect(connectionaddress).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let dockerauth: DockerCred = serde_json::from_str(&fs::read_to_string("dockercred.crd").expect("Unable to read file")).unwrap();
    sess.userauth_password(&dockerauth.username, &dockerauth.password).unwrap();
    assert!(sess.authenticated());
    let mut channel = sess.channel_session().unwrap();
    channel.exec(&command).unwrap();
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    println!("{}", channel.exit_status().unwrap());
    let condition = channel.exit_status().unwrap();
    if condition != 0 {
        println!("Error 0. Unknown Error occurred.");
    }
    let _ = channel.close();
    let mut dockerinfo = File::create("dockerinfo.inf").unwrap();
    dockerinfo.write_all(output.as_bytes()).unwrap();    

}

fn docker_info() -> Vec<DockerInfo> {
    let getnewlines_file = fs::read("dockerinfo.inf").unwrap();
    let newlines_amount = getnewlines_file.lines().count();
    let newlines: i32 = newlines_amount as i32;
    println!("{}", newlines_amount);
    let mut dockerinfo_item: Vec<DockerInfo> = Vec::new();
    let dockerinfo_file = File::open("dockerinfo.inf").unwrap();
    let mut dockerinfo_reader = BufReader::new(dockerinfo_file);
    for line in 0..newlines {
        let mut dockerinfo_outputline = String::new();
        let _dockerinfo_read = dockerinfo_reader.read_line(&mut dockerinfo_outputline);
        let dockerinfo_output: DockerInfo = serde_json::from_str(&dockerinfo_outputline).unwrap();
        dockerinfo_item.push(dockerinfo_output);
    }
    return dockerinfo_item;
}


fn main() {
    let ui = AppWindow::new().unwrap();
    ui.on_send_credentials(|username_input: slint::SharedString, password_input: slint::SharedString, ip_addr_input: slint::SharedString| {
        let uiusername = username_input.to_string();
        let uipassword = password_input.to_string();
        let uiipaddress = ip_addr_input.to_string();
        dockercreds(uiusername, uipassword, uiipaddress);
    });
    if Path::new("dockercred.crd").exists() == true {
        ui.set_credentialcreation(false);
        let dockercred: DockerCred = serde_json::from_str(&fs::read_to_string("dockercred.crd").expect("Unable to read file")).unwrap();
        let dockerip = dockercred.ip_addr.to_string();
        docker_command(dockerip.clone(), r"docker ps -a --format '{{json .}}'".to_string());
        println!("Executed");
        let dockerinfo = docker_info();
        
        
    }
    ui.run().unwrap();
}
