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

fn docker_command(ip_addr_input: String, command: String, case: i32, ui: &AppWindow) {
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
    if case == 0 {
        let mut dockerinfo = File::create("dockerinfo.inf").unwrap();
        dockerinfo.write_all(output.as_bytes()).unwrap();
    } else if case == 1 {
        ui.set_names(output.to_string().into());
    }

}

fn docker_info(ui: &AppWindow) -> Vec<DockerInfo> {
    let getnewlines_file = fs::read("dockerinfo.inf").unwrap();
    let newlines_amount = getnewlines_file.lines().count();
    let newlines: i32 = newlines_amount as i32;
    println!("{}", newlines_amount);
    ui.set_containeramount(newlines);
    let mut dockerinfo_item: Vec<DockerInfo> = Vec::new();
    let dockerinfo_file = File::open("dockerinfo.inf").unwrap();
    let mut dockerinfo_reader = BufReader::new(dockerinfo_file);
    for _line in 0..newlines {
        let mut dockerinfo_outputline = String::new();
        let _dockerinfo_read = dockerinfo_reader.read_line(&mut dockerinfo_outputline);
        let dockerinfo_output: DockerInfo = serde_json::from_str(&dockerinfo_outputline).unwrap();
        dockerinfo_item.push(dockerinfo_output);
    }
    return dockerinfo_item;
}

fn dockerinfo_toslint(ui: &AppWindow) {
    let dockerinfo = docker_info(ui);
    let commands: String = dockerinfo.iter().map(|info| info.command.clone()).collect();
    ui.set_command(commands.into());
    let created_at: String = dockerinfo.iter().map(|info| info.createdat.clone()).collect();
    ui.set_createdat(created_at.into());
    let docker_id: String = dockerinfo.iter().map(|info| info.dockerid.clone()).collect();
    ui.set_dockerid(docker_id.into());
    let image: String = dockerinfo.iter().map(|info| info.image.clone()).collect();
    ui.set_image(image.into());
    let labels: String = dockerinfo.iter().map(|info| info.labels.clone()).collect();
    ui.set_labels(labels.into());
    let localvolumes: String = dockerinfo.iter().map(|info| info.localvolumes.clone()).collect();
    ui.set_localvolumes(localvolumes.into());
    let mounts: String = dockerinfo.iter().map(|info| info.mounts.clone()).collect();
    ui.set_mounts(mounts.into());
//    let names: String = dockerinfo.iter().map(|info| info.names.clone()).collect();
//    ui.set_names(names.into());
    let networks: String = dockerinfo.iter().map(|info| info.networks.clone()).collect();
    ui.set_networks(networks.into());
    ui.set_platform("Null".into());
    let ports: String = dockerinfo.iter().map(|info| info.ports.clone()).collect();
    ui.set_ports(ports.into());
    let uptime: String = dockerinfo.iter().map(|info| info.runningfor.clone()).collect();
    ui.set_runningfor(uptime.into());
    let state: String = dockerinfo.iter().map(|info| info.size.clone()).collect();
    ui.set_size(state.into());
    let status: String = dockerinfo.iter().map(|info| info.state.clone()).collect();
    ui.set_state(status.into());
    let docker_status: String = dockerinfo.iter().map(|info| info.status.clone()).collect();
    ui.set_status(docker_status.into());
    
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
        docker_command(dockerip.clone(), r"docker ps -a --format '{{json .}}'".to_string(), 0, &ui);
        println!("Executed");
        docker_command(dockerip.clone(), r"docker ps --format '{{.Names}}'".to_string(), 1, &ui);
        dockerinfo_toslint(&ui);
        ui.on_terminal_input(move |terminalinput: slint::SharedString| {
            let terminal = terminalinput.to_string();
//            docker_command(dockerip.clone(), terminal, 1);
        });
    }
    ui.run().unwrap();
}
