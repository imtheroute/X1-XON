# ✅ Program Verification Statement

**Program ID:** `D5Ssp6VDvrZ7u6ccmM8UHggoBr8PxkkGZvijo4HbmPyX`
**Source Code:** https://github.com/imtheroute/X1-XON
**Commit Hash:** `15af3c351201eface4378edb9dbed9ca01e6d88e`
**Built Program Hash:** `6abc482af243ce04fea4dad06022d10cc2fa9342335565a3ae614fed870c2586`
**On-chain Program Hash:** `6abc482af243ce04fea4dad06022d10cc2fa9342335565a3ae614fed870c2586`

## 🔍 Verification Details

- **Verification Tool:** solana-verify
- **Solana Version:** 2.3.0
- **Build Method:** Deterministic Docker build
- **Verification Date:** February 23, 2026
- **Verifier:** @imtheroute (GitHub: https://github.com/imtheroute)

## ✅ Result

**HASHES MATCH - Program is verified!**

The on-chain program at `D5Ssp6VDvrZ7u6ccmM8UHggoBr8PxkkGZvijo4HbmPyX` 
was built from the source code at:
https://github.com/imtheroute/X1-XON/tree/15af3c351201eface4378edb9dbed9ca01e6d88e

## 🔐 Commit Signature Verification

This commit is cryptographically signed:
- Signed by: imtheroute
- Signature type: SSH
- Verification status: ✅ Valid

## 📊 Verification Command

```bash
solana-verify verify-from-repo \
  --url https://rpc.mainnet.x1.xyz \
  --program-id D5Ssp6VDvrZ7u6ccmM8UHggoBr8PxkkGZvijo4HbmPyX \
  https://github.com/imtheroute/X1-XON \
  --commit-hash 15af3c351201eface4378edb9dbed9ca01e6d88e \
  --library-name project_x \
  --mount-path .

# View the file
cat VERIFICATION.md

# Check file details
ls -la VERIFICATION.md
