# VM (Solana Program) Functional Summary

## 1. Core VM Management

### VM Initialization
- Token mint association
- Configurable timelock duration
- Dedicated omnibus token account
- Authority-based control system

## 2. Memory Management

### Memory Systems
- Paged memory modules
- Dynamic capacity scaling
- Multiple memory banks (A-D)
- Hot/cold storage optimization

### Storage Features
- Compressed cold storage
- Proof-based retrieval
- Authority-signed transitions

## 3. Account Management

### Supported Account Types
- Timelock accounts
  - Configurable lock periods
  - Structured unlock process
- Nonce accounts
  - Durable transaction sequencing
  - State channel support

### Token Operations
- Non-custodial deposits
- Secure withdrawals from:
  - Hot memory
  - Cold storage
  - Deposit addresses

## 4. Execution Engine

### Core Operations
- Transfer execution
- Withdrawal processing
- Relay management
- Conditional operations
- External interactions

### State Management
- Relay root history
- Compressed state verification
- Authority validations

## 5. Security Architecture

### Core Security Features
- Non-custodial design
- Timelock enforcement
- Signed state transitions
- Merkle proof verification
- Memory bank isolation
- Account validation

### Technical Integration
- SPL Token compatibility
- System program interfaces
- PDA derivation paths
- Account validation framework

## 6. Architecture Components

- PDA-based account system
- Circular state buffer
- Multi-bank memory model
- Compressed storage
- Segregated token accounts
- Proof-based verification

## Supported Operations

### Internal Operations
- Transfer: Moves tokens between virtual accounts with owner signature
- Withdraw: Closes virtual account and moves tokens to another virtual account
- Relay: Processes private payments from relay to virtual accounts

### External Operations
- External Transfer: Moves tokens from virtual to external accounts
- External Withdraw: Closes virtual account and moves tokens to external account
- External Relay: Processes private payments from relay to external accounts
- Conditional Transfer: Links transfers to proof of prior relay operations

#### Common Features
- Nonce-based transaction ordering
- Signature verification for all operations
- Balance arithmetic checks
- Omnibus account management
- Relay commitment tracking
- Merkle root verification

# Program Instructions

## compress.rs
- Compresses virtual accounts from VM working memory into cold storage
- Signs and hashes account data before compression for verification
- Requires VM authority signature
- Deletes account from memory after compression

## decompress.rs
- Decompresses virtual accounts from cold storage back to working memory
- Verifies account signatures and state
- Handles special validation for timelocked accounts
- Checks withdrawal receipts and unlock states

## deposit.rs
- Processes token deposits into virtual accounts
- Transfers tokens from deposit ATA to VM omnibus account
- Updates virtual account balances
- Verifies depositor ownership

## exec.rs
- Executes VM opcodes on virtual accounts
- Handles multiple memory banks (A-D)
- Supports various operations: transfers, withdrawals, relays
- Manages both internal and external transactions

## init_vm.rs
- Initializes new VM instances
- Creates omnibus token accounts
- Sets lock duration parameters
- Establishes VM authority

## init_unlock.rs
- Initiates unlock process for timelocked accounts
- Creates unlock state accounts
- Calculates unlock timeframes
- Validates owner permissions

## init_storage.rs
- Creates new cold storage accounts for VM
- Initializes compressed state storage
- Sets up storage parameters
- Links storage to VM instance

## init_nonce.rs
- Creates virtual durable nonce accounts
- Similar to Solana's durable nonces
- Manages nonce state in VM memory
- Verifies ownership and permissions

## init_memory.rs
- Initializes new memory modules for VM
- Sets up paged memory structure
- Configures account capacity
- Links memory to VM instance

## init_timelock.rs
- Creates virtual timelock accounts
- Establishes state channels between VM and owners
- Enables instant token transfers
- Sets up timelock parameters

## resize.rs
- Resizes memory accounts
- Only allows size increases
- Validates new size parameters
- Maintains account integrity

## snapshot.rs
- Saves current relay root state
- Manages circular buffer for proofs
- Simple state preservation mechanism
- Requires VM authority

## withdraw.rs
- Handles token withdrawals from virtual accounts
- Supports memory, storage, and deposit withdrawals
- Creates withdrawal receipts
- Validates timelock states

*Note: Each instruction set maintains strict security checks, requires appropriate signatures, and updates VM state through POH advancement.*

# Program Operations

## conditional_transfer.rs
- Executes transfers contingent on prior relay operations
- Verifies virtual relay account as proof of prior commitment
- Requires source account owner signature
- Transfers tokens from VM omnibus to external accounts
- Updates nonce states and account balances

## external_relay.rs
- Processes private payments from relay to external accounts
- Verifies relay root history and commitment proofs
- Creates virtual relay accounts as operation receipts
- Transfers tokens directly from relay vault
- Updates relay merkle tree with new commitments

## external_transfer.rs
- Handles token transfers from virtual to external accounts
- Moves tokens from VM omnibus to destination
- Requires source account owner signature
- Updates virtual account balances
- Maintains nonce sequencing

## external_withdraw.rs
- Closes virtual accounts with external token transfers
- Requires owner signature for authorization
- Transfers full balance to external account
- Deletes source virtual account after transfer
- Updates nonce states

## relay.rs
- Processes private payments from relay to virtual accounts
- Verifies relay commitments and merkle proofs
- Creates virtual relay accounts
- Transfers tokens from relay vault to VM omnibus
- Updates relay merkle tree state

## transfer.rs
- Moves tokens between virtual accounts
- Requires source account owner signature
- Updates balances for source and destination
- Maintains nonce sequencing
- Performs balance arithmetic checks

## withdraw.rs
- Closes source virtual account with internal transfer
- Moves full balance to destination virtual account
- Requires owner signature authorization
- Deletes source account after transfer
- Updates nonce states

# Operation Authority Matrix

| Operation            | Authority | Account Owner | Payer |
|----------------------|:---------:|:-------------:|:-----:|
| compress             |     ✓     |               |       |
| decompress           |     ✓     |               |       | 
| deposit              |     ✓     |               |       |
| exec                 |     ✓     |               |       |
| init_vm              |     ✓     |               |       |
| init_unlock          |           |       ✓       |   ✓   |
| init_storage         |     ✓     |               |       |
| init_nonce           |     ✓     |               |       |
| init_memory          |     ✓     |               |       |
| init_timelock        |     ✓     |               |       |
| resize               |     ✓     |               |       |
| snapshot             |     ✓     |               |       |
| withdraw (virtual)   |           |       ✓       |       |
| withdraw (unlocked)  |           |       ✓       |   ✓   |
| conditional transfer |           |       ✓       |       |
| external relay       |           |               |       |
| external transfer    |           |       ✓       |       |
| external withdraw    |           |       ✓       |       |
| relay                |           |               |       |
| transfer             |           |       ✓       |       |

*Account Owner is the "Depositor"*

# Important Notes on Withdrawals

## Non-custodial unlocked withdrawals (instruction/withdraw.rs)
Allows withdrawing tokens from unlocked virtual accounts to external accounts
Requires the depositor's signature as a signer
Requires the payer's signature as a signer
Requires proof that the timelock is in unlocked state via unlock_pda

### Requirements
- Depositor signature (signer)
- Payer signature (signer)
- Unlocked timelock state via unlock_pda

### Withdrawal Sources
1. Memory (hot)
2. Storage (cold) 
3. Deposit accounts

## Virtual account withdrawals (opcode/withdraw.rs)
Handles transfers between virtual accounts within the VM
Requires signature from source account owner
Uses nonce accounts for transaction ordering
Deletes source account after withdrawal
Updates balances of destination virtual account

### Requirements
- Source account owner signature
- Nonce account for ordering
- 3 memory banks/indices

### Flow
1. Source balance deducted
2. Destination balance increased 
3. Source account deleted
4. Nonce updated

## Virtual to external withdrawals (opcode/external_withdraw.rs)
Handles transfers from virtual accounts to external token accounts
Requires signature from source account owner
Uses nonce accounts for transaction ordering
Deletes source account after withdrawal
Transfers tokens through VM's omnibus account

### Requirements
- Source account owner signature  
- Nonce account for ordering
- Token program account
- Omnibus account
- External destination account

### Flow
1. Verify signature against withdraw message
2. Transfer via omnibus account
3. Source account deleted
4. Nonce updated

## Withdrawals Summary

Three distinct withdrawal types with different purposes:
1. Non-custodial unlocked withdrawals (instruction)
2. Virtual account withdrawals (opcode)
3. Virtual to external withdrawals (opcode)

Key differences:
1. Signature Requirements
- Non-custodial: Direct signer accounts
- Virtual/External: Message signatures

2. Account Requirements
- Non-custodial: Unlock proof
- Virtual: VM state only  
- External: Token program + omnibus

3. Access Pattern
- Non-custodial: User-facing instruction
- Virtual/External: VM-internal opcodes

# Relay System
Relays facilitate private token transfers within the program ecosystem using a commitment-based privacy protocol.

## Components

### Relay Account
- Maintains recent root history
- Tracks commitments via Merkle tree
- Controls dedicated token vault
- Configurable depth and history size

### Relay Vault
- PDA-derived token account
- Holds tokens for private transfers
- Owned by relay account
- Facilitates secure token movements

## Operations

### Internal Relay (relay.rs)
- Processes private payments from relay to virtual accounts
- Moves tokens: relay vault → VM omnibus
- Verifies relay root history and commitments
- Creates virtual relay accounts as receipts
- Updates Merkle tree with new commitments

### External Relay (external_relay.rs) 
- Handles private payments from relay to external accounts
- Moves tokens: relay vault → external address
- Verifies relay root history and commitments
- Creates virtual relay accounts as receipts
- Updates Merkle tree with new commitments

### Relay Initialization (init_relay.rs)
- Creates new relay and treasury accounts
- Sets relay parameters (name, depth, history size)
- Initializes Merkle tree state
- Links relay to VM instance
- Creates dedicated token vault

## Security Features
- Commitment verification
- Recent root validation
- Merkle proof checking
- PDA-based account derivation
- Signed token transfers


# Entities and Sequences

## Core Components

## VM (VmAccount)
- Manages the virtual machine state
- Controls token minting authority
- Tracks proof-of-history (poh)
- Maintains omnibus pool for token custody

### Memory (MemoryAccount) 
- Stores hot state virtual accounts
- Provides fast access to active accounts
- Manages virtual account balances
- Handles immediate operations

### Storage (StorageAccount)
- Compressed cold storage using Merkle trees
- Stores inactive accounts efficiently
- Provides cryptographic proofs for account states
- Reduces on-chain storage costs

### Omnibus Pool
- Central token custody mechanism
- Holds tokens for virtual accounts
- Manages token transfers between virtual accounts
- Handles external withdrawals

### Relay Pool
- Facilitates private payments
- Maintains commitment history
- Uses Merkle trees for privacy
- Manages relay-specific token vaults

## Account Types

### Virtual Timelock Account
- Time-locked token storage
- Owner-controlled balances
- Supports transfers and withdrawals
- Requires unlock sequence for withdrawals

### Virtual Relay Account
- Proof of private payments
- Links to relay commitments
- Enables conditional transfers
- Maintains payment privacy

### Virtual Nonce Account
- Prevents replay attacks
- Tracks operation sequence
- Ensures transaction uniqueness
- Updates with VM poh

### Unlock State Account
- Manages timelock status
- Tracks unlock timing
- Controls withdrawal permissions
- Prevents unauthorized access

### Withdraw Receipt
- Records withdrawal history
- Prevents double withdrawals
- Validates withdrawal claims
- Maintains withdrawal integrity

## Sequences
::: mermaid
sequenceDiagram
    participant User as User/Authority
    participant Program as VM
    participant Memory as VM Memory
    participant Storage as VM Storage
    participant Omnibus as Omnibus Pool
    participant Relay as Relay Pool
    participant Token as Token Program

    User->>VM: Initialize VM Account
    VM-->>User: Return VM Address
    
    User->>Memory: Initialize Memory Account
    Memory-->>User: Return Memory Address

    rect rgb(236, 21, 87)
        Note over User,Token: Deposit Flow
        User->>Token: Transfer to Deposit PDA
        User->>VM: Process Deposit
        VM->>Memory: Read Virtual Account
        VM->>VM: Verify Owner
        VM->>Omnibus: Move Tokens (Deposit->Omnibus) 
        VM->>Memory: Update Virtual Balance
    end

    rect rgb(238, 63, 50)
        Note over User,Token: Hot State Operations
        User->>VM: Execute Operation
        alt Transfer
            VM->>Memory: Read Source & Destination
            VM->>VM: Verify Signature & Nonce
            VM->>Memory: Update Virtual Balances
        else Relay Private Payment
            VM->>Memory: Read Accounts
            VM->>VM: Verify Relay Commitment
            VM->>Relay: Check Recent Roots
            VM->>Omnibus: Move Tokens (Relay->Omnibus)
            VM->>Memory: Update Virtual Balances
        else Conditional Transfer
            VM->>Memory: Verify Relay Account
            VM->>Memory: Check Virtual Relay Account
            VM->>Omnibus: Process Transfer
            VM->>Memory: Update Balances
        end
    end

    rect rgb(242, 116, 31)
        Note over User,Token: Cold Storage Operations
        User->>VM: Compress Account
        VM->>Memory: Read Account
        VM->>VM: Authority Signs State
        VM->>Storage: Add to Merkle Tree
        VM->>Memory: Delete from Memory

        User->>VM: Decompress Account
        VM->>Storage: Verify Merkle Proof
        VM->>VM: Verify Authority Signature
        VM->>Memory: Restore to Memory
    end

    rect rgb(237, 172, 29)
        Note over User,Token: Withdrawal Flow
        User->>VM: Initiate Timelock Unlock
        VM->>VM: Create Unlock State
        Note over VM: Wait for Timelock

        User->>VM: Complete Unlock
        VM->>VM: Verify Timelock Expired
        VM->>VM: Update Unlock State

        alt External Withdraw
            User->>VM: Withdraw to External
            VM->>Memory: Clear Account
            VM->>Omnibus: Release Tokens
            VM->>Token: Transfer to External
        else Internal Withdraw
            User->>VM: Withdraw to Virtual
            VM->>Memory: Update Balances
            VM->>Memory: Clear Source Account
        end
    end
:::

### Hot State Operations:
Transfer: No actual token movement, just virtual balance updates
Relay Private Payment: Token movement from Relay Vault -> Omnibus via Token Program
Conditional Transfer: Token movement from Omnibus -> External Address via Token Program

### External Operations (Token Program Interactions):
External Transfer: Token movement from Omnibus -> External Address via Token Program
External Withdraw: Token movement from Omnibus -> External Address via Token Program
External Relay: Token movement from Relay Vault -> External Address via Token Program
Deposit Flow: Token movement from Deposit ATA -> Omnibus via Token Program
Withdrawal Flow: Token movement from Omnibus -> External Address via Token Program

These token movements are explicit in the codebase through the transfer_signed() calls and should be represented as distinct interactions with the Token Program in the sequence diagram.
