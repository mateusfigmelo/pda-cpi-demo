# PDA as Signer CPI Demo

This is the **minimal, best-practice demo** for using Program Derived Addresses (PDAs) as signers for Cross-Program Invocations (CPIs) in Anchor.

## Programs

### 1. Callee Program (`callee`)
Receives CPI calls and requires a PDA to sign the transaction.

- **`initialize`**: Creates a PDA data account with counter = 0
- **`increment`**: Increments the counter (requires PDA as signer)

### 2. Caller Program (`caller`)
Makes CPI calls to the callee program using PDA as signer.

- **`call_increment`**: Does CPI to callee's `increment` using PDA as signer

## Key Concept: PDA as Signer

The core pattern demonstrated:

1. **Callee expects PDA as signer**:
   ```rust
   #[account(seeds = [b"data"], bump)]
   pub pda_signer: SystemAccount<'info>,
   ```

2. **Caller provides PDA signature**:
   ```rust
   let seeds = &[b"data".as_ref(), &[ctx.bumps.pda_signer]];
   let signer = &[&seeds[..]];
   let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
   ```

## Project Structure

```
pda-cpi-demo/
├── programs/
│   ├── caller/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── callee/
│       ├── Cargo.toml
│       └── src/lib.rs
├── tests/
│   └── pda-cpi-demo.ts
├── Anchor.toml
└── Cargo.toml
```

## Building and Testing

```bash
# Build the programs
anchor build

# Run the tests
anchor test
```

## Test Flow

1. **Initialize**: Create PDA data account with counter = 0
2. **First CPI**: Call `increment` via CPI using PDA as signer → counter = 1
3. **Second CPI**: Call `increment` again → counter = 2

## Why This Pattern?

- **Security**: Only the program can sign with its PDAs
- **No Private Keys**: PDAs don't have private keys
- **Deterministic**: Same seeds always produce the same PDA
- **Program Control**: Complete control over when PDAs can sign

This is the **canonical example** for PDA-as-signer CPI in Anchor. 
