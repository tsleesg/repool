#![cfg(test)]
pub mod utils;
use utils::*;

use code_vm_api::{prelude::*, utils::hashv};
use solana_sdk::signer::Signer;

#[test]
fn run_system_account_decompress() {
    let (mut svm, payer, _mint_owner, _mint_pk, vm_address) =
        setup_svm_with_payer_and_vm(21);

    let name = "test";
    let mem_layout = MemoryLayout::Mixed;

    let (vm_mem_address, _) =
        create_and_resize_memory(&mut svm, &payer, vm_address, mem_layout, name);

    let (vm_storage_address, _) =
        create_storage_account(&mut svm, &payer, vm_address, name);

    let virtual_account_owner = create_keypair().pubkey();
    let account_index = 0;
    assert!(tx_create_virtual_nonce(&mut svm, &payer, vm_address, vm_mem_address, virtual_account_owner, account_index).is_ok());

    let account = svm.get_account(&vm_mem_address).unwrap();
    let compact_mem = MemoryAccount::into_indexed_memory(&account.data);
    let data = compact_mem.read_item(account_index).unwrap();

    let va = VirtualAccount::unpack(&data).unwrap();
    let va_hash = va.get_hash();

    let sig = Signature::new(payer.sign_message(va_hash.as_ref()).as_ref());
    let sig_hash = hashv(&[sig.as_ref(), va_hash.as_ref()]);
    
    assert!(tx_account_compress(
        &mut svm, 
        &payer,
        vm_address,
        vm_mem_address,
        vm_storage_address,
        account_index,
        sig
    ).is_ok());

    let account = svm.get_account(&vm_mem_address).unwrap();
    let compact_mem = MemoryAccount::into_indexed_memory(&account.data);
    assert!(compact_mem.is_empty(account_index));

    let compressed_mem = get_storage_account(&svm, vm_storage_address).compressed_state;
    let mut expected = MerkleTree::<{StorageAccount::MERKLE_TREE_DEPTH}>::new(&[
        MERKLE_TREE_SEED,
        create_name(name).as_ref(),
        vm_address.as_ref()
    ]);
    assert!(expected.try_insert(sig_hash).is_ok());
    assert_eq!(expected.get_root(), compressed_mem.get_root());

    let packed_va = va.pack();
    let proof = expected.get_merkle_proof(&[sig_hash], 0);
    let account_index = 42;

    assert!(tx_account_decompress(
        &mut svm, 
        &payer, 
        vm_address,
        vm_mem_address,
        vm_storage_address,
        None,
        None,
        account_index,
        packed_va,
        proof.clone(),
        sig
    ).is_ok());

    let compressed_mem = get_storage_account(&svm, vm_storage_address).compressed_state;

    assert!(expected.try_remove(&proof, sig_hash).is_ok());
    assert_eq!(expected.get_root(), compressed_mem.get_root());

    let account = svm.get_account(&vm_mem_address).unwrap();
    let compact_mem = MemoryAccount::into_indexed_memory(&account.data);
    assert!(compact_mem.is_empty(0));
    assert!(compact_mem.has_item(account_index));

    let data = compact_mem.read_item(account_index).unwrap();
    let va = VirtualAccount::unpack(&data).unwrap();
    assert!(va.into_inner_nonce().is_some());
}