use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use solana_program::pubkey::Pubkey;
use byteorder::{ByteOrder, LittleEndian};

use rand::Rng;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

 for stream in listener.incoming() {
     let stream = stream.unwrap();

    // unpack(stream);
     pack(stream);
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
fn pack(mut stream: TcpStream) {
    let mut rng = rand::thread_rng();
    let nb_asset: u8 = rng.gen::<u8>() % 10 + 1;
    let asset_length = 66u8; 
    let len_data=nb_asset as usize * asset_length as usize;
  //  let mut buffer:Vec<u8> = Vec::with_capacity(147+(len_data as usize));
  let mut buffer = [0; 8192];
    //portfolioAddress
    let  buffer_portfolio = [0; 32];
    let  portfolio_address_key: Pubkey = Pubkey::new(&buffer_portfolio);//pubkey
    println!("portfolio_address {}", portfolio_address_key); 

    //creatorPortfolio
    let  buffer_creator = [1; 32];
    let  creator_address_key: Pubkey = Pubkey::new(&buffer_creator);//pubkey
    println!(" creator_address {}", creator_address_key); 

    //owner
    let  buffer_owner = [2; 32];
    let  owner_address_key: Pubkey = Pubkey::new(& buffer_owner);//pubkey
    println!(" creator_address {}", owner_address_key); 

    //metadataUrl
    let  metadata_url=b"karima zouaoui and jawaher korbosli best friend";//47

    //metdataHash

    let  metadata_hash=999u16;
    let x=metadata_hash.to_le_bytes();
    let initialize=1u8;
    let asset_len=2u8;
    println!(" x {:?}", x); 
   // println!("buffer before clone   {:?}",&buffer[0..]);
    buffer[0..32].clone_from_slice(&buffer_portfolio);
    buffer[32..64].clone_from_slice(&buffer_creator);
    buffer[64..96].clone_from_slice(&buffer_owner);
    buffer[96..143].clone_from_slice(metadata_url);
    buffer[143..145].clone_from_slice(&x);
    buffer[145]=initialize;
    buffer[146]=nb_asset;
    println!("buffer write  {:?}",&buffer[0..147]);
   
    //let mut asset_data:Vec<u8> = Vec::with_capacity(len_data as usize);
   
    let mut i = 0 as usize;
    println!("i before while ={}",i); 
    let offset = 0usize;
   const asset_length2:usize=66;
    const nb_asset2:usize=5;
    const kk:usize=(nb_asset2* asset_length2) as usize;
    println!("kkk {}",kk);
    let mut asset_data=[0;660];
    while i < nb_asset as usize  {
        println!("i after while ={}",i); 
        let start = offset + i * (asset_length as usize);
        println!("i ={}",i); 
        let amount=5u8;
        let period=9u8;
        let  address_asset =[3; 32];
        let  asset_to_sold_into_asset = [4; 32];
        asset_data[start..start+32].clone_from_slice(&address_asset);
        println!("amount {}",amount);
        asset_data[start+32]=amount;
        asset_data[start+33..start+65].clone_from_slice(&asset_to_sold_into_asset);
       // println!("start + 66 ={}",start + 66 );
        asset_data[start+65]=period;
         i = i+1;

        
    }

   
   buffer[147..807].clone_from_slice(&asset_data);
    println!("buffer write  {:?}",  &buffer[0..807]);
   // println!("asset data write  {:?}",&asset_data[0..kk]);
   let mut buffer1:Vec<u8> = Vec::with_capacity(147+len_data);
   buffer1.resize(147+len_data, 0);
   buffer1.copy_from_slice(&buffer[0..147+len_data]);
   println!("buffer1  {:?}",  &buffer1[0..]);
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


