pragma solidity ^0.4.23;

import "openzeppelin-solidity/contracts/math/SafeMath.sol";
import "./PoolToken.sol";

contract Pool {
    using SafeMath for uint;

    uint constant private MILLION = 1000000;
    uint constant private FIXEDPOINT = 1 ether;

    PoolToken public token;
    uint public totalStake;

    // todo use Ownable constructor?
    constructor() public {
        token = new PoolToken();
    }

    function stake() public payable {
        uint tokenPrice = getTokenPrice();
        uint tokens = msg.value.mul(FIXEDPOINT).div(tokenPrice); // msg.value / tokenPrice

        // todo return (msg.value % tokenPrice) to user ?
        token.mint(msg.sender, tokens);
        totalStake = totalStake.add(msg.value);
    }

    function unstake(uint tokens) public {
        require(tokens <= token.balanceOf(msg.sender), "Sender has not enough tokens");
        uint tokenPrice = getTokenPrice();
        uint deposit = tokenPrice.mul(tokens).div(FIXEDPOINT); // tokenPrice * tokens
        require(deposit <= totalStake, "Total stake is less than deposit");

        token.burn(msg.sender, tokens);
        totalStake = totalStake.sub(deposit);
        msg.sender.transfer(deposit);
    }

    function getTokenPrice() public view returns (uint) {
        uint total = token.totalSupply();
        if (total > 0) {
            return totalStake.mul(FIXEDPOINT).div(total);
        }
        return 1 ether;
    }
}
