require('dotenv').config();
const { Deployer } = require("@matterlabs/hardhat-zksync");
const { Provider, Wallet, types } = require("zksync-ethers");

const func = async function (hre) {
    const verifierContractName = "Groth16Verifier"; // Actualizar el nombre del contrato

    // Definir proveedores de red para zkSync y Ethereum Sepolia
    const provider = Provider.getDefaultProvider(types.Network.Sepolia);
    const ethProvider = hre.ethers.getDefaultProvider("sepolia");

    // Obtener la clave privada de la variable de entorno
    const PRIVATE_KEY = process.env.ZKSYNC_SEPOLIA_PRIVATE_KEY !== undefined ? process.env.ZKSYNC_SEPOLIA_PRIVATE_KEY : "";

    // Inicializar la wallet de zkSync
    const zkWallet = new Wallet(PRIVATE_KEY, provider, ethProvider);

    // Crear una instancia de deployer
    const deployer = new Deployer(hre, zkWallet);

    // Cargar el artefacto del contrato Groth16Verifier
    const groth16VerifierArtifact = await deployer.loadArtifact(verifierContractName);

    // Desplegar el contrato Groth16Verifier
    const groth16VerifierContract = await deployer.deploy(groth16VerifierArtifact);

    // Esperar a que el despliegue del contrato termine
    await groth16VerifierContract.waitForDeployment();

    // Obtener la dirección del contrato desplegado
    const contractAddress = await groth16VerifierContract.getAddress();
    console.log(verifierContractName + " desplegado en:", contractAddress);

    // Guardar los detalles del despliegue
    const deployments = await hre.deployments;
    await deployments.save(verifierContractName, {
        address: contractAddress,
        abi: groth16VerifierArtifact.abi,
    });

    return true;
};

module.exports = func;
func.id = "DeployGroth16VerifierZKSync";
func.tags = ["DeployGroth16VerifierZKSync"];
