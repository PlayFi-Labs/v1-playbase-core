const path = require('path');
const fs = require('fs');
const snarkjs = require('snarkjs');

// Get the absolute path to the current directory
const baseDir = __dirname; // This gives the directory where preprocess_input.js is located

function adjustArraySize(arr, targetLength) {
    if (arr.length > targetLength) {
        console.error(`Error: Array exceeds expected length. Got ${arr.length}, expected ${targetLength}.`);
        return arr.slice(0, targetLength);  // Truncate the array to the required size
    } else if (arr.length < targetLength) {
        while (arr.length < targetLength) {
            arr.push(0);  // Pad with zeros if it's too short
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
    // Remove '0x' prefix if present
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
    const unixTime = Math.floor(new Date(dateStr).getTime() / 1000);  // Convert to UNIX timestamp (seconds)
    const byteArray = [];
    let temp = unixTime;
    while (temp > 0) {
        byteArray.push(temp & 0xff);  // Push least significant byte
        temp = temp >> 8;  // Shift right by 8 bits to get the next byte
    }
    return adjustArraySize(byteArray.reverse(), targetLength);  // Reverse to get the most significant byte first
}

// Use absolute paths for files
let input;
try {
    input = require(path.join(baseDir, 'input_preprocessed.json'));
} catch (error) {
    console.error("Error loading input_preprocessed.json:", error);
    process.exit(1);
}

// Validate the input and apply conversions
try {
    input.user = stringToAsciiArray(input.user, 8);
    input.game = stringToAsciiArray(input.game, 16);
    input.character = stringToAsciiArray(input.character, 12);
    input.ability = stringToAsciiArray(input.ability, 10);
    input.place = stringToAsciiArray(input.place, 10);
    input.place2 = stringToAsciiArray(input.place2, 10);
    input.uploader = hexStringToByteArray(input.uploader, 20); // Convert hexadecimal to byte array
    input.timestamp = dateToUnixBytes(input.timestamp, 8); // Convert timestamp to UNIX and then bytes
    input.hash_inputdata = adjustArraySize(input.hash_inputdata, 32);

    // Logs to verify input sizes
    console.log(`user: ${input.user.length} (expected 8)`);
    console.log(`game: ${input.game.length} (expected 16)`);
    console.log(`character: ${input.character.length} (expected 12)`);
    console.log(`ability: ${input.ability.length} (expected 10)`);
    console.log(`place: ${input.place.length} (expected 10)`);
    console.log(`place2: ${input.place2.length} (expected 10)`);
    console.log(`uploader: ${input.uploader.length} (expected 20)`);
    console.log(`hash_inputdata: ${input.hash_inputdata.length} (expected 32)`);
    console.log(`timestamp: ${input.timestamp.length} (expected 8)`);
} catch (error) {
    console.error("Error processing input fields:", error);
    process.exit(1);
}

// Write the processed input back to an input.json file
const outputFilePath = path.join(baseDir, 'input.json');
try {
    fs.writeFileSync(outputFilePath, JSON.stringify(input, null, 2));
    console.log("Processed input written to input.json");
} catch (error) {
    console.error("Error writing input.json:", error);
    process.exit(1);
}

async function generateProofAndVerify() {
    try {
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input, 
            path.join(baseDir, './circuit_js/circuit.wasm'), 
            path.join(baseDir, './circuit_final.zkey')
        );
        
        const vKey = JSON.parse(fs.readFileSync(path.join(baseDir, 'verification_key.json')));
        const isValid = await snarkjs.groth16.verify(vKey, publicSignals, proof);
        return isValid;
    } catch (err) {
        console.error('Error during proof generation or verification:', err);
        return false;
    }
}

(async () => {
    const result = await generateProofAndVerify();
    console.log("Proof verification result:", result);
    process.exit(result ? 0 : 1);
})();
