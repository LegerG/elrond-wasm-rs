use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/erc1155-marketplace.wasm",
        erc1155_marketplace::ContractBuilder,
    );
    blockchain.register_contract_builder(
        "file:../erc1155/output/erc1155.wasm",
        erc1155::ContractBuilder,
    );

    blockchain
}

#[test]
fn auction_single_token_egld_test_rs() {
    elrond_wasm_debug::mandos_rs("mandos/auction_single_token_egld.scen.json", world());
}

#[test]
fn auction_batch_test_rs() {
    elrond_wasm_debug::mandos_rs("mandos/auction_batch.scen.json", world());
}

#[test]
fn bid_first_egld_test_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_first_egld.scen.json", world());
}

#[test]
fn bid_second_egld_test_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_second_egld.scen.json", world());
}

#[test]
fn bid_third_egld_test_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bid_third_egld.scen.json", world());
}

#[test]
fn end_auction_test_rs() {
    elrond_wasm_debug::mandos_rs("mandos/end_auction.scen.json", world());
}
