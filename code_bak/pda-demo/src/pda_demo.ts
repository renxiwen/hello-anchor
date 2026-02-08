import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program} from "@coral-xyz/anchor";
import { PdaDemo } from "../../../target/types/pda_demo";

describe("pda-demo",() => {
    anchor.setProvider(AnchorProvider.env());

    const program = anchor.workspace.PdaDemo as Program<PdaDemo>;
    it('pda demo', async () => {
        const tx = await program.methods.initialize().rpc();
        console.log("Your transaction signature", tx);
    });
})