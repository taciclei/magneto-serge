// Build script for magneto-serge
// Generates UniFFI bindings for multiple languages

fn main() {
    // Generate UniFFI scaffolding from the UDL file
    uniffi::generate_scaffolding("src/matgto_serge.udl")
        .expect("Failed to generate UniFFI scaffolding");

    // Tell cargo to rerun build script if UDL changes
    println!("cargo:rerun-if-changed=src/matgto_serge.udl");
}
