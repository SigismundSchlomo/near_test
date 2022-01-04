# Some notes

## Reinvest
Reinvest should be made as a function in js library. Should take pool id and user's account object / address
Required steps:
1. Claim reward from pool. 
2. Withdraw reward from pool
3. Swap tokens if necessary 
4. Add liquidity to pool
5. Add shares to farming

This all involves interaction with several other smart contracts, so it should be done on client side.