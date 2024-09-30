require('dotenv').config();
require("@nomicfoundation/hardhat-toolbox");
require("hardhat-deploy");
require("@matterlabs/hardhat-zksync-node");
require("@matterlabs/hardhat-zksync");
require("@nomicfoundation/hardhat-chai-matchers");

require("./tasks/verify-verifier-contracts-zksync");

module.exports = {
  zksolc: {
    version: "latest",
    compilerSource: "binary",
    settings: {},
  },
  typechain: {
    target: "ethers-v6",
  },
  mocha: {
    timeout: 10000000000,
  },
  solidity: {
    version: "0.8.16",
    settings: {
      optimizer: {
        enabled: true,
        runs: 9999,
      },
    },
  },
  defaultNetwork: "hardhat",
  networks: {
    zkSyncSepoliaTestnet: {
      url: "https://sepolia.era.zksync.dev",
      ethNetwork: "sepolia",
      zksync: true,
    },
    hardhat: {
      chainId: 1337,
      zksync: false,
    },
    localhost: {
      url: "http://127.0.0.1:8545",
      chainId: 31337,
    },
    albireo: {
      url: "https://albireo-rpc.playfi.ai",
      ethNetwork: "albireo",
      zksync: true,
      verifyURL: "https://albireo-explorer.playfi.ai/contract_verification",
    },
    zkSyncMainnet: {
      url: "https://mainnet.era.zksync.io",
      ethNetwork: "mainnet",
      zksync: true,
      verifyURL: "https://zksync2-mainnet-explorer.zksync.io/contract_verification",
    },
    zkSyncGoerliTestnet: { // deprecated network
      url: "https://testnet.era.zksync.dev",
      ethNetwork: "goerli",
      zksync: true,
      verifyURL: "https://zksync2-testnet-explorer.zksync.dev/contract_verification",
    },
    dockerizedNode: {
      url: "http://localhost:3050",
      ethNetwork: "http://localhost:8545",
      zksync: true,
    },
    inMemoryNode: {
      url: "http://127.0.0.1:8011",
      ethNetwork: "localhost", // in-memory node doesn't support eth node; removing this line will cause an error
      zksync: true,
    },
    hardhat: {
      zksync: true,
    },
  },
  gasReporter: {
    enabled: process.env.REPORT_GAS !== undefined,
    currency: "USD",
  },
  namedAccounts: {
    deployer: {
      default: 0,
      42161: 0, //TODO: set correct address
      324: 0, //TODO: set correct address
    },
    admin: {
      default: 2,
      42161: 2, //TODO: set correct address
      324: 2, //TODO: set correct address
      421614: "0x76D4e57584Bc60A965CE98830F3567d4A23d3BDB",
      80002: "0x76D4e57584Bc60A965CE98830F3567d4A23d3BDB",
      300: "0x76D4e57584Bc60A965CE98830F3567d4A23d3BDB",
      1612127: "0x76D4e57584Bc60A965CE98830F3567d4A23d3BDB"
    },
    fingerPrintProxy: {
      default: 2,
    },
  },
};