require("@nomicfoundation/hardhat-toolbox");


// const INFURA_API_KEY = "904a9154641d44348e7fab88570219e9";
const PRIVATE_KEY = "2be5f421885fb4b02b6421c86d92a2f7ae1d28cea43ec4c3a4d956670c3d1a9f"; // 0x17e66991ac9be7599ec66c08e3b2d63254458549


/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.0",
    settings: {
      optimizer: {
        enabled: true,
        runs: 5000,
      },
      viaIR: true,
    },
  },
  networks: {
    l1xTestnet: {
      url: "https://v2-testnet-rpc.l1x.foundation",
      accounts: [PRIVATE_KEY], 
    },
  }
};
