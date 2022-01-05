import {Account, transactions} from "near-api-js";
import {claimReward, listRewards, withdrawReward} from "./ref-farming-utils";


//TODO: Function to get tokens from pool id
export const reinvest = async (account: Account, poolId: number) => {
  const rewardResult = await claimReward(account, poolId);
  const rewards = await listRewards(account);
  //TODO: Wrong implementation. I should use near transactions
  const rewardsPromises = Object.keys(rewards).map((tokenId, amount) => withdrawReward(account, tokenId, amount.toString()))
  const results = await Promise.all(rewardsPromises);


}
