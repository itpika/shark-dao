const { createUmi } = require('@metaplex-foundation/umi-bundle-defaults');
const {mplTokenMetadata,createV1, mintV1, TokenStandard, burnV1, freezeDelegatedAccount,} = require('@metaplex-foundation/mpl-token-metadata');
const web3 = require("@solana/web3.js");
const splToken = require("@solana/spl-token");



//require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key) || Pubkey::from_str("CkrBdLxYnMSB55hrBB9LE3Ave4GerSpgrpsUsqyTjdgk").unwrap().eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);


// let keypair = web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require('fs').readFileSync('C:\\Users\\pika\\solkey\\1.json'))));

// // Use the RPC endpoint of your choice.
// let umi = createUmi('http://127.0.0.1:8899').use(mplTokenMetadata());
// let umi = createUmi('https://api.devnet.solana.com').use(mplTokenMetadata());
let conn = new web3.Connection('https://api.devnet.solana.com');
// let conn = new web3.Connection('https://api.mainnet-beta.solana.com');
const keypair =  web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require('fs').readFileSync('/home/pika/.config/solana/id.json'))));
// const keypair = umi.eddsa.createKeypairFromSecretKey()));
console.log('keypair address',keypair.publicKey.toBase58());

(async () => {
    let slot = await conn.getSlot();
    let minRent = await conn.getMinimumBalanceForRentExemption(0);
    let blockhash = await conn.getLatestBlockhash();
    let info = (await conn.getAddressLookupTable(new web3.PublicKey('6MXVfzJSJFsroVwuctKXVBkKVj6FPy1XbqXJFTGM332j'))).value;

    // console.log(info.key.toBase58(), info.isActive());
    // console.log(info.state);

    let instr = web3.SystemProgram.transfer({
        fromPubkey: keypair.publicKey,
        toPubkey: new web3.PublicKey('43YU4Jo6R3fHYh7HMpZsdGomJSUzywrjkocmmkSSBGoh'),
        lamports: minRent,
    });

    let messageV0 = new web3.TransactionMessage({
        payerKey: keypair.publicKey,
        instructions: [instr],
        recentBlockhash: blockhash.blockhash,
    }).compileToV0Message();

    const transction = new web3.VersionedTransaction(messageV0);

    transction.sign([keypair]);

    let txId = await web3.sendAndConfirmTransaction(conn, transction);
    console.log(txId);
})();

(async () => {

    let traninfo = await conn.getParsedTransaction('3uTVEtWSXSgq93M1aUmr2ffnsGhUcuTV3LyBdu6WJw3dgbG3jBD85YS1c1o1x5zxChXQYRhDPKVtWigEjVciUfqa', {
        maxSupportedTransactionVersion: 0,
    });
    console.log(traninfo);
    return

    let blockhash = await conn.getLatestBlockhash();


    const inst = web3.AddressLookupTableProgram.extendLookupTable({
        authority: keypair.publicKey,
        payer: keypair.publicKey,
        lookupTable: new web3.PublicKey('6MXVfzJSJFsroVwuctKXVBkKVj6FPy1XbqXJFTGM332j'),
        addresses: [
            new web3.PublicKey('6MXVfzJSJFsroVwuctKXVBkKVj6FPy1XbqXJFTGM332j')
        ]
    });

    let lookTx = new web3.VersionedTransaction(new web3.TransactionMessage({
        payerKey: keypair.publicKey,
        recentBlockhash: blockhash.blockhash,
        instructions: [inst]
    }).compileToV0Message());
    lookTx.sign([keypair]);

    let lookHash = await web3.sendAndConfirmTransaction(conn, lookTx);

    console.log(lookHash);
})();

(async () => {
    return
    let slot = await conn.getSlot();
    let minRent = await conn.getMinimumBalanceForRentExemption(0);
    let blockhash = await conn.getLatestBlockhash();

    let [lookupTableInst, lookupAddress] = web3.AddressLookupTableProgram.createLookupTable({
        authority: keypair.publicKey,
        payer: keypair.publicKey,
        recentSlot: slot,
    });

    console.log("lookup address", lookupAddress.toBase58()); // 6MXVfzJSJFsroVwuctKXVBkKVj6FPy1XbqXJFTGM332j

    const inst = web3.AddressLookupTableProgram.extendLookupTable({
        authority: keypair.publicKey,
        payer: keypair.publicKey,
        lookupTable: lookupAddress,
        addresses: [
            keypair.publicKey,
            web3.SystemProgram.programId,
        ]
    });

    let lookTx = new web3.VersionedTransaction(new web3.TransactionMessage({
        payerKey: keypair.publicKey,
        recentBlockhash: blockhash.blockhash,
        instructions: [lookupTableInst, inst]
    }).compileToV0Message());
    lookTx.sign([keypair]);

    let lookHash = await web3.sendAndConfirmTransaction(conn, lookTx);

    console.log(lookHash);
    return

    let traninfo = await conn.getParsedTransaction('23KP7QunzNwsrvif26ouKfU3pnjNEqpAq9AzkwVvxgTa7DL7U6owNLCRWh7Xt1r5fqQJHs71MY5o5Y1aVSDSepru');
    console.log(traninfo);
    return

    let instr = web3.SystemProgram.transfer({
        fromPubkey: keypair.publicKey,
        toPubkey: new web3.PublicKey('43YU4Jo6R3fHYh7HMpZsdGomJSUzywrjkocmmkSSBGoh'),
        lamports: minRent,
    });

    let messageV0 = new web3.TransactionMessage({
        payerKey: keypair.publicKey,
        instructions: [instr],
        recentBlockhash: blockhash.blockhash,
    }).compileToV0Message();

    const transction = new web3.VersionedTransaction(messageV0);

    transction.sign([keypair]);

    let txId = await web3.sendAndConfirmTransaction(conn, transction);
    console.log(txId);

})();


