# Development Milestones

## First RP Node online

## Oracle online
- Optional identity brokering
- Wallet integration

# Marketing Milestones

## Website online
- Simple concept/marketing/links
- Dashboard online - RP Analytics
- RP launcher online - user pays solana to launch RP
- RP node owner earns RPR for running RP based on metrics submitted to Oracle

::: mermaid
---
config:
  theme: neutral
---
gantt
    title Code VM Roadmap
    dateFormat YYYY-MM-DD
    axisFormat %d-%b-%Y
    section Development
    First RP Node online :dev1, 2025-04-01, 30d
    Oracle online :dev2, after dev1, 45d
    Wallet integration :dev4, after mar4, 30d
    section Marketing
    Website online :mar1, 2025-04-01, 30d
    Simple concept/marketing/links :mar1, 2025-04-01, 30d
    Dashboard online - RP Analytics:mar3, 2025-05-15, 30d
    RP launcher online :mar4, after mar3, 15d
    RP node owner earns RPR :mar5, after mar4, 30d

:::

# Budget

RPR (https://solscan.io/token/Gor9owzBYMnBoeAefLqdHw1wJg5Q1dLJPkCa8Ms4HXCF) is a fully distributed token with no MINT and no freeze authority. Total circulation is 1 billion RPR.

(83.6%) Airdropped to select KIN holders

(~6.4%) Locked CLMM liquidity on Raydium;
- Pool ID 1 (~50.5 million):DkoAbDfgowFA32wdw1fWZFQH72CMAHeboq3UkZc9Tymd 
- Pool ID 2 (~12.8 million): G4s31PALLoFy4hMLQtBtRXmZ2HYmuj1mYjT3GeD8kTY9

Pools have tiered pricing to absorb approximately 500 billion KIN, creating a strong link betweent these markets.

## Rewards

(10%) 100m RPR locked in immutable JUP Lock contract(https://solscan.io/account/5yS3ehyTAxPZBmZBKirqUHAbSogYpF5w1mAKa7L6pCqS)

952,380.95 RPR vests weekly over 2 years. Loosely to be distributed as follows:

## Development Rewards

- Development 60% (60 million)
-- RP Node online (5 million)
-- Oracle online (10 million)
-- Oracle Dashboard (10 million)
-- Wallet integration (10 million)
-- Maintenance and Updates (25 million)

Development rewards will be distributed among all technical contributors proportionally (even split). Distribution may be refined to include metrics such as % LoC merged to main branch should development interest scale.

## Marketing Rewards

- RP Operator Rewards 30% (30 million)
-- Operator rewards for nodes submitting metrics (TBD) to the oracle.

- Advertising/Marketing 10% (10 million)
-- 10 million for publicity and marketing activities
