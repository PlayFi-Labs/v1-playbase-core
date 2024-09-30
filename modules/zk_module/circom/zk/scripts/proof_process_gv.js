const path = require('path');
const snarkjs = require('snarkjs');
const hre = require('hardhat');
const ethers = hre.ethers;

// Get the absolute path to the current directory
const baseDir = __dirname;

// Helper function to check if a value is a valid BigNumber string
function isValidBigNumberString(value) {
    return typeof value === 'string' && !isNaN(value) && value.length > 0;
}

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
const input = require(path.join(baseDir, '../../utils/input_preprocessed.json'));

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

// Function to generate the proof and public signals
async function generateProof() {
    try {
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input,
            path.join(baseDir, '../../utils/circuit_js/circuit.wasm'),
            path.join(baseDir, '../../utils/circuit_final.zkey')
        );

        console.log("Proof generated successfully");
        console.log("Proof output:", proof);
        console.log("Public Signals:", publicSignals);

        // Export the proof and public signals to Solidity-compatible calldata
        const calldata = await snarkjs.groth16.exportSolidityCallData(proof, publicSignals);

        // Split the calldata and map to BigInt
        const argv = calldata
            .replace(/["[\]\s]/g, "")
            .split(",")
            .map((x) => BigInt(x).toString());

        // Extract proof components
        const a = [argv[0], argv[1]];
        const b = [
            [argv[2], argv[3]],
            [argv[4], argv[5]]
        ];
        const c = [argv[6], argv[7]];
        const Input = argv.slice(8);

        console.log("Proof components prepared for Solidity verification.");
        return { a, b, c, Input };
    } catch (err) {
        console.error('Error generating proof:', err);
        return null;
    }
}

// Function to verify the proof on-chain
async function verifyProofOnChain(proof, Input) {
    try {
        const provider = hre.ethers.provider;
        const verifierAddress = '0x2064bB114104A593bA3349B4a192C2442e846a3A'; // Replace with actual deployed verifier address
        const verifierABI = [
            "function verifyProof(uint256[2], uint256[2][2], uint256[2], uint256[1]) public view returns (bool)"
        ];

        const verifierContract = new ethers.Contract(verifierAddress, verifierABI, provider);

        // Debugging: Print the values being sent to verifyProof
        console.log("Proof parameters:");
        console.log("a:", proof.a);
        console.log("b:", proof.b);
        console.log("c:", proof.c);
        console.log("input (public signals):", Input);

        // Call the on-chain verification function
        const isValid = await verifierContract.verifyProof(proof.a, proof.b, proof.c, Input);

        console.log("On-chain verification result:", isValid);
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

    const { a, b, c, Input } = proofData;

    const isValid = await verifyProofOnChain({ a, b, c }, Input);
    console.log("On-chain proof verification result:", isValid);
    process.exit(isValid ? 0 : 1);
}

// Execute the main function
main();
