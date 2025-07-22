# PDA as Signer CPI Demo

This is the **ultra-simple, minimal demo** for using Program Derived Addresses (PDAs) as signers for Cross-Program Invocations (CPIs) in Anchor.

## Programs

### 1. Callee Program (`callee`)
Receives CPI calls and just prints a message.

- **`hello`**: Prints a message, requires any signer

### 2. Caller Program (`caller`)
Makes CPI calls to the callee program using its own PDA as signer.

- **`call_hello`**: Uses caller's PDA to sign CPI to callee's `hello`

## Key Concept: PDA as Signer

The core pattern demonstrated:

1. **Caller has a static PDA**:
   ```rust
   #[account(seeds = [b"caller_pda"], bump)]
   pub caller_pda: UncheckedAccount<'info>,
   ```

2. **Caller uses PDA to sign CPI**:
   ```rust
   let seeds = &[b"caller_pda".as_ref(), &[ctx.bumps.caller_pda]];
   let signer = &[&seeds[..]];
   let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
   ```

3. **Callee just requires any signer**:
   ```rust
   pub signer: Signer<'info>,
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

1. **Derive PDA**: Get the caller's static PDA
2. **CPI Call**: Call `call_hello` which uses the PDA to sign CPI to callee
3. **Success**: Callee prints "Hello from callee! PDA signed this CPI"

## Why This Pattern?

- **Ultra-Simple**: No authority, no data accounts, just PDA signing CPI
- **Security**: Only the program can sign with its PDAs
- **No Private Keys**: PDAs don't have private keys
- **Deterministic**: Same seeds always produce the same PDA

This is the **absolute simplest possible demo** for PDA-as-signer CPI in Anchor. 
