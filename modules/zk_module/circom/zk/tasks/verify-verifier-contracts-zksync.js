require('dotenv').config();
const { task } = require("hardhat/config");
const { VERIFYVERIFIERCONTRACTSZKSYNC } = require("./task-names");
const { Provider, types } = require("zksync-ethers");

task(VERIFYVERIFIERCONTRACTSZKSYNC, "Verifies the Verifier contract", async (_taskArgs, hre) => {
  const { deployments } = hre;

  // Cargar los detalles del despliegue
  let deployment;
  try {
    deployment = await deployments.get("Groth16Verifier");
  } catch (error) {
    console.error("Error al obtener los detalles del despliegue para Verifier:", error);
    throw new Error("No se encontró un despliegue para: Verifier");
  }

  if (!deployment || !deployment.address) {
    throw new Error("La dirección del contrato Verifier no está definida. Asegúrate de que esté desplegado correctamente.");
  }

  const verifierContractAddress = deployment.address;

  const provider = Provider.getDefaultProvider(types.Network.Sepolia);

  try {
    await hre.run("verify:verify", {
      address: verifierContractAddress,
      constructorArguments: [], // Si tu contrato tiene argumentos en el constructor, agrégalos aquí
    });
  } catch (e) {
    if (e.name === "NomicLabsHardhatPluginError" && e.message.includes("Contract source code already verified")) {
      console.log("¡El código fuente del contrato ya está verificado!");
    } else {
      console.log(e);
    }
  }
});
