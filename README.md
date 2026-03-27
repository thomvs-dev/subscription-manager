# Subscription Manager (Orange Belt 🟠)

This project satisfies the Level 3 requirement for the Stellar developer bootcamp by delivering an **end-to-end dApp with premium quality**, robust React caching, and visual loading states—designed to be presented elegantly in the required 1-minute demo video.

## 🚀 Features & Requirements
✅ **Fully Functional dApp:** Integrates cleanly with a custom Soroban Subscription Vault smart contract allowing fixed-interval cycle payments.
✅ **Minimum 3 Tests Passing:** Robust Rust tests confirming balance locking, successful periodic payouts, and user cancelation. Run via `cargo test` in `./contracts/subscription`.
✅ **Loading States:** Implemented skeleton shimmering interfaces during the 'mock' network traversal and loading spinners on all transaction requests so users are never in the dark.
✅ **Basic Caching Implemented:** Active plan metadata leverages standard `localStorage` caching mechanics bounding RPC calls strictly within a 60-second Time-To-Live constraint (`plans_cache_time`).

## 🎥 Prepping for your Demo Video
To capture the 1-minute review video as required by L3 constraints:
1. **Host/Create:** As the Provider wallet, click the "Create Plan" button (Interval 60s). Wait for the ✅ Success Toast.
2. **Setup User:** Log out and log into a secondary "User" wallet testnet account. Switch to "User Dashboard" view.
3. **Showcase UI Caching:** Refresh the page to demonstrate how the "Skeleton loaders" appear briefly the first time, but instantly load on subsequent navigation (via the caching hooks!).
4. **Subscribe:** Submitting the "Subscribe" operation to load XLM into the vault.
5. **Collect:** Fast forward or wait ~1 min on video. Log back into the "Provider" account and run "Collect" against the user's ID to demonstrate interval timing.

## 🧰 Tech Stack
- **Smart Contract:** Rust / Soroban SDK v22.0.0
- **Frontend App:** Vite + React + TypeScript + `localStorage`
- **Styling:** Premium vanilla CSS with custom skeleton loading keyframes. 

## 🛜 Usage & Local Setup
### 1. Start the React node
```bash
cd frontend
npm install
npm run dev
```

### 2. Connect Your Wallet
Ensure you have the Freighter extension installed and set to the Stellar Testnet.

## 📝 Demo & Submission Links
- **Deployed Contract Address:** `CBUTQ5Z5H73Q5FKPBZN3BWVD7AKPRBQ7NER7KGOBULYGKTA36FO6I5FT`
- **Frontend Source:** `/frontend`
- **Contract Source:** `/contracts/subscription`
