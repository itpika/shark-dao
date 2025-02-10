const { createUmi } = require('@metaplex-foundation/umi-bundle-defaults');
const {mplTokenMetadata,createV1, mintV1, TokenStandard, burnV1, freezeDelegatedAccount,} = require('@metaplex-foundation/mpl-token-metadata');
const web3 = require("@solana/web3.js");
const splToken = require("@solana/spl-token");



//require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key) || Pubkey::from_str("CkrBdLxYnMSB55hrBB9LE3Ave4GerSpgrpsUsqyTjdgk").unwrap().eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);


// let keypair = web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require('fs').readFileSync('C:\\Users\\pika\\solkey\\1.json'))));

// // Use the RPC endpoint of your choice.
// let umi = createUmi('http://127.0.0.1:8899').use(mplTokenMetadata());
// let umi = createUmi('https://api.devnet.solana.com').use(mplTokenMetadata());
// let conn = new web3.Connection('https://api.devnet.solana.com');
let conn = new web3.Connection('https://api.mainnet-beta.solana.com');
const keypair =  web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require('fs').readFileSync('/home/pika/.config/solana/id.json'))));
// const keypair = umi.eddsa.createKeypairFromSecretKey()));
console.log('keypair address',keypair.publicKey.toBase58());

let mint = new web3.PublicKey('EBsivAuMmq6UJnKuEEJKBNMYCnYeFyMBYG9MNnUUmoUP');
(async () => {
    let mintObj = await splToken.getMint(conn, mint);
    // console.log(mintObj);
    // let tx = await splToken.setAuthority(conn, keypair, mint, keypair, splToken.AuthorityType.MintTokens, null);
    let tx = await splToken.setAuthority(conn, keypair, mint, keypair, splToken.AuthorityType.FreezeAccount, null);
    console.log(tx);
    // let addr = splToken.getMintCloseAuthority(mintObj)
    // console.log('addr',addr);

})();


