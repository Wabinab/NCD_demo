# NCD_demo
LNC NCD demonstration

We took inspiration from existing blogging sites like read.cash, publish0x, medium, etc. Whatever looks clean, we use it. 

## IMPORTANT
The main contract is in the `tipping` folder. The contract inside `main_project/contract` is abandoned, and we had changed `config.js` inside `main_project` to reflect that changes. So, we need to deploy first (separately), then it's optional to 
deploy the abandoned contract (one uses it as a normal account to try tip to my deployed contract). 

## SECURITY ISSUES + THINGS NOTICED
Of course, there are some security issues with the contract. These aren't shown in the demo due to time constraints. Particularly: 
- If one creates an article, but one deletes the wallet that one uses to write the article, tip can't go through to that wallet. Hence, with the safe version, tip will be returned to tipper. With `unsafe` version, will redirect money to the account hosting the contract. 
- `unsafe` function is **much much cheaper** than `safe` function, **if you're only tipping 0.001 NEAR**. Particularly, the cost of a safe tipping is 0.0018 N (more than you tip, cost 18 TGas), while unsafe is 0.0007N (7 TGas). 
- 

## UPDATE FAILURE
The linkdrop failed. One wrote two articles on it: https://read.cash/@wabinab/sign-up-smart-contract-partial-solution-and-its-security-concerns-2b651931 and 
https://read.cash/@wabinab/sign-up-smart-contract-continuation-2-createaccountnotallowed-1451815b which you could check out why. 

One will move on to another tipping module. 


