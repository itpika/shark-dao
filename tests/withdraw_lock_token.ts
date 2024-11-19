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



  let mint = 'FBQAsNhTiQSWyDL7NGz8w9fV9BVqLbUSviWRy8McbTXU'
  let num = new BN(200*1000000);
  let etm = new BN(1731920400);
  let now = new Date().getTime()
  let [lock_info] = web3.PublicKey.findProgramAddressSync([Buffer.from("lock_info"),
    new web3.PublicKey(mint).toBuffer(),
    new web3.PublicKey(wallet.publicKey).toBuffer(),
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
    const tx = await program.methods.withdrawUnlockToken().
    accounts({
      mint,
    }).signers([wallet]).rpc();
    console.log("Your transaction signature", tx);

  });
});
