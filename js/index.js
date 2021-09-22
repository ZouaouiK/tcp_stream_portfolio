var net = require('net');

const solanaWeb3 = require('@solana/web3.js');
const from = solanaWeb3.Keypair.generate();

var client = new net.Socket();
client.connect(7878, '127.0.0.1', function () {
    let pubkey = from.publicKey;
    console.log('client connected');
    console.log("writing", from.publicKey.toBase58())

    // Assets :
    let x = Math.ceil(Math.random() * 10);
    //let x=2;
    console.log('length table x is :', x);
    let tab = [];
    let creatorPortfolio = solanaWeb3.Keypair.generate().publicKey;//32
    let metadataUrl = Buffer.from("portfolio metadataUrl                         ************************", "ascii");
    //console.log("len ", metadataUrl.length);// 70
    let metadataHash = Buffer.from(Array.from(Array(32).keys()));//32  
    console.log("hash ", metadataHash);
    tab.push(creatorPortfolio.toBuffer());
    tab.push(metadataUrl);
    tab.push(metadataHash);
   
    let assets = [];
    assets.push(Buffer.from([x]));

    for (let i = 0; i < x; i++) {
        let amount = 2;//1
        let addressAsset = solanaWeb3.Keypair.generate().publicKey;//32
        let periodAsset = 6;// 1
        let assetToSoldIntrAsset = solanaWeb3.Keypair.generate().publicKey;//32
        assets.push(Buffer.from([amount]));
        assets.push(assetToSoldIntrAsset.toBuffer());
        assets.push(Buffer.from([periodAsset]));
        assets.push(addressAsset.toBuffer());

    }
    const array3 = tab.concat(assets);
  /*   let ftab = Buffer.concat(tab);
   // console.log("tab = ", new Uint8Array(ftab));
    let fassets = Buffer.concat(assets);
    //console.log("assets = ", new Uint8Array(fassets)) */
   let f = Buffer.concat(array3);
    //console.log("Buffer = ", new Uint8Array(f))
    client.write(f);
});

client.on('data', function (data) {
    let b = Buffer.from(data)
    var dd = new Uint8Array(b);
    console.log("response : ", dd);
    let create =b.slice(0,32);
    var view = new Uint8Array(create);
    console.log("view : ",view);
    let metdata_url =b.slice(32,79);
    var metdata_url1 = new Uint8Array(metdata_url);
    console.log(" metdata_url : ", metdata_url1);
    let metdata =b.slice(79,111);
    var metdata_hash = new Uint8Array(metdata);
    console.log(" metdata_url : ", metdata_hash);
    let length_assets=b[112];
    console.log("length_assets ",length_assets);
    let i=0;
  /*   const DEMO_FROM_SECRET_KEY = new Uint8Array([
        37, 21, 197, 185, 105, 201, 212, 148, 164, 108, 251, 159, 174, 252, 43, 246,
        225, 156, 38, 203, 99, 42, 244, 73, 252, 143, 34, 239, 15, 222, 217, 91, 132,
        167, 105, 60, 17, 211, 120, 243, 197, 99, 113, 34, 76, 127, 190, 18, 91, 246,
        121, 93, 189, 55, 165, 129, 196, 104, 25, 157, 209, 168, 165, 149,
    ]);//64
    var from = solanaWeb3.Keypair.fromSecretKey(DEMO_FROM_SECRET_KEY); */
    //let x=view.slice([i..i+32]);
    console.log("create address : ",from.publicKey.toBase58());
    console.log("create : ", create);
    client.destroy(); // kill client after server's response
});
client.on('close', function () {
    console.log('Connection closed');
});

