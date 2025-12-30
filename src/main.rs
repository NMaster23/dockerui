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

fn docker_commands(ip_addr_input: String, command: String, count: i32, ui: &AppWindow) {
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
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", channel.exit_status().unwrap());
    let condition = channel.exit_status().unwrap();
    let output = s;
    if condition != 0 {
        println!("Error 0. Unknown Error occurred.");
    }
    let _ = channel.close();
    println!("Intended Disconnection Suceeded");
    if count == 1 {
        ui.set_dockeroutput1(output.into());
    } else if count == 2 {
        ui.set_dockeroutput2(output.into());
    } else if count == 3 {
        ui.set_dockeroutput3(output.into());
    } else if count == 4 {
        ui.set_dockeroutput4(output.into());
    } else if count == 5 {
        ui.set_dockeroutput5(output.into());
    } else if count == 6 {
        ui.set_dockeroutput6(output.into());
    } else if count == 7 {
        ui.set_dockeroutput7(output.into());
    } else if count == 8 {
        ui.set_dockeroutput8(output.into())
    }
}

fn refresh(dockerip: String, ui: &AppWindow) {
    docker_commands(dockerip.clone(), r"docker container list --format 'table {{.ID}}'".to_string(), 1,ui);
    docker_commands(dockerip.clone(), r"docker container list --format 'table {{.Size}}'".to_string(), 2,ui);
    docker_commands(dockerip.clone(), r"docker ps --format 'table {{.Names}}'".to_string(), 3, ui);
    docker_commands(dockerip.clone(), r"docker image list --format 'table {{.ID}}'".to_string(), 4, ui);
    docker_commands(dockerip.clone(), r"docker image list --format 'table {{.Repository}}'".to_string(), 5, ui);
    docker_commands(dockerip.clone(), r"docker image list --format 'table {{.Tag}}'".to_string(), 6, ui);
    docker_commands(dockerip.clone(), r"docker image list --format 'table {{.Size}}'".to_string(), 7, ui);
    docker_commands(dockerip.clone(), r"hostnamectl".to_string(), 8, ui);
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
        refresh(dockerip, &ui);
    }
    ui.run().unwrap();
}
