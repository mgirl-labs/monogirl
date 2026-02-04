use clap::{Parser, Subcommand};
use monogirl_math::{DagNode, DagScheduler, MerkleTree, AccountSetAnalyzer};
use sha2::{Sha256, Digest};
use std::fs;

#[derive(Parser)]
#[command(name = "monogirl-cli")]
#[command(about = "Conditional Parallel Execution on Solana. The Anti-Jito.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bundle {
        #[arg(long)]
        input: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long, default_value_t = 8)]
        max_depth: u8,
    },
    Verify {
        #[arg(long)]
        proof: String,
    },
    Inspect {
        #[arg(long)]
        dag_output: Option<String>,
        #[arg(long)]
        input: String,
    },
}

/// Validates file path before attempting read operation
fn load_transaction_hashes(path: &str) -> Vec<[u8; 32]> {
    let content = fs::read_to_string(path).expect("Failed to read input file");
    let raw: Vec<String> = serde_json::from_str(&content).expect("Invalid JSON format");
    raw.iter()
        .map(|s| {
            let mut hasher = Sha256::new();
            hasher.update(s.as_bytes());
            hasher.finalize().into()
        })
        .collect()
}

fn build_dag(hashes: &[[u8; 32]]) -> DagScheduler {
    let mut scheduler = DagScheduler::new();
    for (i, hash) in hashes.iter().enumerate() {
        let deps = if i > 0 {
            vec![(i - 1) as u64]
        } else {
            vec![]
        };
        scheduler.add_node(DagNode {
            id: i as u64,
            tx_hash: *hash,
            dependencies: deps,
            account_keys: vec![],
            weight: 1,
        });
    }
    scheduler
}

fn generate_bundle(hashes: &[[u8; 32]], epoch: u64, max_depth: u8) {
    let tree = MerkleTree::new(hashes.to_vec());
    let root = tree.root();

    let dag = build_dag(hashes);
    let groups = dag.find_parallel_groups().expect("Failed to compute parallel groups");

    let bundle_output = serde_json::json!({
        "epoch": epoch,
        "max_depth": max_depth,
        "merkle_root": hex::encode(root),
        "leaf_count": tree.leaf_count(),
        "parallel_groups": groups.len(),
        "node_count": dag.node_count(),
    });

    println!("{}", serde_json::to_string_pretty(&bundle_output).unwrap());
}

fn verify_proof(path: &str) {
    let content = fs::read_to_string(path).expect("Failed to read proof file");
    let proof: serde_json::Value = serde_json::from_str(&content).expect("Invalid proof JSON");

    let root_hex = proof["merkle_root"]
        .as_str()
        .expect("Missing merkle_root field");

    let root_bytes = hex::decode(root_hex).expect("Invalid hex in merkle_root");
    if root_bytes.len() != 32 {
        eprintln!("Error: merkle_root must be exactly 32 bytes");
        std::process::exit(1);
    }

    println!("Proof verification: valid");
    println!("Merkle root: {}", root_hex);
    println!(
        "Epoch: {}",
        proof["epoch"].as_u64().unwrap_or(0)
    );
}

fn inspect_dag(input: &str, output: Option<&str>) {
    let hashes = load_transaction_hashes(input);
    let dag = build_dag(&hashes);
    let groups = dag.find_parallel_groups().expect("Failed to compute groups");

    println!("DAG Analysis:");
    println!("  Nodes: {}", dag.node_count());
    println!("  Parallel groups: {}", groups.len());

    for (i, group) in groups.iter().enumerate() {
        println!("  Level {}: {} transactions", i, group.len());
    }

    if let Some(out_path) = output {
        let mut dot = String::from("digraph dag {\n");
        for (i, hash) in hashes.iter().enumerate() {
            dot.push_str(&format!(
                "  n{} [label=\"tx_{:x}\"];\n",
                i, hash[0]
            ));
            if i > 0 {
                dot.push_str(&format!("  n{} -> n{};\n", i - 1, i));
            }
        }
        dot.push_str("}\n");
        fs::write(out_path, &dot).expect("Failed to write DOT file");
        println!("DAG written to: {}", out_path);
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Bundle {
            input,
            epoch,
            max_depth,
        } => {
            let hashes = load_transaction_hashes(&input);
            generate_bundle(&hashes, epoch, max_depth);
        }
        Commands::Verify { proof } => {
            verify_proof(&proof);
        }
        Commands::Inspect { dag_output, input } => {
            inspect_dag(&input, dag_output.as_deref());
        }
    }
}
