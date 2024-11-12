import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { SharkDao } from "../target/types/shark_dao";
import BN from "bn.js";
// import mysql from "mysql2";
import * as mysql from 'mysql2/promise';
describe("shark-dao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SharkDao as Program<SharkDao>;
  let wallet = anchor.Wallet.local().payer;

  console.log("program", program.programId.toBase58());
  console.log("wallet", wallet.publicKey.toBase58());


  // let result = await db_conn.query("SELECT * FROM tk_lock")
  // console.log(result);
  // return
  let targetAccount = 'HydGLEyt6C9CWQhokDPUqxzZdvPQewSQ6ecKVPTrs6t8'
  let mint = 'FBQAsNhTiQSWyDL7NGz8w9fV9BVqLbUSviWRy8McbTXU'
  let num = new BN(2000*1000000);
  let etm = new BN(1731312304);
  let now = new Date().getTime()
  let [lock_info] = web3.PublicKey.findProgramAddressSync([Buffer.from("lock_info"),
    new web3.PublicKey(mint).toBuffer(),
    new web3.PublicKey(targetAccount).toBuffer(),
  ], program.programId);
  console.log("lock_info", lock_info.toBase58());

  
  it("Is initialized!", async () => {
    let db_conn = await mysql.createConnection({
      database: 'token',
      host: "8.222.214.50",
      port: 3306,
      user: 'token',
      password: 'GJbHA6aMt2bXWrrH'.toString(),
    })
    await db_conn.connect();

    // Add your test here.
    const tx = await program.methods.lockToken(num, etm).
    accounts({
      mint,
      payer: wallet.publicKey,
      targetAccount,
    }).signers([wallet]).rpc();
    console.log("Your transaction signature", tx);

    await db_conn.execute(
        `INSERT INTO tk_lock(\`user_addr\`, \`hash\`, \`token_num\`, \`ctm\`, \`lock_etm\`, \`chain_time\`, \`mint\`, \`lock_account\`)
        VALUES ('${targetAccount}', '${tx}', '${num.toString()}', ${now}, ${etm.toNumber()}, ${now}, '${mint}', '${lock_info.toBase58()}')`)
  });
});
