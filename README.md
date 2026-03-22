# 🔗 RWAgallery.art — Transmutation Protocol

**Bridging traditional fine art and the Kusama/Polkadot ecosystem through generative transmutation and RWA verification.**

> *"The NFT is not just a digital image. It is the cryptographic key that allows the collector to claim the original physical artwork."* — FDaniel

---

## 🌌 Overview

**RWAgallery.art** is a decentralized protocol that facilitates the secure verification, ownership and physical delivery of Real World Art Assets (RWA) within the Kusama ecosystem.

The core is the **Transmutation Protocol**: a multi-stage process where traditional paintings by **Pepe Semitiel Segura** are interpreted through sound and generative algorithms by **FDaniel** to become unique NFTs on the Kusama network. The NFT acts as the cryptographic key to claim the original physical artwork.

```
Physical Painting → Musical Composition → Generative Art → RWA NFT (Kusama)
```

---

## 🛠 Key Features

- **Gas-free Verification** — Cryptographic message signature to verify NFT ownership without spending gas. Legally valid under EU eIDAS regulation.
- **Non-custodial Escrow** — Decentralized escrow system. Buyer deposits price + 1.5% fee. Seller deposits 10-20% guarantee. Funds released on confirmed delivery.
- **Privacy-First Logistics** — Encrypted off-chain PostgreSQL storage for shipping data. Never on blockchain. GDPR compliant.
- **IPFS Evidence** — Photos, tracking and insurance stored permanently on IPFS before release of funds.
- **Automated COA Generator** — Digital Certificate of Authenticity minted as NFT to buyer's wallet on delivery confirmation.
- **Artists Directory** — Open registry for artists to join the protocol with their physical works.
- **3% Shared Commission** — 1.5% buyer + 1.5% seller. Transparent and fair.

---

## 🏗 Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | HTML5, CSS3, Vanilla JS (v1) → Next.js 14 (v2) |
| Blockchain | Kusama Asset Hub / Polkadot Asset Hub |
| Web3 | polkadot-js/api, Talisman, SubWallet, Nova Wallet |
| Storage | IPFS (evidence + COA metadata) |
| Database | PostgreSQL (encrypted off-chain) |
| Graphics | Three.js / p5.js (generative engine) |

---

## 📋 Fulfillment Workflow

```
1. CONNECT    → Collector connects wallet (Talisman / SubWallet / Nova)
2. SIGN       → Free message signature verifies NFT ownership (no gas)
3. CLAIM      → Secure RWA form unlocked. Collector submits encrypted address.
4. ESCROW     → Buyer + Seller funds locked. Non-custodial.
5. SHIP       → Artist ships with photos + tracking + insurance → IPFS
6. CONFIRM    → Buyer confirms receipt OR 7-day timer auto-releases
7. COA        → Digital Certificate of Authenticity minted to buyer wallet
8. RELEASE    → Payment released. 3% commission split. Cycle complete.
```

---

## 💰 Commission Model

| Party | Amount |
|-------|--------|
| Buyer pays | Price + shipping + **1.5%** |
| Seller receives | Price − **1.5%** |
| RWAgallery receives | **3% total** |

*Example: 100 KSM artwork → Buyer deposits 101.5 KSM · Seller receives 98.5 KSM · RWAgallery 3 KSM*

---

## 🎨 Inaugural Collection: Transmutación

**Semitiel × FDaniel** — The first RWA collection on Kusama.

- **Pepe Semitiel Segura** — Traditional painter. Founder of Centro de Arte Semitiel, Cieza (Murcia, Spain). Physical originals.
- **FDaniel** — Digital artist. Generative transmutation of Semitiel's works into audiovisual NFTs on Kusama.

NFTs minted on **Chaotic.art** (Kusama Asset Hub).

---

## 🚀 Roadmap

| Phase | Month | Milestone |
|-------|-------|-----------|
| 1 | M1 | Base infra · Wallet integration · Gas-free signature · GitHub |
| 2 | M2 | Generative engine · Transmutation Collection mint · Gallery |
| 3 | M3 | Escrow system · PostgreSQL · IPFS + COA generator |
| 4 | M4 | Beta test 10 artworks · Public launch · KSM Initiative report |

---

## 🔒 Privacy & Legal

- GDPR compliant (EU Regulation 2016/679)
- Shipping addresses: encrypted, deleted after delivery
- Blockchain data: public by nature (wallet addresses)
- Signatures: valid under eIDAS (EU Regulation 910/2014)
- Jurisdiction: Spain / European Union
- Full Terms & Conditions: [rwagallery.art/legal.html](https://fdaniel-art.github.io/rwagallery-art/legal.html)

---

## 🌐 Live

**Web:** https://fdaniel-art.github.io/rwagallery-art/
**Email:** rwagallery@proton.me
**X:** [@frandanvg](https://x.com/frandanvg)
**Wallet:** `13md2Kef1dbqQS43yHtoCxJzsFfxH1LNb5LVKdCVfP5z7pFw`

---

*KSM Art & Social Experiments Initiative · Est. 2022 · Expect chaos.*
