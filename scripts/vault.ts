import { PublicKey } from "@solana/web3.js";

const programId = new PublicKey(
  "4bbnXrCnpxtMLHBcbLqwsGAfnB8jfzVtFHxv8bHwrqad"
);

const [vaultAuthority] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault-authority")],
  programId
);

console.log("Vault Authority PDA:", vaultAuthority.toBase58());
