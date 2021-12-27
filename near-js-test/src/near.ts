import {ConnectConfig} from "near-api-js";

const {KeyPair, keyStores, connect, WalletConnection, Contract} = require("near-api-js");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";

const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

export const getConfig = (): ConnectConfig => {
  return {
    keyStore,
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
    headers: {},
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
  };
}
