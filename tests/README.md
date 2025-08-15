# Capstone Project Test Suite

This directory contains comprehensive tests for the Solana program that implements a DeFi vault with spending controls, yield farming, and merchant payment processing.

## Test Structure

The test suite is organized into the following test groups:

### 1. Vault Program Tests
- **Vault Initialization**: Tests proper vault account creation with correct authority and bump
- **Deposit Functionality**: Tests SOL deposits to vault with balance verification
- **Withdrawal Functionality**: Tests SOL withdrawals with proper balance updates
- **Insufficient Balance Handling**: Tests withdrawal failures when balance is insufficient
- **Authorization Controls**: Tests that only vault owners can access their vaults

### 2. Spending Account Tests
- **Account Initialization**: Tests spending account creation with proper defaults
- **Spending Limit Updates**: Tests automatic spending limit calculation (50% of vault balance)
- **Spend Authorization**: Tests successful spend authorization within limits
- **Limit Enforcement**: Tests failure when attempting to spend beyond limits
- **Spend Tracker Reset**: Tests the ability to reset spending counters

### 3. Treasury and Yield Tests
- **Treasury Initialization**: Tests global treasury account creation
- **Yield Account Setup**: Tests individual yield account initialization
- **Staking Functionality**: Tests SOL staking to treasury with proper balance transfers
- **Unstaking Functionality**: Tests partial unstaking with balance verification

### 4. Merchant and Payment Tests
- **Merchant Registration**: Tests merchant account creation with name storage
- **Payment Processing**: Tests successful SOL transfers from users to merchants
- **Spending Limit Validation**: Tests payment failures when exceeding spending limits

### 5. Integration Tests
- **Complete User Flow**: Tests the entire user journey from account setup to payment
- **Multi-User Independence**: Tests that multiple users can operate independently

### 6. Error Handling Tests
- **Account Not Found**: Tests proper error handling for non-existent accounts
- **Invalid PDA Seeds**: Tests constraint validation for PDA derivation
- **Missing Signers**: Tests signature verification requirements

### 7. Edge Cases Tests
- **Zero Amounts**: Tests handling of zero-value operations
- **Empty Account States**: Tests behavior with minimal account balances

## Running the Tests

### Prerequisites
- Node.js and Yarn installed
- Solana CLI tools installed
- Anchor framework installed
- Local Solana validator running

### Setup
1. Install dependencies:
   ```bash
   yarn install
   ```

2. Build the program:
   ```bash
   anchor build
   ```

3. Start a local validator (if not already running):
   ```bash
   solana-test-validator
   ```

### Running Tests
```bash
# Run all tests
anchor test

# Run tests with verbose output
anchor test -- --verbose

# Run specific test file
anchor test tests/capstone_project.ts

# Run tests with specific pattern
anchor test -- --grep "Vault Program Tests"
```

### Test Configuration
The tests are configured in `Anchor.toml` with:
- Test timeout: 1,000,000ms (16+ minutes)
- Test runner: ts-mocha
- TypeScript configuration from `tsconfig.json`

## Test Data
- **Test Amounts**: 
  - Deposit: 2 SOL
  - Spend: 0.5 SOL  
  - Stake: 1 SOL
- **Account Funding**: Each test account receives 5 SOL via airdrop
- **Fresh Keypairs**: New keypairs are generated for each test to ensure isolation

## Expected Test Results
- **Vault Tests**: All should pass, testing core vault functionality
- **Spending Tests**: All should pass, testing spending controls
- **Yield Tests**: All should pass, testing staking mechanisms
- **Payment Tests**: All should pass, testing merchant interactions
- **Integration Tests**: All should pass, testing end-to-end workflows
- **Error Tests**: All should pass, testing proper error handling

## Troubleshooting

### Common Issues
1. **Airdrop Failures**: Ensure local validator has sufficient SOL
2. **Account Creation Failures**: Check program ID matches in Anchor.toml
3. **PDA Derivation Errors**: Verify seed strings match program implementation
4. **Timeout Errors**: Increase timeout in Anchor.toml if needed

### Debug Mode
Run tests with debug logging:
```bash
anchor test -- --verbose --timeout 1000000
```

## Test Coverage
The test suite covers:
- ✅ All program instructions
- ✅ Account state management
- ✅ Error conditions and constraints
- ✅ Cross-instruction interactions
- ✅ Edge cases and boundary conditions
- ✅ Multi-user scenarios
- ✅ Integration workflows

This comprehensive test suite ensures the reliability and correctness of the DeFi vault program across all its functionality.
