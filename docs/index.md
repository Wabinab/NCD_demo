# NEAR Blog

An attempt to build a NEAR Blog like read.cash or medium that allows people to sign in with their wallet, write a blog, and others to tip the owner. The (MVP) completed code is in `tipping` folder. 
It's an MVP because there are things that can be improved, like royalty is not developed yet for this contract. 

## Frontend
Well, the frontend is bad. Given my inability to write properly in React, it'll be bad. In the future, one will try to port to Ruby on Rails to code (because it's more Pythonic!) the frontend, then re-link the backend from server and database to on-chain and blockchain network. 
Of course, given it follows 'convention over configuration', it won't be easy, nor one thought it would be easy to redirect "Active Storage" (aka database) to IPFS. 

## Backend
The first contract one tried is the signup contract, but it failed badly. Lessons are learnt, with security issues, Gas issues, farmers issues, etc. 

The second contract is the tipping contract, a way to tip others. It's a simple contract. In fact, **it's deliberately made as simple as possible** so readers could easily understand what is being written. 
The more you write, the more error you have, and the more (security) loopholes you're going to create with more permutations. Keeping it as simple as possible means less gas fee to pay, and easier understanding of how things work. 

For more info, read the README. If one have time, one might update the documentation to be more descriptive in the future. 

Thanks for reading. 

**Wabinab**
