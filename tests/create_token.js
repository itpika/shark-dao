const { createUmi } = require('@metaplex-foundation/umi-bundle-defaults');
const UMI = require('@metaplex-foundation/umi');
const {mplTokenMetadata,createV1, mintV1, TokenStandard, burnV1, freezeDelegatedAccount,} = require('@metaplex-foundation/mpl-token-metadata');
const web3 = require("@solana/web3.js");
const {irysUploader} = require("@metaplex-foundation/umi-uploader-irys");
const aaa = require('@metaplex-foundation/mpl-token-metadata');



//require!(ctx.accounts.state.admin.eq(ctx.accounts.payer.key) || Pubkey::from_str("CkrBdLxYnMSB55hrBB9LE3Ave4GerSpgrpsUsqyTjdgk").unwrap().eq(ctx.accounts.payer.key), ErrorCode::NotAuthorized);


// let keypair = web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require('fs').readFileSync('C:\\Users\\pika\\solkey\\1.json'))));

// // Use the RPC endpoint of your choice.
// let umi = createUmi('http://127.0.0.1:8899').use(mplTokenMetadata());
// let umi = createUmi('https://api.devnet.solana.com').use(mplTokenMetadata());
let umi = createUmi('https://api.mainnet-beta.solana.com').use(mplTokenMetadata());
const keypair = umi.eddsa.createKeypairFromSecretKey(Buffer.from(JSON.parse(require('fs').readFileSync('/home/pika/.config/solana/id.json'))));
console.log('keypair address',keypair.publicKey);
const { base58 } = require("@metaplex-foundation/umi/serializers");

umi = umi.use(UMI.keypairIdentity(keypair));


let mint = UMI.generateSigner(umi);
// mint = new web3.PublicKey('F7fgtQtoMAstyXtoyuDXf6oazS1yrszn24EW5GA7kZxy');
console.log('mint address', mint.publicKey);
// create mint
(async () => {
    let hs = null;
    hs = await createV1(umi, {
        mint,
        name: 'shark dao',
        symbol: 'sharkdao',
        uri: 'https://red-written-sturgeon-318.mypinata.cloud/ipfs/QmNvTu5LnZiNQ2PAkEEtqxCXMmTwYpkKbrX1MiQYf1skqr',
        decimals: UMI.some(6), // for 0 decimals use some(0)
        tokenStandard: TokenStandard.Fungible,
        sellerFeeBasisPoints: UMI.percentAmount(5.5),
    }).sendAndConfirm(umi)


    console.log(base58.deserialize(hs.signature).toString());

    hs = await mintV1(umi, {
        mint,
        authority: keypair,
        amount: 1000000000000000,
        tokenOwner: keypair.publicKey,
        tokenStandard: TokenStandard.Fungible,
    }).sendAndConfirm(umi);



    console.log(base58.deserialize(hs.signature).toString());
})();

// mint
(async () => {
    // umi = umi.use(irysUploader())
    // let filebyte = require('fs').readFileSync('./token.jfif');
    //
    // await createGenericFileFromBrowserFile("./token.jfif")
    //
    // const [imageUri] = await umi.uploader.upload([await createGenericFileFromBrowserFile("./token.jfif")]);
    // // const [imageUri] = await umi.uploader.upload([createGenericFile(filebyte, 'shark-dao.jfif')]);
    // console.log('imageUri', imageUri);
    // const uri = await umi.uploader.uploadJson({
    //     name: 'shark-dao',
    //     description: 'shark-dao',
    //     image: imageUri,
    // });
    // console.log('uri', uri);
    // return

    // hs = await mintV1(umi, {
    //     mint: new PublicKey('7VopHDN1nyZtG1HGHJ66gJixxnc6Bh6sGyCCAnb7btBm'),
    //     authority: keypair,
    //     amount: 300000000000000,
    //     // tokenOwner: keypair.publicKey,
    //     tokenOwner: 'AUm4ddsrEvyuhy7TLuL5Sw8Sa5gWf3n5MfaoSMBC17rD',
    //     tokenStandard: TokenStandard.Fungible,
    // }).sendAndConfirm(umi);
    //
    // console.log(base58.deserialize(hs.signature).toString());
})();
// burn
// (async () => {
//     hs = await burnV1(umi, {
//         mint: new PublicKey('4kJiWeXmduR6UMd7s8vvJyoMz16hdE7acjWNH7VMPkLV'),
//         authority: keypair,
//         amount: 5064000047,
//         tokenOwner: keypair.publicKey,
//         tokenStandard: TokenStandard.Fungible,
//         splTokenProgram: new PublicKey('TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb')
//     }).sendAndConfirm(umi);

//     console.log(base58.deserialize(hs.signature).toString());
// })();


// BYAiFtHDcGxbDVNAGeWDhq8nGZcS6nPYHyDewWfpKpLu