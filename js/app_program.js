var net = require('net');

const solanaWeb3 = require('@solana/web3.js');
const from = solanaWeb3.Keypair.generate();

var client = new net.Socket();
client.connect(7878, '127.0.0.1', function() {
let pubkey=from.publicKey;
    console.log('client connected');
    console.log( "writing", from.publicKey.toBase58())
	client.write(pubkey.toBuffer());
    });
 
client.on('data', function(data){
    let b = Buffer.from(data)
    var view = new Uint8Array(b);
    console.log("reading : ",view);
    client.destroy(); // kill client after server's response
});
client.on('close', function() {
	console.log('Connection closed');
});

