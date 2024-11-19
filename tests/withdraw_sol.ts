import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { SharkDao } from "../target/types/shark_dao";
import BN from "bn.js";

describe("shark-dao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SharkDao as Program<SharkDao>;
  let wallet = anchor.Wallet.local().payer;

  console.log("program", program.programId.toBase58());
  console.log("wallet", wallet.publicKey.toBase58());
  let preorder_name = "first_preorders"
  let [preorder] = web3.PublicKey.findProgramAddressSync([Buffer.from("PREORDER"),
    Buffer.from(preorder_name),
  ], program.programId);
  console.log("preorder", preorder.toBase58());

  let [state] = web3.PublicKey.findProgramAddressSync([Buffer.from("state"),
  ], program.programId);

  it("Is initialized!", async () => {
    let preorderInfo = await program.account.preOrder.fetch(preorder);
    console.log('preorderInfo', preorderInfo)

    // Add your test here.
    const tx = await program.methods.withdrawSol(new BN(100000000)).
    accounts({}).signers([wallet]).rpc();
    console.log("Your transaction signature", tx);

  });
});
