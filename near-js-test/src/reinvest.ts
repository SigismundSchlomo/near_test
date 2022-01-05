import {Account} from "near-api-js";
import {claimReward, listRewards, withdrawReward} from "./ref-farming-utils";


export const reinvest = async (account: Account, poolId: number) => {
  const rewardResult = await claimReward(account, poolId);
  const rewards = await listRewards(account);
  const rewardsPromises = Object.keys(rewards).map((tokenId, amount) => withdrawReward(account, tokenId, amount.toString()))
  const results = await Promise.all(rewardsPromises);


}
