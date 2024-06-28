async function main() {
  const [deployer] = await ethers.getSigners();

  console.log("Deploying contracts with the account:", deployer.address);

  const SimpleStorage = await ethers.getContractFactory("SimpleStorage");
  const simpleStorage = await SimpleStorage.deploy();
  
  console.log("SimpleStorage contract deployed to:", await simpleStorage.getAddress());
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error("Error deploying contracts:", error);
    process.exit(1);
  });
