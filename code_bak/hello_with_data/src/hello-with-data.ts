import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWithData } from "../../../../target/types/hello_with_data";
import * as assert from "assert";

describe("hello-with-data", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HelloWithData as Program<HelloWithData>;

  it("Initializes an account with data", async () => {
    // Create a new account keypair
    const newAccount = anchor.web3.Keypair.generate();
    const initialData = new anchor.BN(42);

    // Execute the initialize instruction
    await program.methods.initialize(initialData)
      .accounts({
        newAccount: newAccount.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([newAccount])
      .rpc();

    // Fetch the newly created account
    const account = await program.account.newAccount.fetch(newAccount.publicKey);

    // Verify the data was stored correctly
    assert.ok(account.data.eq(initialData));
    console.log("✅ Account initialized with data:", account.data.toString());
  });


});