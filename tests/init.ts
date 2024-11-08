import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { SharkDao } from "../target/types/shark_dao";

describe("shark-dao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SharkDao as Program<SharkDao>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.init().rpc();
    console.log("Your transaction signature", tx);
  });
});
