use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
};


use starknet_rs::{
    core::contract_address::{
        starknet_contract_address::compute_deprecated_class_hash,
        v2::starknet_sierra_contract_address::compute_sierra_class_hash,
    },
    services::api::contract_classes::deprecated_contract_class::ContractClass,
};

fn main() {
    let path = PathBuf::from("factorial.json");
    let contract_class = ContractClass::try_from(path).unwrap();

    println!(
        "compute_deprecated_class_hash: {}",
        compute_deprecated_class_hash(&contract_class).unwrap()
    );

    let path = PathBuf::from("fibonacci.sierra");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sierra_contract_class: cairo_lang_starknet::contract_class::ContractClass =
        serde_json::from_reader(reader).unwrap();

    println!(
        "compute_sierra_class_hash: {}",
        compute_sierra_class_hash(&sierra_contract_class).unwrap()
    );
}
