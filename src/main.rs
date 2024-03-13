use std::{env, io::Write, net::{self, UdpSocket}, path::{Path, PathBuf}};
use std::fs::File;
use std::iter::once;
use dirs_2;

const DEFAULT_PORT : &str = "4123";

const MAX_HEADER_SIZE : usize = 1024;

enum BufferType {
    MESSAGE = 1,
    FILE = 2
}

enum MessageState {
    READY = 3,
}

fn display_help(){
    println!("Simple Udp server/client to transfer data. 
    Usage: sus [-h] [-m <MESSAGE> <ADDRESS> | -f <PATH> <ADDRESS> | -s <PORT>]
    -h                        Displays help message
    -m <MESSAGE> <ADDRESS>    Sends message to address
    -f <PATH> <ADDRESS>       Sends file to address
    -s <PORT>                 Start listener on port. Port 4123 is reserved for sending");
}

fn start_socket(port: &str) -> net::UdpSocket {
    println!("Starting server on PORT {:?}", port);
    let socket = UdpSocket::bind("0.0.0.0:".to_owned()+port).expect("Unable to start server");
    
    socket
}

fn rename_file_on_duplicate(mut path: PathBuf, file_name: &str) -> PathBuf{
    let mut i = 1;
    while Path::new(&path).exists() {
        let iteration = format!(" ({i})");
        let temp = file_name.to_owned().to_string();
        let file_parts = temp.split(".");
        let mut new_name: String = "".to_string();
        let mut added_iteration = false;
        for part in file_parts.into_iter(){
            if !added_iteration{
                new_name = new_name + part;
                new_name = new_name + &iteration[..];
                added_iteration = true;
            }
            else{
                new_name = new_name + "." + part;
            }
        }
        path.set_file_name(new_name);
        i += 1;
    }
    path.to_owned()
}

fn receive(port: &str){
    let socket = start_socket(port);
    let mut header = [0;MAX_HEADER_SIZE];

    loop {
        //Get header
        let (_, src_recv) = socket.recv_from(&mut header).unwrap();

        if header[0] == BufferType::MESSAGE as u8 {
            let ready_message = &[MessageState::READY as u8];
            let sent_size = socket.send_to(ready_message, src_recv).expect("Failed to send message");
            if sent_size > 0{
                    loop{
                        let mut buffer: Vec<u8> = vec![0; header[1] as usize];
                        let (_, src_recv) = socket.recv_from(&mut buffer).unwrap();
                        println!("Received: {:?} from {:?}", String::from_utf8(buffer).unwrap(), src_recv);
                        break;
                    }
            }
        }
        else if header[0] == BufferType::FILE as u8 {
            let file_name: String = String::from_utf8(header[2..MAX_HEADER_SIZE].to_vec()).unwrap_or("sus_file".to_string()).chars().filter(|c| *c != '\0').collect();
            let ready_message = &[MessageState::READY as u8];
            let sent_size = socket.send_to(ready_message, src_recv).expect("Failed to send message");
            if sent_size > 0{
                loop{
                    let mut buffer: Vec<u8> = vec![0;header[1] as usize];
                    socket.recv_from(&mut buffer).unwrap();
                    let mut download_dir = dirs_2::download_dir().unwrap();
                    download_dir.push(file_name.to_owned());

                    let output_dir = rename_file_on_duplicate(download_dir, &file_name);
                    
                    let mut file = File::create(output_dir).expect("Unable to create file");

                    file.write_all(&buffer).expect("Unable to write file");

                    println!("File downloaded: {:?}", &file_name);
                    break;
                }
            }
        }
        else{
            println!("Invalid header received");
            break;
        }
    }
}

fn send_message(socket: &UdpSocket, message: &str, addr: &str){
    let header_buffer = &[BufferType::MESSAGE as u8, message.as_bytes().len() as u8];
    socket.send_to(header_buffer, addr).expect("Failed to send header");

    loop{
        let mut ready_message = [0 ;1];
        let (size_recv, _) = socket.recv_from(&mut ready_message).unwrap();

        if size_recv > 0 {
            if ready_message[0] == MessageState::READY as u8{
                let result = socket.send_to(message.as_bytes(), addr).expect("Failed to send message");
                println!("Sent bytes: {:?}", result);
                break;
            }
        }
    }
}

fn send_file(socket: &UdpSocket, path: &str, addr: &str){
    let file_bytes = std::fs::read(path).expect("Unable to get file");
    let file_name = path.split("\\").last().unwrap();
    let header_buffer_list: Vec<u8> = once(BufferType::FILE as u8)
    .chain(once(file_bytes.len() as u8))
    .chain(file_name.as_bytes().iter().cloned())
    .collect();
    let header_buffer = &header_buffer_list[..];
    socket.send_to(header_buffer, addr).expect("Failed to send header");

    loop{
        let mut ready_message = [0;1];
        let (size_recv, _) = socket.recv_from(&mut ready_message).unwrap();
        if size_recv > 0 {
            if ready_message[0] == MessageState::READY as u8{
                let result = socket.send_to(&file_bytes, addr).expect("Failed to send file");
                println!("Sent file {} bytes {}", file_name, result);
                break;
            }
        }
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        display_help();
        return
    }
    match args[1].as_str() {
        "-h" => {display_help(); return},
        "-m" =>{
            if args.len() < 4 {
                display_help();
                return
            }
            let socket = start_socket(DEFAULT_PORT);
            send_message(&socket, &args[2], &args[3]);
        },
        "-f" =>{
            if args.len() < 4 {
                display_help();
                return
            }
            let socket = start_socket(DEFAULT_PORT);
            send_file(&socket, &args[2], &args[3]);
        },
        "-s" => {
            if args.len() < 3 {
                display_help();
                return
            }
            receive(&args[2]);
        },
        _=> {display_help(); return}
    }
    
    
}
