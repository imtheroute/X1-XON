import { PublicKey } from "@solana/web3.js";

const programId = new PublicKey(
  "D5Ssp6VDvrZ7u6ccmM8UHggoBr8PxkkGZvijo4HbmPyX"
);

const [vaultAuthority] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault-authority")],
  programId
);

console.log("Vault Authority PDA:", vaultAuthority.toBase58());
