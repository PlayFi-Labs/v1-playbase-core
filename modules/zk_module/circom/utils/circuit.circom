pragma circom 2.1.5;
include "./circomlib/circuits/poseidon.circom";

template SimpleProof() {
    signal input game[16];
    signal input character[12];
    signal input ability[10];
    signal input place[10];
    signal input place2[10];
    signal input aimodel;
    signal input aiversion;
    signal input ainode;
    signal input uploader[20];
    signal input timestamp[8];
    signal input source;
    signal input sourcetype;
    signal input hash_inputdata[32];

    // Poseidon hash component for first 16 elements of hash_inputdata
    component hash1 = Poseidon(16);

    for (var i = 0; i < 16; i++) {
        hash1.inputs[i] <== hash_inputdata[i];
    }

    // Poseidon hash component for the remaining 16 elements of hash_inputdata
    component hash2 = Poseidon(16);

    for (var i = 0; i < 16; i++) {
        hash2.inputs[i] <== hash_inputdata[i + 16];
    }

    // Hash the results of the two previous hashes
    component finalHash = Poseidon(2);
    finalHash.inputs[0] <== hash1.out;
    finalHash.inputs[1] <== hash2.out;

    signal output proof;
    proof <== finalHash.out;
}

component main = SimpleProof();
