# Developer Guide

This guide provides essential information for building, deploying and working with the Relay Pool ("RePool") programme. This is a community fork of the CODE VM (https://github.com/code-payments/code-vm).

Refer to the technical documentation for detailed specifications of each component.

## Environment Setup

1. Install Rust and Solana CLI tools:

```bash:setup_rust.sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash:setup_solana.sh
sh -c "$(curl -sSfL https://release.solana.com/v1.18.9/install)"
```

2. Install build dependencies:

```bash:install_deps.sh
sudo apt-get update && sudo apt-get install -y pkg-config build-essential libudev-dev
```

3. Install Node.js dependencies for TypeScript development:

```bash:install_node_deps.sh
npm install vitest @coral-xyz/anchor @solana/web3.js @solana/spl-token
```

```bash:install_dev_deps.sh
npm install --save-dev @types/node
```

4. Configure TypeScript:

```json:tsconfig.json
{
  "compilerOptions": {
    "types": ["mocha", "chai", "node", "vitest"],
    "typeRoots": ["./node_modules/@types"],
    "lib": ["es2015", "dom"],
    "module": "NodeNext", 
    "target": "es6",
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "resolvePackageJsonImports": true,
    "moduleResolution": "NodeNext",
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  }
}
```

## Building the Programme

Clone and build:

```bash:build.sh
cargo build-sbf
```

Run tests:

```bash:test.sh
cargo test-sbf -- --nocapture
```

## Devnet Deployment Guide

1. Generate Programme Keypair:

```bash:gen_keypair.sh
solana-keygen new -o target/deploy/code_vm-keypair.json
```

2. Get Programme ID:

```bash:get_program_id.sh
solana address -k target/deploy/code_vm-keypair.json
```

3. Switch to Devnet:

```bash:set_devnet.sh
solana config set --url devnet
```

4. Fund Your Wallet:

```bash:fund_wallet.sh
solana airdrop 4 --url devnet
```

5. Deploy Programme:

```bash:deploy.sh
solana program deploy target/deploy/code_vm_program.so
```

6. Verify Deployment:
- View your programme ID at https://solscan.io/?cluster=devnet
- Get programme data:

```bash:verify_deploy.sh
solana program show --url devnet PROGRAM_ID
```

## Testing Guide

Basic Timelock Creation and Deposit Test:

```bash:test_deposit.sh
cargo test run_withdraw_from_deposit_pda
```

Memory-based Operations Test:

```bash:test_memory.sh
cargo test run_withdraw_from_memory
```

Storage-based Operations Test:

```bash:test_storage.sh
cargo test run_withdraw_from_storage
```


## Development Workflow

### Initialize new VM instance:
- Create VM account
- Setup memory banks
- Configure token mint
- Initialize omnibus pool

### Test core operations:
- Virtual account creation
- Token deposits/withdrawals
- Memory compression/decompression
- Relay operations

### Verify security:
- Authority checks
- Signature verification
- Timelock enforcement
- State transitions

## Integration Testing

### Local environment:
- Use solana-test-validator
- Deploy test token mint
- Create test accounts
- Execute operation sequences

### Devnet testing:
- Deploy to devnet
- Test with devnet tokens
- Verify gas costs
- Measure performance

### Program verification:
- Account validation
- State consistency
- Error handling
- Edge cases

## Monitoring & Maintenance

### Setup monitoring:
- Account state tracking
- Operation logging
- Error reporting
- Performance metrics

### Regular maintenance:
- State cleanup
- Memory optimization
- Security updates
- Performance tuning

## Additional Resources
- Program IDL: `/idl/code_vm.json`
- State indexer: GitHub repo link
- Technical docs: `/docs`
- Test suite: `/tests`

## Common Operations

### Memory management:
```rust
// Initialize new memory bank
init_memory_bank(bank_index: u8, capacity: u32)

// Compress inactive accounts
compress_accounts(accounts: Vec<Pubkey>)

// Decompress for active use
decompress_accounts(proofs: Vec<MerkleProof>)
```

### Account operations:
```rust
// Create virtual account
init_virtual_account(owner: Pubkey, timelock: i64)

// Process deposit
process_deposit(amount: u64, owner: Pubkey)

// Execute transfer
execute_transfer(from: Pubkey, to: Pubkey, amount: u64)
```

### State management:
```rust
// Update relay root
update_relay_root(new_root: [u8; 32])

// Verify account state
verify_account_state(account: Pubkey, proof: MerkleProof)

// Process state transition
transition_state(from_state: State, to_state: State)
```


# RePool Sequence Diagrams

## VM Initialisation Sequence
::: mermaid
sequenceDiagram
    participant Client
    participant Program
    participant VM State
    participant Memory Manager
    participant Account Registry
    participant Token Program

    rect rgb(236, 21, 87)
    Note over Program: VM Bootstrap Phase
        Client->>Program: InitVM(authority, mint, lock_duration)
        Program->>VM State: Create VM State PDA
        Program->>Token Program: Create Omnibus Account
        VM State->>VM State: Initialize POH Counter
        VM State->>VM State: Register Authority
    end

    rect rgb(242, 116, 31)
    Note over Program: Memory Management Layer
        Program->>Memory Manager: InitMemory(name, size)
        Program->>Memory Manager: InitStorage(name)
        Memory Manager->>Memory Manager: Setup Page Tables
        Memory Manager->>Memory Manager: Initialize Merkle Trees
    end

    rect rgb(181, 197, 50)
    Note over Program: Account System Layer
        Program->>Account Registry: InitRelay(name)
        Program->>Account Registry: InitTimelock(owner)
        Program->>Account Registry: InitNonce(owner)
        Account Registry->>Account Registry: Register PDAs
        Account Registry->>Token Program: Setup Token Vaults
    end

    rect rgb(3, 170, 164)
    Note over Program: State Control Layer
        VM State-->>Memory Manager: Memory Access Control
        VM State-->>Account Registry: Account Permissions
        VM State-->>VM State: Authority Verification
        VM State-->>VM State: POH Advancement
    end
:::

### The diagram above demonstrates:
- All PDAs are deterministically derived
- Hierarchical permission structure
- State transitions tracked via POH
- Clear parent-child account relationships

This architecture enables secure, deterministic account creation while maintaining clear ownership and authority relationships throughout the system.

#### VM Bootstrap Phase
- Authority key management
- POH counter initialization
- Omnibus account setup for token management
- Core VM state initialization

#### Security through:
- Consistent PDA derivation patterns
- Authority validation at each level
- Proper account initialization checks
- Layered state management

#### Memory Management Layer
- Paged virtual memory system
- Cold storage compression
- Merkle tree state management
- Memory access controls

#### Account System Layer
- Virtual account types (Nonce, Timelock, Relay)
- PDA derivation and management
- Token vault associations
- Account state transitions

#### State Control Layer
- Authority delegation system
- POH-based sequencing
- Permission management
- Cross-component coordination


## Setup and Initialization Flow

::: mermaid
sequenceDiagram
    participant Client
    participant SVM
    participant Program
    participant TokenProgram

    rect rgb(236, 21, 87)
        Client->>SVM: setup_svm()
        Note over SVM: Loads program bytes
        SVM-->>Client: LiteSVM instance

        Client->>SVM: create_payer()
        Note over SVM: Creates new Keypair
        SVM->>SVM: airdrop to payer
        SVM-->>Client: payer_keypair
    end

    rect rgb(238, 63, 50)
        Client->>SVM: create_mint(payer, owner)
        SVM->>TokenProgram: CreateMint instruction
        TokenProgram-->>Client: mint_pubkey

        Client->>SVM: create_ata(payer, mint, owner)
        SVM->>TokenProgram: CreateAssociatedTokenAccount
        TokenProgram-->>Client: ata_pubkey
    end

    rect rgb(242, 116, 31)
        Client->>SVM: mint_to(payer, mint, owner, dest, amount)
        Note over SVM: Requires mint_owner signature
        SVM->>TokenProgram: MintTo instruction
    end
:::
*This flow demonstrates the complete key lifecycle from program initialization through token operations, with each step precisely mapped to implementation files.*

### Initial Setup
**program_bytes() in svm.rs:**
- Loads program binary from ../target/deploy/code_vm_program.so
- Returns program bytes for VM initialization

**SVM Initialization (setup_svm() in svm.rs):**
- Creates new LiteSVM instance
- Adds program using code_vm_api::ID
- Returns configured SVM instance

### Account Setup
**Payer Creation (create_payer() in svm.rs):**
- Generates new Keypair
- Airdrops 64_000_000_000 lamports
- Returns payer_keypair for transaction signing

**Mint Creation (create_mint() in svm.rs):**
- Takes payer_kp and owner_pk parameters
- Uses CreateMint::new() to initialize token mint
- Sets mint authority to owner_pk
- Returns mint pubkey

### Token Operations
**Associated Token Account Creation (create_ata() in svm.rs):**
- Takes payer_kp, mint_pk, owner_pk parameters
- Creates ATA using CreateAssociatedTokenAccount
- Links ATA to specified owner
- Returns ATA pubkey

**Token Minting (mint_to() in svm.rs):**
- Requires payer, mint, mint_owner, destination, amount
- Validates mint_owner signature
- Executes MintTo instruction
- Returns Result<(), FailedTransactionMetadata>

### Transaction Processing
**Transaction Handling (send_tx() in svm.rs):**
- Processes transaction through SVM
- Prints detailed transaction metadata
- Includes signature verification
- Logs compute units and execution details

## Relay & Merkle Tree Operations

::: mermaid
sequenceDiagram
    participant RelayAccount
    participant MerkleTree
    participant TokenPool
    participant Hash

    rect rgb(181, 197, 50)
        RelayAccount->>MerkleTree: Initialize Tree
        RelayAccount->>TokenPool: Setup Treasury
    end
    
    rect rgb(98, 191, 70)
        loop For Each Transaction
            RelayAccount->>MerkleTree: Add Commitment
            MerkleTree->>Hash: Calculate Root
            RelayAccount->>RelayAccount: Save Recent Root
            RelayAccount->>TokenPool: Update Balance
        end
    end
:::


### Relay Account Setup (relay.rs)
- Initializes with VM reference
- Sets up treasury pool
- Configures Merkle tree parameters

### Transaction Processing
- Merkle tree management (relay.rs)
- Root calculation and history tracking
- Commitment storage
- Uses RELAY_STATE_DEPTH for tree depth
- Maintains RELAY_HISTORY_ITEMS recent roots

### Token Pool Operations (pool.rs)
- Manages vault addresses
- Handles token balances
- Uses splitter program for relay destinations

### Security
- Ed25519 signatures (signature.rs)
- SHA256/SHA512 hashing (hash.rs)
- Program derived addresses (pdas.rs)
- Operation codes defined in opcode.rs

## Account Management & Operations Flow

::: mermaid
sequenceDiagram
    participant User
    participant VirtualAccount
    participant TimelockAccount
    participant RelayAccount
    participant NonceAccount

    rect rgb(43, 159, 196)
        User->>VirtualAccount: Create Virtual Account
        VirtualAccount->>+TimelockAccount: Initialize Timelock
        TimelockAccount->>-VirtualAccount: Return Timelock Info
    end
    
    rect rgb(92, 118, 188)
        Note over User,RelayAccount: Transfer Flow
        User->>VirtualAccount: Request Transfer
        VirtualAccount->>NonceAccount: Get Nonce
        VirtualAccount->>TimelockAccount: Create Transfer Message
        TimelockAccount->>RelayAccount: Execute Transfer
    end

    rect rgb(137, 40, 137)
        Note over User,RelayAccount: Withdraw Flow
        User->>VirtualAccount: Request Withdraw
        VirtualAccount->>NonceAccount: Get Nonce
        VirtualAccount->>TimelockAccount: Create Withdraw Message
        TimelockAccount->>RelayAccount: Process Withdrawal
    end
:::

### Virtual Account Creation (virtual_account.rs)
- Handles account type selection (Nonce/Timelock/Relay)
- Manages account data packing/unpacking
- Size specifications:
  - Nonce: 64 bytes
  - Timelock: 76 bytes

### Timelock Operations (timelock.rs)
- Creates timelock addresses using PDAs
- Manages token vaults
- Handles unlock/withdraw receipt generation

### Transfer Operations (transfer.rs)
- Creates transfer messages with:
  - Source timelock address
  - Destination timelock address
  - Amount
  - Virtual durable nonce
- Supports both internal and external transfers

### Withdraw Operations (withdraw.rs)
- Generates withdraw messages
- Handles both internal and external withdrawals
- Uses virtual durable nonce for transaction uniqueness