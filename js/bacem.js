var net = require('net');

const solanaWeb3 = require('@solana/web3.js');
const from = solanaWeb3.Keypair.generate();

console.log(from);