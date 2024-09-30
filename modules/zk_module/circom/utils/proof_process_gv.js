const path = require('path');
const snarkjs = require('snarkjs');
const hre = require('hardhat');
const ethers = hre.ethers;

// Get the absolute path to the current directory
const baseDir = __dirname;

// Ajuste de tamaño de arrays y funciones de conversión
function adjustArraySize(arr, targetLength) {
    if (arr.length > targetLength) {
        console.error(`Error: Array exceeds expected length. Got ${arr.length}, expected ${targetLength}.`);
        return arr.slice(0, targetLength);
    } else if (arr.length < targetLength) {
        while (arr.length < targetLength) {
            arr.push(0);
        }
        return arr;
    }
    return arr;
}

function stringToAsciiArray(str, targetLength) {
    if (typeof str !== 'string') {
        throw new Error(`Expected a string but got ${typeof str}. Value: ${str}`);
    }
    const asciiArray = str.split('').map(char => char.charCodeAt(0));
    return adjustArraySize(asciiArray, targetLength);
}

function hexStringToByteArray(hexStr, targetLength) {
    if (hexStr.startsWith('0x')) {
        hexStr = hexStr.slice(2);
    }
    const byteArray = [];
    for (let i = 0; i < hexStr.length; i += 2) {
        byteArray.push(parseInt(hexStr.substr(i, 2), 16));
    }
    return adjustArraySize(byteArray, targetLength);
}

function dateToUnixBytes(dateStr, targetLength) {
    const unixTime = Math.floor(new Date(dateStr).getTime() / 1000);
    const byteArray = [];
    let temp = unixTime;
    while (temp > 0) {
        byteArray.push(temp & 0xff);
        temp = temp >> 8;
    }
    return adjustArraySize(byteArray.reverse(), targetLength);
}

// Load input data and preprocess
const input = require(path.join(baseDir, '../../utils/src/json_object/json1.json'));

try {
    input.user = stringToAsciiArray(input.user, 8);
    input.game = stringToAsciiArray(input.game, 16);
    input.character = stringToAsciiArray(input.character, 12);
    input.ability = stringToAsciiArray(input.ability, 10);
    input.place = stringToAsciiArray(input.place, 10);
    input.place2 = stringToAsciiArray(input.place2, 10);
    input.uploader = hexStringToByteArray(input.uploader, 20);
    input.hash_inputdata = adjustArraySize(input.hash_inputdata, 32);
    input.timestamp = dateToUnixBytes(input.timestamp, 8);
} catch (error) {
    console.error("Error processing input fields:", error);
    process.exit(1);
}

// Function to generate the proof
async function generateProof() {
    try {
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input, 
            path.join(baseDir, './circuit_js/circuit.wasm'), 
            path.join(baseDir, './circuit_final.zkey')
        );

        console.log("Proof generated successfully");

        const proofForSolidity = {
            _pA0: proof.pi_a[0],
            _pA1: proof.pi_a[1],
            _pB00: proof.pi_b[0][0],
            _pB01: proof.pi_b[0][1],
            _pB10: proof.pi_b[1][0],
            _pB11: proof.pi_b[1][1],
            _pC0: proof.pi_c[0],
            _pC1: proof.pi_c[1],
            _pubSignals0: publicSignals[0],
        };

        return { proof: proofForSolidity, publicSignals };
    } catch (err) {
        console.error('Error generating proof:', err);
        return null;
    }
}

async function verifyProofOnChain(proof) {
    try {
        const provider = hre.ethers.provider;
        const verifierAddress = '0x3D4206092FEF5AdDdb20B1f2392D2a2BB3FBe894';
        const verifierABI = [
            "function verifyProof(uint256[2], uint256[2][2], uint256[2], uint256[1]) public view returns (bool)"
        ];

        const verifierContract = new ethers.Contract(verifierAddress, verifierABI, provider);

        const a = [proof._pA0, proof._pA1];
        const b = [[proof._pB00, proof._pB01], [proof._pB10, proof._pB11]];
        const c = [proof._pC0, proof._pC1];
        const input = [proof._pubSignals0];

        // Imprimir todos los valores antes de hacer la llamada
        console.log("Llamando a verifyProof con los siguientes parámetros:");
        console.log("a:", a);
        console.log("b:", b);
        console.log("c:", c);
        console.log("input (señales públicas):", input);

        // Llama a la función verifyProof en el contrato
        const isValid = await verifierContract.verifyProof(a, b, c, input);

        console.log("Resultado de verificación on-chain:", isValid);
        return isValid;
    } catch (error) {
        console.error('Error verifying proof on-chain:', error);
        return false;
    }
}



// Main function to generate proof and verify it on-chain
async function main() {
    const proofData = await generateProof();
    if (!proofData) {
        console.error('Proof generation failed');
        process.exit(1);
    }

    const isValid = await verifyProofOnChain(proofData.proof);
    console.log("On-chain proof verification result:", isValid);
    process.exit(isValid ? 0 : 1);
}

// Execute the main function
main();