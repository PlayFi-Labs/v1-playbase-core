const path = require('path');
const snarkjs = require('snarkjs');

// Get the absolute path to the current directory
const baseDir = __dirname;

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
    const unixTime = Math.floor(new Date(dateStr).getTime() / 1000);  // Convert to UNIX timestamp in seconds
    const byteArray = [];
    let temp = unixTime;
    while (temp > 0) {
        byteArray.push(temp & 0xff);  // Extract the least significant byte
        temp = temp >> 8;  // Shift right by 8 bits
    }
    return adjustArraySize(byteArray.reverse(), targetLength);  // Reverse to get most significant byte first
}

// Use absolute paths for files
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

    // Log information to stderr to avoid interfering with JSON output
    console.error(`user: ${input.user.length} (expected 8)`);
    console.error(`game: ${input.game.length} (expected 16)`);
    console.error(`character: ${input.character.length} (expected 12)`);
    console.error(`ability: ${input.ability.length} (expected 10)`);
    console.error(`place: ${input.place.length} (expected 10)`);
    console.error(`place2: ${input.place2.length} (expected 10)`);
    console.error(`uploader: ${input.uploader.length} (expected 20)`);
    console.error(`hash_inputdata: ${input.hash_inputdata.length} (expected 32)`);
    console.error(`timestamp: ${input.timestamp.length} (expected 8)`);
} catch (error) {
    console.error("Error processing input fields:", error);
    process.exit(1);
}

async function generateProof() {
    try {
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input, 
            path.join(baseDir, './circuit_js/circuit.wasm'), 
            path.join(baseDir, 'circuit_final.zkey')
        );

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

        console.log(JSON.stringify(proofForSolidity));  // Output JSON directly to stdout
    } catch (err) {
        console.error('Error generating proof:', err);
        process.exit(1);
    }
}

(async () => {
    await generateProof();
    process.exit(0);  // Ensure the script ends properly
})();