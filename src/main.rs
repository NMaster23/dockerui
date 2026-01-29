use std::thread::sleep;
use std::time;
use std::{fs, io::BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use serde::{Serialize, Deserialize};
use slint::ComponentHandle;
use std::net::TcpStream;
use ssh2::Session;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

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
    let encryption = new_magic_crypt!("magickey", 256);
    let auth = serde_json::to_string(&dockercred).unwrap();
    let mut authentication = File::create("dockercred.crd").unwrap();
    let encrypted = encryption.encrypt_str_to_base64(&auth);
    authentication.write_all(encrypted.as_bytes()).unwrap();
}

fn docker_command(ip_addr_input: String, command: String, case: i32, ui: &AppWindow) {
    let encryption = new_magic_crypt!("magickey", 256);
    let connectionaddress = format!("{}:22", ip_addr_input);
    let tcp = TcpStream::connect(connectionaddress).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let auth_file = fs::read_to_string("dockercred.crd").expect("Unable to read file");
    let decrypted: String = encryption.decrypt_base64_to_string(&auth_file).unwrap();
    let dockerauth: DockerCred = serde_json::from_str(&decrypted).unwrap();
    sess.userauth_password(&dockerauth.username, &dockerauth.password).unwrap();
    assert!(sess.authenticated());
    let mut channel = sess.channel_session().unwrap();
    channel.exec(&command).unwrap();
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    let condition = channel.exit_status().unwrap();
    if condition != 0 {
        println!("Error 0. Unknown Error occurred.");
    }
    let _ = channel.close();
    if case == 0 {
        let mut dockerinfo = File::create("dockerinfo.inf").unwrap();
        dockerinfo.write_all(output.as_bytes()).unwrap();
    } else if case == 1 {
        if output.contains("exists") {
            ui.set_file_exist(true);
        } else {
            ui.set_file_exist(false);
        }
    } else if case == 2 {
        println!("Docker UI directory created.");
    } else if case == 3 {
        println!("New container directory created. Please edit the docker.yaml file.");
        println!("Docker YAML file created. Please proceed to deploy the container.");
    }
}

fn docker_info(ui: &AppWindow) -> Vec<DockerInfo> {
    let getnewlines_file = fs::read("dockerinfo.inf").unwrap();
    let newlines_amount = getnewlines_file.lines().count();
    let newlines: i32 = newlines_amount as i32;
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
    let commands = dockerinfo_item.iter().map(|i| i.command.clone()).collect::<Vec<_>>().join("\n");
    let createdat = dockerinfo_item.iter().map(|i| i.createdat.clone()).collect::<Vec<_>>().join("\n");
    let dockerid = dockerinfo_item.iter().map(|i| i.dockerid.clone()).collect::<Vec<_>>().join("\n");
    let image = dockerinfo_item.iter().map(|i| i.image.clone()).collect::<Vec<_>>().join("\n");
    let labels = dockerinfo_item.iter().map(|i| i.labels.clone()).collect::<Vec<_>>().join("\n");
    let localvolumes = dockerinfo_item.iter().map(|i| i.localvolumes.clone()).collect::<Vec<_>>().join("\n");
    let mounts = dockerinfo_item.iter().map(|i| i.mounts.clone()).collect::<Vec<_>>().join("\n");
    let names = dockerinfo_item.iter().map(|i| i.names.clone()).collect::<Vec<_>>().join("\n");
    let networks = dockerinfo_item.iter().map(|i| i.networks.clone()).collect::<Vec<_>>().join("\n");
    let platform = dockerinfo_item.iter().map(|i| i.platform.clone().unwrap_or("None".into())).collect::<Vec<_>>().join("\n");
    let ports = dockerinfo_item.iter().map(|i| i.ports.clone()).collect::<Vec<_>>().join("\n");
    let runningfor = dockerinfo_item.iter().map(|i| i.runningfor.clone()).collect::<Vec<_>>().join("\n");
    let size = dockerinfo_item.iter().map(|i| i.size.clone()).collect::<Vec<_>>().join("\n");
    let state = dockerinfo_item.iter().map(|i| i.state.clone()).collect::<Vec<_>>().join("\n");
    let status = dockerinfo_item.iter().map(|i| i.status.clone()).collect::<Vec<_>>().join("\n");

    ui.set_command(commands.into());
    ui.set_createdat(createdat.into());
    ui.set_dockerid(dockerid.into());
    ui.set_image(image.into());
    ui.set_labels(labels.into());
    ui.set_localvolumes(localvolumes.into());
    ui.set_mounts(mounts.into());
    ui.set_names(names.into());
    ui.set_networks(networks.into());
    ui.set_platform(platform.into());
    ui.set_ports(ports.into());
    ui.set_runningfor(runningfor.into());
    ui.set_size(size.into());
    ui.set_state(state.into());
    ui.set_status(status.into());

    return dockerinfo_item;
}

fn refresh(ui: &AppWindow) {
    if Path::new("dockercred.crd").exists() == true {
        ui.set_credentialcreation(false);
        let encryption = new_magic_crypt!("magickey", 256);
        let auth_file = fs::read_to_string("dockercred.crd").expect("Unable to read file");
        let decrypted: String = encryption.decrypt_base64_to_string(&auth_file).unwrap();
        let dockercred: DockerCred = serde_json::from_str(&decrypted).unwrap();
        let dockerip = dockercred.ip_addr.to_string();
        docker_command(dockerip.clone(), r"docker ps -a --format '{{json .}}'".to_string(), 0, &ui);
        docker_info(&ui);
    }
}

fn main() {
    let ui = AppWindow::new().unwrap();
    refresh(&ui);
    ui.on_send_credentials(|username_input: slint::SharedString, password_input: slint::SharedString, ip_addr_input: slint::SharedString| {
        let uiusername = username_input.to_string();
        let uipassword = password_input.to_string();
        let uiipaddress = ip_addr_input.to_string();
        dockercreds(uiusername, uipassword, uiipaddress);
    });
    if Path::new("dockercred.crd").exists() == true {
        let encryption = new_magic_crypt!("magickey", 256);
        let auth_file = fs::read_to_string("dockercred.crd").expect("Unable to read file");
        let decrypted: String = encryption.decrypt_base64_to_string(&auth_file).unwrap();
        let dockercred: DockerCred = serde_json::from_str(&decrypted).unwrap();
        let dockerip = dockercred.ip_addr.to_string();
        let ui_refresh = ui.clone_strong();
        let dockerinfo_text = docker_info(&ui);
        let button_textpre: Vec<slint::SharedString> = dockerinfo_text.iter().map(|d| slint::SharedString::from(d.names.clone())).collect();
        let button_texts = slint::ModelRc::new(slint::VecModel::from(button_textpre));
        ui.set_button_texts(button_texts);
        docker_command(dockerip.clone(), r"[ -f dockerui ] && echo 'exists' || echo 'missing'".to_string(), 1, &ui);
        if ui.get_file_exist() == false {
            docker_command(dockerip.clone(), "mkdir dockerui".to_string(), 2, &ui);
        }
        ui.on_send_refresh_state(move |refreshstate: bool| {
            if refreshstate == true {
                refresh(&ui_refresh);
            }
        });
        let ui_container = ui.clone_strong();
        ui.on_docker_newcontainer_info(move |containername: slint::SharedString, dockeryaml: slint::SharedString| {
            let uicontainername = containername.to_string().to_lowercase();
            let uidockeryaml = dockeryaml.to_string();
            let new_container = format!("cd dockerui && mkdir {} && cd {} && cat > compose.yaml <<< \"{}\" && docker compose up -d", uicontainername, uicontainername, uidockeryaml);
            docker_command(dockerip.clone(), new_container, 3, &ui_container);
            refresh(&ui_container);
        });
    }
    ui.run().unwrap();
}
