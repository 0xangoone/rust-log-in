use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::fs;
use std::thread;
use mongodb::{bson::{doc, Document}, options::ClientOptions, Client};
use tokio::runtime::Runtime;

fn handle(mut stream: TcpStream){
    let mut buff = [0u8; 50];
    let data = stream.read(&mut buff).unwrap();
    let str_data = String::from_utf8_lossy(&buff[0..data]);
    println!("{:?}", str_data);
    let path = str_data.split(" ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[0];
    println!("{path}");
    if path == "/"{
        let code:String = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",code.len(),code);
        println!("{}",response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    else if path.starts_with("/in="){
        let splited = path.split("=").collect::<Vec<&str>>();
        println!("{:?}",splited);
        let email = splited[1].split("&").collect::<Vec<&str>>()[0];
        let password = path.split("&").collect::<Vec<&str>>()[1];
        println!{
            "password: {password}\n
             \remail: {email}"
        };
        let code = "{\"response\":\"succed\"}".to_string();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",code.len(),code);
        let mut rt = Runtime::new().unwrap();
        rt.block_on(save_user_data(email, password));
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
async fn save_user_data(email: &str, password: &str){
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("users");
    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("{}", collection_name);
    }
    let connection = db.collection::<Document>("accounts");
    connection.insert_one(doc! {"email":email,"password":password}, None).await.unwrap();
}
fn main() {
    let tl = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in tl.incoming(){
        thread::spawn(move||
            if let Ok(stream) = stream {
                handle(stream);
            } else {
                println!("Failed");
            }
        );
    }
}
