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


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.newPreorder("three", new BN(10000*1000000), new BN(50000), new BN(1730896800), new BN(1735689600)).
    accounts({
      mint: 'FBQAsNhTiQSWyDL7NGz8w9fV9BVqLbUSviWRy8McbTXU',
      collectionMint: '6DfkPaFibZPXyzfYCEMMx1bNJ6UCBwTREGrzbgDQDUxp',
      payer: wallet.publicKey,
    }).signers([wallet]).rpc();
    console.log("Your transaction signature", tx);
  });
});
