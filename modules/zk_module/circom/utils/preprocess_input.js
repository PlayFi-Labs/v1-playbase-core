const path = require('path');
const fs = require('fs');
const snarkjs = require('snarkjs');

// Get the absolute path to the current directory
const baseDir = __dirname; // This gives the directory where preprocess_input.js is located

function adjustArraySize(arr, targetLength) {
    if (arr.length > targetLength) {
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
    const asciiArray = str.split('').map(char => char.charCodeAt(0));
    return adjustArraySize(asciiArray, targetLength);
}

// Use absolute paths for files
const input = require(path.join(baseDir, 'input_preprocessed.json'));

input.game = stringToAsciiArray(input.game, 16);
input.character = stringToAsciiArray(input.character, 12);
input.ability = stringToAsciiArray(input.ability, 10);
input.place = stringToAsciiArray(input.place, 10);
input.place2 = stringToAsciiArray(input.place2, 10);
input.uploader = stringToAsciiArray(input.uploader, 20);
input.hash_inputdata = adjustArraySize(input.hash_inputdata, 32);
input.timestamp = stringToAsciiArray(input.timestamp, 8);

async function generateProofAndVerify() {
    try {
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input, 
            path.join(baseDir, './circuit_js/circuit.wasm'), 
            path.join(baseDir, 'circuit_final.zkey')
        );
        const vKey = JSON.parse(fs.readFileSync(path.join(baseDir, 'verification_key.json')));
        return await snarkjs.groth16.verify(vKey, publicSignals, proof);
    } catch (err) {
        console.error('Error:', err);
        return false;
    }
}

(async () => {
    const result = await generateProofAndVerify();
    console.log(result);
    process.exit(result ? 0 : 1);
})();
