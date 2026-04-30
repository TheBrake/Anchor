import * as anchor from "@coral-xyz/anchor";   //Seccion de importaciones de las librerias necesarias para el proyecto, como Anchor, BN y Keypair.
import { Program } from "@coral-xyz/anchor";
import { MyProject } from "../target/types/my_project";
import { BN } from "@coral-xyz/anchor";


const provider = anchor.AnchorProvider.env();
const myWallet = provider.wallet as anchor.Wallet;
const transferAmount = 1000000; // Amount in lamports (1 SOL = 1,000,000,000 lamports)

describe("my_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.myProject as Program<MyProject>;

  it("SOL Transfer", async () => {
    const transactionSignature = await program.methods
      .solTransfer(new BN(transferAmount))
      .accounts({
        sender: provider.publicKey,
        recipient: myWallet.publicKey,
      })
      .signers([myWallet.payer])
      .rpc();

    console.log(
      `\nTransaction Signature:` +
      `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`,
    );
  });
});
