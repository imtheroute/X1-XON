import * as anchor from "@coral-xyz/anchor";
import idl from "../target/idl/xon_time_mint.json";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Program is resolved from IDL metadata.address
const program = new anchor.Program(
  idl as anchor.Idl,
  provider
);

async function main() {
  const creator = new anchor.web3.PublicKey(
    "PEk9CSUuyU6sMsS839HSxRmXmxKssC62Q1NjBzu21Zp"
  );

  const [globalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const tx = await program.methods
    .initialize(creator)
    .accounts({
      global: globalPda,
      payer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log("✅ Initialize successful");
  console.log("TX:", tx);
}

main().catch(console.error);

