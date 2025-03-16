## Voting Mechanism Limitations

This mechanism is only suited for simple votes within user-authenticated environments e.g. FlipChat. for reasons outlined below.

### Threshold and Time Limit Tracking

The current implementation provides basic time-based validation e.g. tracking `is_poll_active()` and `verify_vote_validity_with_expiry()` for validity and eligibility of voters/voting based on timelock expiry limit > vote expiry time.

More sophisticated time-based thresholds require client-side implementation. No mechanism is included for queueing operations.


## To Do:
### Reward Distribution Optimisation

Batch processing e.g. using "airdop" could be beneficial to reduce transaction costs.

### Auditability and Transparency

Vote logs or event emissions to facilitate external verification of voting processes would be helpful.

### Quorum Requirements


## No Plans to Implement:

### Vote Weight Calculation

`calculate_winning_option()` provides a simple majority-based calculation.

### Vote Revocation and Modification
### Sybil Resistance

::: mermaid
sequenceDiagram
    participant User
    participant CodeVmAccount as VM
    participant Creator as Creator VTA
    participant Voter as Voter VTA
    participant OptionAcc as Option VTA
    participant VDN as VirtualDurableNonce

    %% Poll Creation
    User->>Creator: Initiate poll creation
    Creator->>CodeVmAccount: Request poll creation
    CodeVmAccount->>+Creator: create_poll_metadata()
    Creator->>VDN: Get nonce
    VDN-->>Creator: Return nonce
    Creator->>-CodeVmAccount: create_poll_init_message_with_expiry()
    CodeVmAccount-->>User: Return poll initialization hash
    User->>CodeVmAccount: Submit poll_init transaction (opcode 0x02)
    CodeVmAccount->>CodeVmAccount: process_poll_init()
    CodeVmAccount->>CodeVmAccount: Initialize option accounts with zero balance
    CodeVmAccount->>VDN: Update nonce
    CodeVmAccount-->>User: Poll created successfully

    %% Voting Process
    User->>Voter: Initiate vote
    Voter->>CodeVmAccount: Request vote validation
    CodeVmAccount->>+Voter: verify_vote_validity_with_expiry()
    Voter->>-CodeVmAccount: Vote is valid
    Voter->>VDN: Get nonce
    VDN-->>Voter: Return nonce
    Voter->>CodeVmAccount: create_vote_message()
    CodeVmAccount-->>Voter: Return vote hash
    User->>CodeVmAccount: Submit vote transaction (opcode 0x01)
    CodeVmAccount->>CodeVmAccount: process_vote()
    CodeVmAccount->>CodeVmAccount: Check voter has sufficient funds
    CodeVmAccount->>Voter: Deduct tokens
    CodeVmAccount->>OptionAcc: Add tokens (record vote)
    CodeVmAccount->>VDN: Update nonce
    CodeVmAccount-->>User: Vote recorded successfully

    %% Poll Completion
    User->>Creator: Request poll completion
    Creator->>CodeVmAccount: Check if poll expired
    CodeVmAccount->>+Creator: is_poll_active()
    Creator->>-CodeVmAccount: Poll is expired
    Creator->>CodeVmAccount: calculate_winning_option()
    CodeVmAccount-->>Creator: Return winning option index
    Creator->>VDN: Get nonce
    VDN-->>Creator: Return nonce
    Creator->>CodeVmAccount: create_poll_complete_message()
    CodeVmAccount-->>Creator: Return poll completion hash
    User->>CodeVmAccount: Submit poll_complete transaction (opcode 0x03)
    CodeVmAccount->>CodeVmAccount: process_poll_complete()
    CodeVmAccount->>CodeVmAccount: Verify winner has highest balance
    CodeVmAccount->>OptionAcc: Reset all option account balances
    CodeVmAccount->>VDN: Update nonce
    CodeVmAccount-->>User: Poll completed, winner determined

    %% Optional: Reward Distribution
    Creator->>OptionAcc: Request reward distribution
    OptionAcc->>VDN: Get nonce
    VDN-->>OptionAcc: Return nonce
    OptionAcc->>CodeVmAccount: create_reward_distribution_message()
    CodeVmAccount-->>OptionAcc: Return reward distribution hash
    User->>CodeVmAccount: Submit reward distribution transaction
    CodeVmAccount->>Creator: Deduct reward tokens
    CodeVmAccount->>Voter: Distribute rewards to winning voters
    CodeVmAccount-->>User: Rewards distributed successfully

    %% Optional: Vote Refund (if poll cancelled)
    alt Poll is cancelled
        Creator->>VDN: Get nonce
        VDN-->>Creator: Return nonce
        Creator->>CodeVmAccount: create_vote_refund_message()
        CodeVmAccount-->>Creator: Return refund hash
        User->>CodeVmAccount: Submit refund transaction
        CodeVmAccount->>OptionAcc: Deduct tokens
        CodeVmAccount->>Voter: Return tokens to voters
        CodeVmAccount-->>User: Votes refunded successfully
    end

:::