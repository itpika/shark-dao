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


  let [userPreorder] = web3.PublicKey.findProgramAddressSync([Buffer.from("USER_PREORDER_SOL"),
    (new web3.PublicKey(preorder)).toBuffer(),
    wallet.publicKey.toBuffer(),
  ], program.programId);
  console.log("userPreorder", userPreorder.toBase58());
  it("Is initialized!", async () => {
    let preorderInfo = await program.account.preOrder.fetch(preorder);
    console.log('preorderInfo', preorderInfo)

    // Add your test here.
    const tx = await program.methods.preorderTokenSol(preorder_name, new BN(50000000000)).
    accounts({
      mint: 'FBQAsNhTiQSWyDL7NGz8w9fV9BVqLbUSviWRy8McbTXU',
      payer: wallet.publicKey,
    }).signers([wallet]).rpc();
    console.log("Your transaction signature", tx);

    let orderInfo = await program.account.userSolPreOrder.fetch(userPreorder);
    console.log(orderInfo.amount.toNumber(), orderInfo.buyAmount.toNumber())
  });
});
