// import * as anchor from "@coral-xyz/anchor";
// import { Program, BN } from "@coral-xyz/anchor";
// import { Contract } from "../target/types/contract";
// import { Keypair } from "@solana/web3.js";

// describe("contract", () => {
//   // Configure the client to use the local cluster.
//   const provider = anchor.AnchorProvider.env();
//   const wallet = provider.wallet as anchor.Wallet;
//   const program = anchor.workspace.contract as Program<Contract>;

//   it("Creating Room!", async () => {
//     const Newacc=new Keypair();
//     console.log(Newacc.publicKey)
//     // Add your test here.
//     const tx = await program.methods.createRoom(
//       "Room1",
//       new BN(1),
//       new BN(3),
//       new BN(40)
//     ).accounts({
//       payer:wallet.publicKey,
      
//     }).signers([Newacc]).rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Contract } from "../target/types/contract";
import { PublicKey, Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.contract as Program<Contract>;

  it("Creating Room and Verifying!", async () => {
    const roomId = new BN(1);
    const roomTitle = "Room1";
    const startTs = new BN(Math.floor(Date.now() / 1000)); // UNIX timestamp in seconds
    const duration = new BN(3600); // 1 hour

    // Generate a new keypair for the room account (since we're not using PDA anymore)
    const yapRoomKeypair = Keypair.generate();

    // Create Room
    const tx = await program.methods
      .createRoom(roomTitle, roomId, startTs, duration)
      .accounts({
        payer: wallet.publicKey,
        // yapRoom: yapRoomKeypair.publicKey,
      })
      .signers([yapRoomKeypair]) // Need to sign with the room keypair
      .rpc();

    console.log("✅ Room created. Tx:", tx);
    console.log("🔑 Room Account Address:", yapRoomKeypair.publicKey.toBase58());

    // Fetch Room Data
    const roomAccount = await program.account.yapRoom.fetch(yapRoomKeypair.publicKey);

    console.log("📄 Room Data:");
    console.log("Room ID:", roomAccount.roomId.toString());
    console.log("Title:", roomAccount.roomTitle);
    console.log("Start Time:", roomAccount.startTime.toString());
    console.log("Duration:", roomAccount.duration.toString());
    console.log("Creator:", roomAccount.hostPubkey.toBase58());

    // Assertions to verify the data
    expect(roomAccount.roomId.toString()).to.equal(roomId.toString());
    expect(roomAccount.roomTitle).to.equal(roomTitle);
    expect(roomAccount.startTime.toString()).to.equal(startTs.toString());
    expect(roomAccount.duration.toString()).to.equal(duration.toString());
    expect(roomAccount.hostPubkey.toBase58()).to.equal(wallet.publicKey.toBase58());

    console.log("✅ All assertions passed!");
  });
});