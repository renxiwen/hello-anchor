import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider ,BN} from "@coral-xyz/anchor";
import { CpiPdaDemo } from "../target/types/cpi_pda_demo";
import { sendAndConfirmTransaction, SystemProgram,Transaction } from "@solana/web3.js";

describe("cpi_pda_demo", () => { 
  const provider = AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet;
  const program = anchor.workspace.CpiPdaDemo as Program<CpiPdaDemo>;
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("pda"), wallet.publicKey.toBuffer()],
    program.programId
  );
  const lamports = 1000000;
  it("Fund PDA with SOL", async () => {
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: pda,
      lamports,
    });
    const transaction = new Transaction().add(transferInstruction);
    
    const transactionSignature = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer]);
    
    console.log(
      `\nTransaction Signature:` +
      `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`,
    );
  });
  
  it("SOL Transfer with PDA signer", async () => {
    const transactionSignature = await program.methods
      .solTransfer(new BN(lamports))
      .accounts({
        recipient: wallet.publicKey
      }).rpc();
    console.log(
        `\nTransaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`,
      );
  })
})
