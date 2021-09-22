use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use solana_program::pubkey::Pubkey;

fn main() {
       let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    } 
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 32];
    println!(" reading ");
    let n=stream.read(&mut buffer).unwrap();
    println!("n: {}",n);
 
    let mut pubkey=&buffer[0..32];
    println!("pubkey rust : {:?}",pubkey);
   
    let public_key: Pubkey = Pubkey::new(pubkey);
    println!(" {}",public_key); 
   

    println!(" writing ");
    let mut buffer1 = [0; 32];

    buffer1[0..32].clone_from_slice(&pubkey);
   let nn= stream.write(&buffer1[0..32]).unwrap();
  
    stream.flush().unwrap();
    

  
}


/* vec [Hashmap]
use std::collections::HashMap;

struct User {
    reference: String,
    email: String
}

fn main() {

    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("first".to_string(), User { reference: "ref1".to_string(), email: "test@test.com".to_string() });
    users.insert("second".to_string(), User { reference: "ref2".to_string(), email: "test1@test.com".to_string() });
    users.insert("third".to_string(), User { reference: "ref3".to_string(), email: "test3@test.com".to_string() });

    //this is my failed attempt
    let user_refs: Vec<String> = users.iter().map(|(_, user)| &user.reference.clone()).collect();

}
*/
