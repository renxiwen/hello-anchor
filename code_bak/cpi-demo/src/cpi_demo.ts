import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import { CpiDemo } from "../target/types/cpi_demo"; 
import { BN } from "bn.js";

describe("cpi-demo",() => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.CpiDemo as Program<CpiDemo>;
    const wallet = provider.wallet as anchor.Wallet;
    const transferAmount = new BN(1000000000);
    const receiver = anchor.web3.Keypair.generate();

    it("should create a new cpi demo", async () => {
        const transactionSignature = await program.methods
        .solTransfer(transferAmount)
        .accounts({
            sender: wallet.publicKey,
            receiver: receiver.publicKey,
        })
        .rpc();
        console.log("transactionSignature", transactionSignature);
    });

})