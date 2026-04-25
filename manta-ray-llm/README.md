# Manta Ray LLM

A simple LLM tool for climate care initiatives, built in Rust.

## Features

- Answer questions about climate care using Hugging Face models
- Perform calculations for carbon footprint estimates
- Generate petition texts for corporations

## Usage

Build the project:

```bash
cargo build --release
```

Run commands:

```bash
# Answer a question
./target/release/manta-ray-llm answer --question "What is carbon footprint?" --context "Carbon footprint is the total amount of greenhouse gases emitted..."

# Calculate
./target/release/manta-ray-llm calculate --expression "2 + 3 * 4"

# Generate petition
./target/release/manta-ray-llm petition --corporation "BigCorp" --issue "using private jets"
```

## Dependencies

- rust-bert: For question answering
- clap: For CLI
- evalexpr: For calculations