var net = require('net');
const solanaWeb3 = require('@solana/web3.js');
const portfolioAccount = solanaWeb3.Keypair.generate();
const creatorPortfolioAccount = solanaWeb3.Keypair.generate();
const ownerAccount = solanaWeb3.Keypair.generate();
const addressAsset = solanaWeb3.Keypair.generate().publicKey;
const assetToSoldIntoAsset = solanaWeb3.Keypair.generate().publicKey;

const Asset1 = {
    /*
        * The amount of  asset
    */
    amountAsset: 2,
    /*
        * The address of first asset
    */
    addressAsset: addressAsset,
    /*
        * The period of first asset
    */
    periodAsset: 3,
    /*
        * The asset solde of first asset
    */
    assetToSoldIntoAsset: assetToSoldIntoAsset,

}
const Asset2 = {
    /*
        * The amount of  asset
    */
    amountAsset: 4,
    /*
        * The address of first asset
    */
    addressAsset: addressAsset,
    /*
        * The period of first asset
    */
    periodAsset: 1,
    /*
        * The asset solde of first asset
    */
    assetToSoldIntoAsset: assetToSoldIntoAsset,

}
const portfolio = {
    /* The address of this account*/
    portfolioAddress: portfolioAccount.publicKey,
    /* the address of the creator*/
    creatorPortfolio: creatorPortfolioAccount.publicKey,
    /* Owner of this account */
    owner: ownerAccount.publicKey,

    /* metadata url*/
    metadataUrl: "portfolio metadataUrl                         ************************",
    // metadataUrl: BufferLayout.blob,

    /* metadata hash*/
     metaDataHash : new Uint16Array([999]),

    /* initialized account*/
    is_initialize: 0,
    /* Length of assets by portfolio*/
    asset_data_len: 2,
    /* assets informations*/
    asset_data: [Asset1, Asset2],
}




var client = new net.Socket();
client.connect(7878, '127.0.0.1', function () {
    console.log('client connected');
    let tab = [];

    console.log('portfolio.portfolioAccount js ::', portfolio.portfolioAddress.toBase58());
    tab.push(portfolio.portfolioAddress.toBuffer());//32

    console.log('portfolio.creatorPortfolio js ::', portfolio.creatorPortfolio.toBase58());
    tab.push(portfolio.creatorPortfolio.toBuffer());//32

    console.log('portfolio.owner js ::', portfolio.owner.toBase58());
    tab.push(portfolio.owner.toBuffer());//32

    let metadataUrl = Buffer.from(portfolio.metadataUrl, "ascii");
    tab.push(metadataUrl);//70
    console.log("kkkk ",portfolio.metaDataHash);
    let x=Buffer.alloc(2,portfolio.metaDataHash)
    console.log("jjjjj",x);
    tab.push(x);//2
    
    tab.push(Buffer.from([portfolio.is_initialize]));//1
    tab.push(Buffer.from([portfolio.asset_data_len]));//1
    let assets=[];
    console.log("length ", portfolio.asset_data.length);
    for (let i = 0; i < portfolio.asset_data.length; i++) {
//66
        console.log(" element "+i+" value "+portfolio.asset_data[i].amountAsset);
        assets.push(Buffer.from([portfolio.asset_data[i].amountAsset]));//1
        assets.push(portfolio.asset_data[i].addressAsset.toBuffer());//32
        console.log(" addres asset : "+portfolio.asset_data[i].addressAsset.toBase58());
        assets.push(Buffer.from([portfolio.asset_data[i].periodAsset]));//1
        assets.push(portfolio.asset_data[i].assetToSoldIntoAsset.toBuffer());//32
        console.log(" addres asset : "+portfolio.asset_data[i].assetToSoldIntoAsset.toBase58());
    
        
    } 
    const array3 = tab.concat(assets);
    let f = Buffer.concat(array3);
    client.write(f);

});
client.on('data', function (data) {
    console.log("reading : ", data);
    client.destroy(); // kill client after server's response
});
client.on('close', function () {
    console.log('Connection closed');
});