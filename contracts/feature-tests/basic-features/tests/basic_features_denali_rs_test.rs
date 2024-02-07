use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/basic-features.wasm",
        Box::new(|context| Box::new(basic_features::contract_obj(context))),
    );
    contract_map
}

#[test]
fn big_int_to_i64_rs() {
    dharitri_wasm_debug::denali_rs("denali/big_int_to_i64.scen.json", &contract_map());
}

#[test]
fn big_num_conversions_rs() {
    dharitri_wasm_debug::denali_rs("denali/big_num_conversions.scen.json", &contract_map());
}

#[test]
fn big_uint_sqrt_rs() {
    dharitri_wasm_debug::denali_rs("denali/big_uint_sqrt.scen.json", &contract_map());
}

#[test]
fn big_uint_to_u64_rs() {
    dharitri_wasm_debug::denali_rs("denali/big_uint_to_u64.scen.json", &contract_map());
}

#[test]
fn block_info_rs() {
    dharitri_wasm_debug::denali_rs("denali/block_info.scen.json", &contract_map());
}

#[test]
fn boxed_bytes_zeros_rs() {
    dharitri_wasm_debug::denali_rs("denali/boxed_bytes_zeros.scen.json", &contract_map());
}

#[test]
fn codec_err_rs() {
    dharitri_wasm_debug::denali_rs("denali/codec_err.scen.json", &contract_map());
}

#[test]
fn count_ones_rs() {
    dharitri_wasm_debug::denali_rs("denali/count_ones.scen.json", &contract_map());
}

// #[test]
// fn crypto_elliptic_curves_rs() {
//     dharitri_wasm_debug::denali_rs("denali/crypto_elliptic_curves.scen.json", &contract_map());
// }

#[test]
fn crypto_keccak256_rs() {
    dharitri_wasm_debug::denali_rs("denali/crypto_keccak256.scen.json", &contract_map());
}

// #[test]
// fn crypto_ripemd160_rs() {
//     dharitri_wasm_debug::denali_rs("denali/crypto_ripemd160.scen.json", &contract_map());
// }

#[test]
fn crypto_sha256_rs() {
    dharitri_wasm_debug::denali_rs("denali/crypto_sha256.scen.json", &contract_map());
}

// #[test]
// fn crypto_verify_funcs_rs() {
//     dharitri_wasm_debug::denali_rs("denali/crypto_verify_funcs.scen.json", &contract_map());
// }

#[test]
fn echo_array_u8_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_array_u8.scen.json", &contract_map());
}

#[test]
fn echo_async_result_empty_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_async_result_empty.scen.json", &contract_map());
}

#[test]
fn echo_async_result_empty_managed_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/echo_async_result_empty_managed.scen.json",
        &contract_map(),
    );
}

#[test]
fn echo_big_int_nested_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_big_int_nested.scen.json", &contract_map());
}

#[test]
fn echo_big_int_top_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_big_int_top.scen.json", &contract_map());
}

#[test]
fn echo_big_uint_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_big_uint.scen.json", &contract_map());
}

#[test]
fn echo_boxed_bytes_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_boxed_bytes.scen.json", &contract_map());
}

#[test]
fn echo_i32_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_i32.scen.json", &contract_map());
}

#[test]
fn echo_i64_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_i64.scen.json", &contract_map());
}

#[test]
fn echo_managed_bytes_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_managed_bytes.scen.json", &contract_map());
}

#[test]
fn echo_managed_vec_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_managed_vec.scen.json", &contract_map());
}

#[test]
fn echo_nothing_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_nothing.scen.json", &contract_map());
}

#[test]
fn echo_ser_ex_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_ser_ex_1.scen.json", &contract_map());
}

#[test]
fn echo_slice_u8_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_slice_u8.scen.json", &contract_map());
}

#[test]
fn echo_str_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_str.scen.json", &contract_map());
}

#[test]
fn echo_str_box_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_str_box.scen.json", &contract_map());
}

#[test]
fn echo_string_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_string.scen.json", &contract_map());
}

#[test]
fn echo_tuple_into_multiresult_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/echo_tuple_into_multiresult.scen.json",
        &contract_map(),
    );
}

#[test]
fn echo_u64_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_u64.scen.json", &contract_map());
}

#[test]
fn echo_usize_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_usize.scen.json", &contract_map());
}

#[test]
fn echo_varags_tuples_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_varags_tuples.scen.json", &contract_map());
}

#[test]
fn echo_varargs_u32_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_varargs_u32.scen.json", &contract_map());
}

#[test]
fn echo_vec_u8_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_vec_u8.scen.json", &contract_map());
}

#[test]
fn events_rs() {
    dharitri_wasm_debug::denali_rs("denali/events.scen.json", &contract_map());
}

#[test]
fn events_legacy_rs() {
    dharitri_wasm_debug::denali_rs("denali/events_legacy.scen.json", &contract_map());
}

#[test]
fn get_caller_rs() {
    dharitri_wasm_debug::denali_rs("denali/get_caller.scen.json", &contract_map());
}

#[test]
fn get_cumulated_validator_rewards_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/get_cumulated_validator_rewards.scen.json",
        &contract_map(),
    );
}

// TODO: uncomment after implemented the full DCT format in denali-rs
// #[test]
// fn get_dct_local_roles_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/get_dct_local_roles.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn managed_buffer_concat_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_concat_1.scen.json", &contract_map());
}

#[test]
fn managed_buffer_concat_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_concat_2.scen.json", &contract_map());
}

#[test]
fn managed_buffer_eq_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_eq.scen.json", &contract_map());
}

#[test]
fn managed_buffer_overwrite_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_overwrite.scen.json", &contract_map());
}

#[test]
fn managed_buffer_slice_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_slice_1.scen.json", &contract_map());
}

#[test]
fn managed_buffer_slice_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_slice_2.scen.json", &contract_map());
}

#[test]
fn managed_vec_address_push_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_vec_address_push.scen.json", &contract_map());
}

#[test]
fn managed_vec_biguint_push_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_vec_biguint_push.scen.json", &contract_map());
}

#[test]
fn only_owner_rs() {
    dharitri_wasm_debug::denali_rs("denali/only_owner.scen.json", &contract_map());
}

// Will never run in denali-rs.
// #[test]
// fn out_of_gas_rs() {
//     dharitri_wasm_debug::denali_rs("denali/out_of_gas.scen.json", &contract_map());
// }

#[test]
fn panic_rs() {
    dharitri_wasm_debug::denali_rs("denali/panic.scen.json", &contract_map());
}

#[test]
fn return_codes_rs() {
    dharitri_wasm_debug::denali_rs("denali/return_codes.scen.json", &contract_map());
}

// TODO: Fix by implementing is_smart_contract mock
/*
#[test]
fn sc_properties_rs() {
    dharitri_wasm_debug::denali_rs("denali/sc_properties.scen.json", &contract_map());
}
*/

#[test]
fn sc_result_rs() {
    dharitri_wasm_debug::denali_rs("denali/sc_result.scen.json", &contract_map());
}

#[test]
fn storage_addr_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_addr.scen.json", &contract_map());
}

#[test]
fn storage_big_int_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_big_int.scen.json", &contract_map());
}

#[test]
fn storage_big_uint_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_big_uint.scen.json", &contract_map());
}

#[test]
fn storage_bool_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_bool.scen.json", &contract_map());
}

#[test]
fn storage_clear_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_clear.scen.json", &contract_map());
}

#[test]
fn storage_i64_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_i64.scen.json", &contract_map());
}

#[test]
fn storage_i64_bad_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_i64_bad.scen.json", &contract_map());
}

#[test]
fn storage_map1_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_map1.scen.json", &contract_map());
}

#[test]
fn storage_map2_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_map2.scen.json", &contract_map());
}

#[test]
fn storage_map3_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_map3.scen.json", &contract_map());
}

#[test]
fn storage_mapper_linked_list_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/storage_mapper_linked_list.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_queue_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_mapper_queue.scen.json", &contract_map());
}

#[test]
fn storage_mapper_map_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_mapper_map.scen.json", &contract_map());
}

#[test]
fn storage_mapper_map_storage_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/storage_mapper_map_storage.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_set_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_mapper_set.scen.json", &contract_map());
}

#[test]
fn storage_mapper_single_value_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/storage_mapper_single_value.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_token_attributes_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/storage_mapper_token_attributes.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_vec_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_mapper_vec.scen.json", &contract_map());
}

#[test]
fn storage_opt_addr_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_opt_addr.scen.json", &contract_map());
}

#[test]
fn storage_reserved_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_reserved.scen.json", &contract_map());
}

#[test]
fn storage_u64_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_u64.scen.json", &contract_map());
}

#[test]
fn storage_u64_bad_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_u64_bad.scen.json", &contract_map());
}

#[test]
fn storage_usize_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_usize.scen.json", &contract_map());
}

#[test]
fn storage_usize_bad_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_usize_bad.scen.json", &contract_map());
}

#[test]
fn storage_vec_u8_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_vec_u8.scen.json", &contract_map());
}
