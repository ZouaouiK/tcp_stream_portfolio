use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use solana_program::pubkey::Pubkey;
use byteorder::{ByteOrder, LittleEndian};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

 for stream in listener.incoming() {
     let stream = stream.unwrap();

     unpack(stream);
 } 
 
}


fn unpack(mut stream: TcpStream) {
    let mut buffer = [0;302];
    let mut buffer_metadata_url:[u8;70] = [0; 70];
    println!(" reading ");
    let n=stream.read(&mut buffer).unwrap();
    println!("n : {}",n);
    println!("buffer : {:?}",buffer);
    let portfolio=&buffer[0..32];
    let portfolio_key: Pubkey = Pubkey::new(portfolio);
    println!("portfolio_key rust : {:?}",portfolio_key);
    let creator=&buffer[32..64];
    let creator_key: Pubkey = Pubkey::new(creator);
    println!("creator_key rust : {:?}",creator_key);
    let owner=&buffer[64..96];
    let owner_key: Pubkey = Pubkey::new(owner);
    println!("owner_key rust : {:?}",owner_key);
   buffer_metadata_url.copy_from_slice(&buffer[96..166]); //metadataUrl
   //println!("buffer_metadata_url : {:?}",buffer_metadata_url);
    let x=std::str::from_utf8(&buffer_metadata_url);
    println!("metadata_url  : {}", x.unwrap());
    let metadata_hash=&buffer[166..168];
    let num =LittleEndian::read_u16(metadata_hash);
    println!("metadata_hash : {}",num);
    let is_initializ=buffer[168]==1;
    println!("is_initializ : {}",is_initializ);
    let nb_asset=buffer[169] as usize;
    println!("nb_asset : {}",nb_asset);
    let offset = 170usize;
   let asset_data=&buffer[170..302];
    let asset_length = 66usize; 
    let mut i = 0 as usize;
    while i < nb_asset{
        let start = offset + i * asset_length;
        println!("start {}",start);
        let amount=buffer[start];
        println!("amount_asset{} :{}",i,amount);
        let  address_asset=&buffer[start+1..(start+33)];
        let address_asset_key: Pubkey = Pubkey::new(address_asset);
        println!("address_asset{} is {}",i,address_asset_key);
        let period = buffer[(start+33)];//pariod assets
        println!("period_asset{} :{}",i,period);
        let  asset_to_sold_into_asset=&buffer[(start+34)..(start+66)];
        let asset_to_sold_into_asset_key: Pubkey = Pubkey::new(asset_to_sold_into_asset);
        println!("asset_to_sold_into_asset{} is {}",i,asset_to_sold_into_asset_key);
        i = i+1;
        

    }
    let p=Portfolio {
        portfolio_account: portfolio_key,
        creator_portfolio: creator_key,
        metadata_url: buffer_metadata_url.to_vec(),
        metadata_hash: num,
      is_initialize: buffer[168],
      asset_data_len: buffer[169],
      asset_data: asset_data.to_vec(),

    };

    println!("p {:?}",p);
   /* 
    let mut pubkey=&buffer[0..32];
    println!("pubkey rust : {:?}",pubkey);
   
    let public_key: Pubkey = Pubkey::new(pubkey);
    println!(" public key rust {}",public_key);  */

    let response = "🍎 Narjassi \r\n\r\n";
   
    stream.write(response.as_bytes()).unwrap(); 
    stream.flush().unwrap();

}



#[derive(Clone, /* Copy,*/ Debug, Default, PartialEq)]
pub struct Portfolio {
    /// The account's creator
    pub portfolio_account: Pubkey,
    /// The owner of this account.
    pub creator_portfolio: Pubkey,
    /// The data of portfolio.
    pub metadata_url: Vec<u8>,
    /// the hash of data
    pub metadata_hash: u16,
    /// is initialize
    pub is_initialize: u8,
    /// asset's size
    pub asset_data_len: u8,
    ///  Asset's information
    pub asset_data: Vec<u8>,
   


}


