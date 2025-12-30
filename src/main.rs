use std::fs;
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
    command: String,
    createdat: String,
    dockerid: String,
    image: String,
    labels: String,
    localvolumes: String,
    mounts: String,
    names: String,
    networks: String,
    platform: String,
    ports: String,
    runningfor: String,
    size: String,
    state: String,
    status: String,
}

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

fn main() {
    let ui = AppWindow::new().unwrap();
    ui.on_send_credentials(|username_input: slint::SharedString, password_input: slint::SharedString, ip_addr_input: slint::SharedString| {
        let uiusername = username_input.to_string();
        let uipassword = password_input.to_string();
        let uiipaddress = ip_addr_input.to_string();
        dockercreds(uiusername, uipassword, uiipaddress);
    });
    if Path::new("D:\\VSCode\\hackclub\\dockerui\\dockercred.crd").exists() == true {
        ui.set_credentialcreation(false);
        let dockercred: DockerCred = serde_json::from_str(&fs::read_to_string("dockercred.crd").expect("Unable to read file")).unwrap();
        let dockerip = dockercred.ip_addr.to_string();
        docker_command(dockerip.clone(), r"docker ps -a --format '{{json .}}'".to_string());
    }
    ui.run().unwrap();
}
