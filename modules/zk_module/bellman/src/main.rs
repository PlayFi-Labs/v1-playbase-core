use bellman::groth16::{generate_random_parameters, create_random_proof, prepare_verifying_key, verify_proof};
use bellman::{ConstraintSystem, SynthesisError, Circuit};
use pairing::bls12_381::{Bls12, Fr};
use ff::{Field, PrimeField};
use rand::thread_rng;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

struct ZKCircuit {
    pub user: Option<Fr>,
    pub game: Option<Fr>,
    pub character: Option<Fr>,
    pub ability: Option<Fr>,
    pub place: Option<Fr>,
    pub place2: Option<Fr>,
    pub aimodel: Option<Fr>,
    pub aiversion: Option<Fr>,
    pub ainode: Option<Fr>,
    pub uploader: Option<Fr>,
    pub timestamp: Option<Fr>,
    pub source: Option<Fr>,
    pub sourcetype: Option<Fr>,
    pub hash_inputdata: Vec<Option<Fr>>,
}

impl Circuit<Fr> for ZKCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocating each field from the JSON input to the circuit.
        let user = cs.alloc(|| "user", || self.user.ok_or(SynthesisError::AssignmentMissing))?;
        let game = cs.alloc(|| "game", || self.game.ok_or(SynthesisError::AssignmentMissing))?;
        let character = cs.alloc(|| "character", || self.character.ok_or(SynthesisError::AssignmentMissing))?;
        let ability = cs.alloc(|| "ability", || self.ability.ok_or(SynthesisError::AssignmentMissing))?;
        let place = cs.alloc(|| "place", || self.place.ok_or(SynthesisError::AssignmentMissing))?;
        let place2 = cs.alloc(|| "place2", || self.place2.ok_or(SynthesisError::AssignmentMissing))?;
        let aimodel = cs.alloc(|| "aimodel", || self.aimodel.ok_or(SynthesisError::AssignmentMissing))?;
        let aiversion = cs.alloc(|| "aiversion", || self.aiversion.ok_or(SynthesisError::AssignmentMissing))?;
        let ainode = cs.alloc(|| "ainode", || self.ainode.ok_or(SynthesisError::AssignmentMissing))?;
        let uploader = cs.alloc(|| "uploader", || self.uploader.ok_or(SynthesisError::AssignmentMissing))?;
        let timestamp = cs.alloc(|| "timestamp", || self.timestamp.ok_or(SynthesisError::AssignmentMissing))?;
        let source = cs.alloc(|| "source", || self.source.ok_or(SynthesisError::AssignmentMissing))?;
        let sourcetype = cs.alloc(|| "sourcetype", || self.sourcetype.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocating each element of the hash_inputdata array.
        let mut input_hash_data = Vec::new();
        for (i, input) in self.hash_inputdata.iter().enumerate() {
            let data = cs.alloc(|| format!("hash_inputdata[{}]", i), || input.ok_or(SynthesisError::AssignmentMissing))?;
            input_hash_data.push(data);
        }

        // Simple constraint example: you can define custom logic here to hash the inputs or compare them.
        // For instance, you can hash the data and compare the result with a precomputed hash value.
        // Example constraint: user + game = character (this is just a simple illustrative example)
        cs.enforce(
            || "user + game = character",
            |lc| lc + user + game,
            |lc| lc + CS::one(),
            |lc| lc + character,
        );

        // You can also add more complex constraints involving multiple variables and hash computations here.
        // The current constraints are just placeholders and should be adapted to your circuit's needs.

        Ok(())
    }
}

fn main() {
    let rng = &mut thread_rng();

    // Load the input JSON from file
    let file = File::open("json_input.json").expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Parse each value from the JSON into Fr (finite field elements) for the circuit
    let user_hash = Fr::from_str(&json["user"].as_str().unwrap().to_string()).unwrap();
    let game_hash = Fr::from_str(&json["game"].as_str().unwrap().to_string()).unwrap();
    let character_hash = Fr::from_str(&json["character"].as_str().unwrap().to_string()).unwrap();
    let ability_hash = Fr::from_str(&json["ability"].as_str().unwrap().to_string()).unwrap();
    let place_hash = Fr::from_str(&json["place"].as_str().unwrap().to_string()).unwrap();
    let place2_hash = Fr::from_str(&json["place2"].as_str().unwrap().to_string()).unwrap();
    let aimodel_hash = Fr::from_str(&json["aimodel"].to_string()).unwrap();
    let aiversion_hash = Fr::from_str(&json["aiversion"].to_string()).unwrap();
    let ainode_hash = Fr::from_str(&json["ainode"].to_string()).unwrap();
    let uploader_hash = Fr::from_str(&json["uploader"].as_str().unwrap().to_string()).unwrap();
    let timestamp_hash = Fr::from_str(&json["timestamp"].as_str().unwrap().to_string()).unwrap();
    let source_hash = Fr::from_str(&json["source"].to_string()).unwrap();
    let sourcetype_hash = Fr::from_str(&json["sourcetype"].to_string()).unwrap();

    // Prepare the hash_inputdata array
    let hash_inputdata = json["hash_inputdata"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| Some(Fr::from_str(&v.to_string()).unwrap()))
        .collect::<Vec<Option<Fr>>>();

    // Initialize the circuit with parsed values
    let circuit = ZKCircuit {
        user: Some(user_hash),
        game: Some(game_hash),
        character: Some(character_hash),
        ability: Some(ability_hash),
        place: Some(place_hash),
        place2: Some(place2_hash),
        aimodel: Some(aimodel_hash),
        aiversion: Some(aiversion_hash),
        ainode: Some(ainode_hash),
        uploader: Some(uploader_hash),
        timestamp: Some(timestamp_hash),
        source: Some(source_hash),
        sourcetype: Some(sourcetype_hash),
        hash_inputdata,
    };

    // Generate the proving parameters (this step is computationally expensive)
    let params = generate_random_parameters::<Bls12, _, _>(circuit.clone(), rng).unwrap();

    // Create the zk-SNARK proof
    let proof = create_random_proof(circuit, &params, rng).unwrap();

    // Verify the proof locally
    let pvk = prepare_verifying_key(&params.vk);
    let result = verify_proof(&pvk, &proof, &[]).unwrap();
    println!("Proof verified: {}", result);

    // Save the verification key to a file for SnarkJS usage (to generate Verifier.sol)
    use std::fs::File;
    use std::io::Write;
    let mut vk_file = File::create("verification_key.json").unwrap();
    let vk_data = serde_json::to_string(&params.vk).unwrap();
    vk_file.write_all(vk_data.as_bytes()).unwrap();
}
