use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use solana_program::pubkey::Pubkey;
use rand::Rng;

fn main() {
       let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    } 
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 8192];
    println!(" reading ");
    let nn =stream.read(&mut buffer).unwrap();
   // println!(" buffer read {:?}" , buffer);
   // println!("Reading length stream  {} numbers" , nn);
  

    let mut s1:[u8;32] = [0; 32];
    let  s:[u8;8] = [0; 8];
    let mut s2:[u8;70] = [0; 70];
    let mut i = 0 as usize;

    s1.copy_from_slice(&buffer[0..32]); //creatorPortfolio
    let pubkey_c=&buffer[0..32];
    let public_c_key: Pubkey = Pubkey::new(pubkey_c);
    println!(" pubkey creatorPortfolio {}",public_c_key); 

    s2.copy_from_slice(&buffer[32..102]); //metadataUrl
    let  metadata_url=&buffer[32..102];
    let x=str::from_utf8(metadata_url);
    println!("metadata_url  : {:?}",metadata_url);
    println!(" x  : {}", x);

    s1.copy_from_slice(&buffer[102..134]); //metadataHash
    let metadata_hash=&buffer[102..134];
    println!(" metadata_hash  : {:?}", metadata_hash);
   // println!(" s1 : {:?}, s2 : {:?}",&s1,&s2);

 //println!("nnnn {}" , nnnn);
    let length = buffer[134] as usize;
    let offset = 135usize;

    let asset_length = 66usize; 
    println!("Length Table {}" , length);
    while i < length {
        let start = offset + i * asset_length;

        let amount =buffer[start];

        s1.copy_from_slice(&buffer[start..(start+32)]);// addressAsset
        let  address_asset=&buffer[start..(start+32)];
        let address_asset_key: Pubkey = Pubkey::new(address_asset);
       // println!("address_asset {}",address_asset_key); 

        let period = buffer[(start+33)];//pariod assets

        s1.copy_from_slice(&buffer[(start+33)..(start+65)]);//assetToSoldIntrAsset
        let  asset_to_sold_into_asset=&buffer[(start+33)..(start+65)];
        let asset_to_sold_into_asset_key: Pubkey = Pubkey::new(asset_to_sold_into_asset);
       // println!("asset_to_sold_into_asset {}",asset_to_sold_into_asset_key); 
      
        println!(" amount : {} ,period : {} , address_asset: {}, asset_to_sold_into_asset_key :{} ",    
               amount,
               period,
               address_asset_key,
               asset_to_sold_into_asset_key
            );
            i = i+1;
        }

     
    println!(" writing ");
    let mut buffer = [0; 8192];
    let  buffer_creator = [4; 32];
    let  creator_address_key: Pubkey = Pubkey::new(&buffer_creator);//pubkey
    println!(" creator_address {}", creator_address_key); 

    let  metdata_url=b"karima zouaoui and jawaher korbosli best friend";//47
    println!(" metdata_url {:?}",metdata_url );
    println!(" metdata_url.len() {}",metdata_url.len() );

    let  metadata_hash=[1;32];
    println!(" metadata_hash {:?}",metadata_hash ); //32
   
   /*  let asset_to_sold_into_asset_key: Pubkey = Pubkey::new(&buffer_asset_to_sold);//pubkey
    println!("asset_to_sold_into_asset {}",asset_to_sold_into_asset_key);  */
   /*  let x=asset_to_sold_into_asset_key.to_bytes();
    println!("x {:?}",x);  */
   
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen::<u8>() % 10 + 1;

 //let number: u8 = rng.gen::<u8>() % 10 + 1;
   

    let asset_length = 66usize; 
   
    println!("writing {}",number); 
   // buffer[111].clone_from_slice(2);
   buffer[0..32].clone_from_slice(&buffer_creator);
   buffer[32..79].clone_from_slice(metdata_url);
   buffer[79..111].clone_from_slice(&metadata_hash);
   
    let mut i = 0usize;
    buffer[111]=number;
    let offset = 112usize;
    while i < number as usize  {
        let start = offset + i * asset_length;
        println!("start ={}",start);
        let amount=3u8;
        let period=9u8;
        let  address_asset =[5; 32];
        let  asset_to_sold_into_asset = [7; 32];
        buffer[start..(start+32)].clone_from_slice(&address_asset);
        buffer[start+32]=amount;
        buffer[(start+33)..(start+65)].clone_from_slice(&asset_to_sold_into_asset);
        println!("start + 66 ={}",start + 66 );
        buffer[start+65]=period;
        
      
         
         i = i+1;

        
    }
   
    

    let nn=  stream.write(&buffer[0..((number as usize) *asset_length+offset)]).unwrap();


  
    println!("buffer write  {:?}",&buffer[0..((number as usize) *asset_length+offset)]);

   /*  for (i,j) in (&buffer[..245]).iter().enumerate() {
        print!("[{}:{}] " , i , j);
    }
 */
    println!("nn {}",nn);
    /* let response = "🍎 Narjassi \r\n\r\n";
   
    stream.write(response.as_bytes()).unwrap(); */
    stream.flush().unwrap();
  
}
