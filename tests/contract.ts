import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Contract } from "../target/types/contract";
import {
  TOKEN_PROGRAM_ID,
  createMint,
} from "@solana/spl-token";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.contract as Program<Contract>;
  const wallet = provider.wallet as anchor.Wallet;

  let yapMint: PublicKey;

  // Derive PDAs helpers
  const deriveUserPDA = async (username: string): Promise<[PublicKey, number]> => {
    return await PublicKey.findProgramAddressSync(
      [Buffer.from("user"), Buffer.from(username)],
      program.programId
    );
  };

  const derivePlatformPDA = async (): Promise<[PublicKey, number]> => {
    return await PublicKey.findProgramAddressSync(
      [Buffer.from("platform")],
      program.programId
    );
  };

  it("initiating platform", async () => {
    yapMint = await createMint(
      provider.connection,
      wallet.payer,
      wallet.publicKey, // mint authority
      null,
      6 // decimals
    );

    const [platformPda] = await derivePlatformPDA();

    const tx = await program.methods
      .initializePlatform()
      .accounts({
        authority: wallet.publicKey,
        yapMint: yapMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        // platform: platformPda,
        // systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Platform initialized:", tx);
  });

  it("creating user", async () => {
    const user_name = "User";
    const user_username = "User1";

    const [userPda] = await deriveUserPDA(user_username);
    const [platformPda] = await derivePlatformPDA();

    const tx = await program.methods
      .initializeUser(user_name, user_username)
      .accounts({
        payer: wallet.publicKey,
        // userCreated: userPda,
        // platform: platformPda,
        // systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ User created:", tx);

    // Optional: fetch and check values from the account
    const userAccount = await program.account.user.fetch(userPda);
    console.log("👤 User Account:", userAccount);
  });
});
