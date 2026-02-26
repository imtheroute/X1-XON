# ✅ Program Verxification Statement

**Program ID:** `D5Ssp6VDvrZ7u6ccmM8UHggoBr8PxkkGZvijo4HbmPyX`
**Source Code:** https://github.com/imtheroute/X1-XON
**Commit Hash:** `0471b5f9e814254ee5b9724bdb1e519899623a54`
**Built Program Hash:** `dc31e4559d83857b3c88f3bc52a60530e77343a9891c015fb6b04a474243d5a8`
**On-chain Program Hash:** `dc31e4559d83857b3c88f3bc52a60530e77343a9891c015fb6b04a474243d5a8`

## 🔍 Verification Details

- **Verification Tool:** solana-verify
- **Solana Version:** 2.3.0
- **Build Method:** Deterministic Docker build
- **Verification Date:** February 26, 2026
- **Verifier:** @imtheroute (GitHub: https://github.com/imtheroute)

## ✅ Result

**HASHES MATCH - Program is verified!**

The on-chain program at `D5Ssp6VDvrZ7u6ccmM8UHggoBr8PxkkGZvijo4HbmPyX`
was built from the source code at:
https://github.com/imtheroute/X1-XON/tree/0471b5f9e814254ee5b9724bdb1e519899623a54

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
  --commit-hash 0471b5f9e814254ee5b9724bdb1e519899623a54 \
  --library-name project_x \
  --mount-path .
