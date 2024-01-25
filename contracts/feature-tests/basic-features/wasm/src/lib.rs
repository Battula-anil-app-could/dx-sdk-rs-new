////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    basic_features::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    basic_features::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_assign_big_int() {
    basic_features::endpoints::add_assign_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_assign_big_int_ref() {
    basic_features::endpoints::add_assign_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_assign_big_uint() {
    basic_features::endpoints::add_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_assign_big_uint_ref() {
    basic_features::endpoints::add_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_big_int() {
    basic_features::endpoints::add_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_big_int_ref() {
    basic_features::endpoints::add_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_big_uint() {
    basic_features::endpoints::add_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn add_big_uint_ref() {
    basic_features::endpoints::add_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_int_from_biguint() {
    basic_features::endpoints::big_int_from_biguint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_int_from_i64_1() {
    basic_features::endpoints::big_int_from_i64_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_int_from_i64_2() {
    basic_features::endpoints::big_int_from_i64_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_int_to_i64() {
    basic_features::endpoints::big_int_to_i64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_int_to_parts() {
    basic_features::endpoints::big_int_to_parts(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_int_zero() {
    basic_features::endpoints::big_int_zero(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_uint_from_u64_1() {
    basic_features::endpoints::big_uint_from_u64_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_uint_from_u64_2() {
    basic_features::endpoints::big_uint_from_u64_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_uint_to_u64() {
    basic_features::endpoints::big_uint_to_u64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn big_uint_zero() {
    basic_features::endpoints::big_uint_zero(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_and_assign_big_uint() {
    basic_features::endpoints::bit_and_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_and_assign_big_uint_ref() {
    basic_features::endpoints::bit_and_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_and_big_uint() {
    basic_features::endpoints::bit_and_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_and_big_uint_ref() {
    basic_features::endpoints::bit_and_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_or_assign_big_uint() {
    basic_features::endpoints::bit_or_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_or_assign_big_uint_ref() {
    basic_features::endpoints::bit_or_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_or_big_uint() {
    basic_features::endpoints::bit_or_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_or_big_uint_ref() {
    basic_features::endpoints::bit_or_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_xor_assign_big_uint() {
    basic_features::endpoints::bit_xor_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_xor_assign_big_uint_ref() {
    basic_features::endpoints::bit_xor_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_xor_big_uint() {
    basic_features::endpoints::bit_xor_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bit_xor_big_uint_ref() {
    basic_features::endpoints::bit_xor_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn boxed_bytes_concat_2() {
    basic_features::endpoints::boxed_bytes_concat_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn boxed_bytes_split() {
    basic_features::endpoints::boxed_bytes_split(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn boxed_bytes_zeros() {
    basic_features::endpoints::boxed_bytes_zeros(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn clear_single_value_mapper() {
    basic_features::endpoints::clear_single_value_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn clear_storage_value() {
    basic_features::endpoints::clear_storage_value(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_contract_call() {
    basic_features::endpoints::codec_err_contract_call(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_contract_init() {
    basic_features::endpoints::codec_err_contract_init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_event_data() {
    basic_features::endpoints::codec_err_event_data(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_event_topic() {
    basic_features::endpoints::codec_err_event_topic(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_finish() {
    basic_features::endpoints::codec_err_finish(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_storage_get() {
    basic_features::endpoints::codec_err_storage_get(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_storage_key() {
    basic_features::endpoints::codec_err_storage_key(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn codec_err_storage_set() {
    basic_features::endpoints::codec_err_storage_set(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compare_h256() {
    basic_features::endpoints::compare_h256(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn computeKeccak256() {
    basic_features::endpoints::computeKeccak256(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn computeRipemd160() {
    basic_features::endpoints::computeRipemd160(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn computeSha256() {
    basic_features::endpoints::computeSha256(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_create_ec() {
    basic_features::endpoints::compute_create_ec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_ec_add() {
    basic_features::endpoints::compute_ec_add(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_ec_double() {
    basic_features::endpoints::compute_ec_double(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_generate_key_ec() {
    basic_features::endpoints::compute_generate_key_ec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_get_ec_length() {
    basic_features::endpoints::compute_get_ec_length(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_get_priv_key_byte_length() {
    basic_features::endpoints::compute_get_priv_key_byte_length(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_get_values() {
    basic_features::endpoints::compute_get_values(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_is_on_curve_ec() {
    basic_features::endpoints::compute_is_on_curve_ec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_marshal_compressed_ec() {
    basic_features::endpoints::compute_marshal_compressed_ec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_marshal_ec() {
    basic_features::endpoints::compute_marshal_ec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_scalar_base_mult() {
    basic_features::endpoints::compute_scalar_base_mult(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_scalar_mult() {
    basic_features::endpoints::compute_scalar_mult(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_secp256k1_der_signature() {
    basic_features::endpoints::compute_secp256k1_der_signature(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_unmarshal_compressed_ec() {
    basic_features::endpoints::compute_unmarshal_compressed_ec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn compute_unmarshal_ec() {
    basic_features::endpoints::compute_unmarshal_ec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn count_ones() {
    basic_features::endpoints::count_ones(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_assign_big_int() {
    basic_features::endpoints::div_assign_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_assign_big_int_ref() {
    basic_features::endpoints::div_assign_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_assign_big_uint() {
    basic_features::endpoints::div_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_assign_big_uint_ref() {
    basic_features::endpoints::div_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_big_int() {
    basic_features::endpoints::div_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_big_int_ref() {
    basic_features::endpoints::div_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_big_uint() {
    basic_features::endpoints::div_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn div_big_uint_ref() {
    basic_features::endpoints::div_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_array_u8() {
    basic_features::endpoints::echo_array_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_async_result_empty() {
    basic_features::endpoints::echo_async_result_empty(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_big_int() {
    basic_features::endpoints::echo_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_big_int_option() {
    basic_features::endpoints::echo_big_int_option(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_big_int_tuple() {
    basic_features::endpoints::echo_big_int_tuple(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_big_int_vec() {
    basic_features::endpoints::echo_big_int_vec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_big_uint() {
    basic_features::endpoints::echo_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_bool() {
    basic_features::endpoints::echo_bool(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_boxed_array_u8() {
    basic_features::endpoints::echo_boxed_array_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_boxed_bytes() {
    basic_features::endpoints::echo_boxed_bytes(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_boxed_ser_example_1() {
    basic_features::endpoints::echo_boxed_ser_example_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_boxed_ser_example_2() {
    basic_features::endpoints::echo_boxed_ser_example_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_h256() {
    basic_features::endpoints::echo_h256(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_i32() {
    basic_features::endpoints::echo_i32(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_i64() {
    basic_features::endpoints::echo_i64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_i8() {
    basic_features::endpoints::echo_i8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_isize() {
    basic_features::endpoints::echo_isize(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_large_boxed_byte_array() {
    basic_features::endpoints::echo_large_boxed_byte_array(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_managed_address() {
    basic_features::endpoints::echo_managed_address(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_managed_async_result_empty() {
    basic_features::endpoints::echo_managed_async_result_empty(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_managed_buffer() {
    basic_features::endpoints::echo_managed_buffer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_managed_vec_of_managed_vec() {
    basic_features::endpoints::echo_managed_vec_of_managed_vec(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_managed_vec_of_token_identifier() {
    basic_features::endpoints::echo_managed_vec_of_token_identifier(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_non_zero_usize() {
    basic_features::endpoints::echo_non_zero_usize(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_nothing() {
    basic_features::endpoints::echo_nothing(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_opt_bool() {
    basic_features::endpoints::echo_opt_bool(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_ser_example_1() {
    basic_features::endpoints::echo_ser_example_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_ser_example_2() {
    basic_features::endpoints::echo_ser_example_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_simple_enum() {
    basic_features::endpoints::echo_simple_enum(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_slice_u8() {
    basic_features::endpoints::echo_slice_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_str() {
    basic_features::endpoints::echo_str(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_str_box() {
    basic_features::endpoints::echo_str_box(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_string() {
    basic_features::endpoints::echo_string(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_tuple_into_multiresult() {
    basic_features::endpoints::echo_tuple_into_multiresult(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_u32() {
    basic_features::endpoints::echo_u32(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_u64() {
    basic_features::endpoints::echo_u64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_u8() {
    basic_features::endpoints::echo_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_usize() {
    basic_features::endpoints::echo_usize(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_varags_big_uint() {
    basic_features::endpoints::echo_varags_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_varags_tuples() {
    basic_features::endpoints::echo_varags_tuples(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_varags_u32() {
    basic_features::endpoints::echo_varags_u32(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_vec_of_managed_buffer() {
    basic_features::endpoints::echo_vec_of_managed_buffer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_vec_u8() {
    basic_features::endpoints::echo_vec_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn finish_simple_enum_variant_1() {
    basic_features::endpoints::finish_simple_enum_variant_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getListMapper() {
    basic_features::endpoints::getListMapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_block_epoch() {
    basic_features::endpoints::get_block_epoch(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_block_nonce() {
    basic_features::endpoints::get_block_nonce(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_block_random_seed() {
    basic_features::endpoints::get_block_random_seed(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_block_round() {
    basic_features::endpoints::get_block_round(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_block_timestamp() {
    basic_features::endpoints::get_block_timestamp(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_caller() {
    basic_features::endpoints::get_caller(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_cumulated_validator_rewards() {
    basic_features::endpoints::get_cumulated_validator_rewards(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_dct_local_roles() {
    basic_features::endpoints::get_dct_local_roles(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_gas_left() {
    basic_features::endpoints::get_gas_left(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_nr_to_clear() {
    basic_features::endpoints::get_nr_to_clear(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_owner_address() {
    basic_features::endpoints::get_owner_address(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_prev_block_epoch() {
    basic_features::endpoints::get_prev_block_epoch(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_prev_block_nonce() {
    basic_features::endpoints::get_prev_block_nonce(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_prev_block_random_seed() {
    basic_features::endpoints::get_prev_block_random_seed(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_prev_block_round() {
    basic_features::endpoints::get_prev_block_round(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_prev_block_timestamp() {
    basic_features::endpoints::get_prev_block_timestamp(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_shard_of_address() {
    basic_features::endpoints::get_shard_of_address(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_state_root_hash_legacy() {
    basic_features::endpoints::get_state_root_hash_legacy(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_tx_hash_legacy() {
    basic_features::endpoints::get_tx_hash_legacy(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn h256_is_zero() {
    basic_features::endpoints::h256_is_zero(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn is_empty_opt_addr() {
    basic_features::endpoints::is_empty_opt_addr(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn is_empty_single_value_mapper() {
    basic_features::endpoints::is_empty_single_value_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn is_smart_contract() {
    basic_features::endpoints::is_smart_contract(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperBack() {
    basic_features::endpoints::listMapperBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperFront() {
    basic_features::endpoints::listMapperFront(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperIterateByHand() {
    basic_features::endpoints::listMapperIterateByHand(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperIterateByIter() {
    basic_features::endpoints::listMapperIterateByIter(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperPopBack() {
    basic_features::endpoints::listMapperPopBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperPopFront() {
    basic_features::endpoints::listMapperPopFront(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperPushAfter() {
    basic_features::endpoints::listMapperPushAfter(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperPushBack() {
    basic_features::endpoints::listMapperPushBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperPushBefore() {
    basic_features::endpoints::listMapperPushBefore(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperPushFront() {
    basic_features::endpoints::listMapperPushFront(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperRemoveNode() {
    basic_features::endpoints::listMapperRemoveNode(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn listMapperRemoveNodeById() {
    basic_features::endpoints::listMapperRemoveNodeById(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_addr() {
    basic_features::endpoints::load_addr(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_big_int() {
    basic_features::endpoints::load_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_big_uint() {
    basic_features::endpoints::load_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_bool() {
    basic_features::endpoints::load_bool(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_i64() {
    basic_features::endpoints::load_i64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_map1() {
    basic_features::endpoints::load_map1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_map2() {
    basic_features::endpoints::load_map2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_map3() {
    basic_features::endpoints::load_map3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_opt_addr() {
    basic_features::endpoints::load_opt_addr(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_ser_1() {
    basic_features::endpoints::load_ser_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_ser_2() {
    basic_features::endpoints::load_ser_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_u64() {
    basic_features::endpoints::load_u64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_usize() {
    basic_features::endpoints::load_usize(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn load_vec_u8() {
    basic_features::endpoints::load_vec_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn log2_big_uint() {
    basic_features::endpoints::log2_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn log2_big_uint_ref() {
    basic_features::endpoints::log2_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn logEventA() {
    basic_features::endpoints::logEventA(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn logEventB() {
    basic_features::endpoints::logEventB(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn logLegacyEventA() {
    basic_features::endpoints::logLegacyEventA(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn logLegacyEventB() {
    basic_features::endpoints::logLegacyEventB(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_address_eq() {
    basic_features::endpoints::managed_address_eq(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_address_from() {
    basic_features::endpoints::managed_address_from(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_address_zero() {
    basic_features::endpoints::managed_address_zero(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_vec_address_push() {
    basic_features::endpoints::managed_vec_address_push(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_vec_biguint_eq() {
    basic_features::endpoints::managed_vec_biguint_eq(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_vec_biguint_push() {
    basic_features::endpoints::managed_vec_biguint_push(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_vec_new() {
    basic_features::endpoints::managed_vec_new(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_contains_key() {
    basic_features::endpoints::map_mapper_contains_key(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_entry_and_modify() {
    basic_features::endpoints::map_mapper_entry_and_modify(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_entry_or_default_update_increment() {
    basic_features::endpoints::map_mapper_entry_or_default_update_increment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_entry_or_insert_default() {
    basic_features::endpoints::map_mapper_entry_or_insert_default(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_entry_or_insert_with_key() {
    basic_features::endpoints::map_mapper_entry_or_insert_with_key(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_get() {
    basic_features::endpoints::map_mapper_get(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_insert() {
    basic_features::endpoints::map_mapper_insert(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_keys() {
    basic_features::endpoints::map_mapper_keys(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_remove() {
    basic_features::endpoints::map_mapper_remove(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_mapper_values() {
    basic_features::endpoints::map_mapper_values(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_my_single_value_mapper() {
    basic_features::endpoints::map_my_single_value_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_clear() {
    basic_features::endpoints::map_storage_mapper_clear(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_contains_key() {
    basic_features::endpoints::map_storage_mapper_contains_key(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_entry_and_modify_increment_or_default() {
    basic_features::endpoints::map_storage_mapper_entry_and_modify_increment_or_default(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_entry_or_default_update() {
    basic_features::endpoints::map_storage_mapper_entry_or_default_update(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_entry_or_default_update_increment() {
    basic_features::endpoints::map_storage_mapper_entry_or_default_update_increment(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_get() {
    basic_features::endpoints::map_storage_mapper_get(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_get_value() {
    basic_features::endpoints::map_storage_mapper_get_value(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_insert_default() {
    basic_features::endpoints::map_storage_mapper_insert_default(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_insert_value() {
    basic_features::endpoints::map_storage_mapper_insert_value(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_remove() {
    basic_features::endpoints::map_storage_mapper_remove(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn map_storage_mapper_view() {
    basic_features::endpoints::map_storage_mapper_view(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_concat_1() {
    basic_features::endpoints::mbuffer_concat_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_concat_2() {
    basic_features::endpoints::mbuffer_concat_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_eq() {
    basic_features::endpoints::mbuffer_eq(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_from_boxed_bytes() {
    basic_features::endpoints::mbuffer_from_boxed_bytes(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_from_slice() {
    basic_features::endpoints::mbuffer_from_slice(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_new() {
    basic_features::endpoints::mbuffer_new(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_overwrite() {
    basic_features::endpoints::mbuffer_overwrite(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_slice_1() {
    basic_features::endpoints::mbuffer_slice_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mbuffer_slice_2() {
    basic_features::endpoints::mbuffer_slice_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_assign_big_int() {
    basic_features::endpoints::mul_assign_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_assign_big_int_ref() {
    basic_features::endpoints::mul_assign_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_assign_big_uint() {
    basic_features::endpoints::mul_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_assign_big_uint_ref() {
    basic_features::endpoints::mul_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_big_int() {
    basic_features::endpoints::mul_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_big_int_ref() {
    basic_features::endpoints::mul_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_big_uint() {
    basic_features::endpoints::mul_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn mul_big_uint_ref() {
    basic_features::endpoints::mul_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn my_single_value_mapper_increment_1() {
    basic_features::endpoints::my_single_value_mapper_increment_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn my_single_value_mapper_increment_2() {
    basic_features::endpoints::my_single_value_mapper_increment_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn my_single_value_mapper_set_if_empty() {
    basic_features::endpoints::my_single_value_mapper_set_if_empty(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn my_single_value_mapper_subtract_with_require() {
    basic_features::endpoints::my_single_value_mapper_subtract_with_require(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn non_zero_usize_iter() {
    basic_features::endpoints::non_zero_usize_iter(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn non_zero_usize_macro() {
    basic_features::endpoints::non_zero_usize_macro(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn only_owner_endpoint() {
    basic_features::endpoints::only_owner_endpoint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn only_owner_legacy() {
    basic_features::endpoints::only_owner_legacy(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn panicWithMessage() {
    basic_features::endpoints::panicWithMessage(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pow_big_int() {
    basic_features::endpoints::pow_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pow_big_int_ref() {
    basic_features::endpoints::pow_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pow_big_uint() {
    basic_features::endpoints::pow_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn pow_big_uint_ref() {
    basic_features::endpoints::pow_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn queue_mapper() {
    basic_features::endpoints::queue_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn queue_mapper_front() {
    basic_features::endpoints::queue_mapper_front(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn queue_mapper_pop_front() {
    basic_features::endpoints::queue_mapper_pop_front(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn queue_mapper_push_back() {
    basic_features::endpoints::queue_mapper_push_back(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn raw_byte_length_single_value_mapper() {
    basic_features::endpoints::raw_byte_length_single_value_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_assign_big_int() {
    basic_features::endpoints::rem_assign_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_assign_big_int_ref() {
    basic_features::endpoints::rem_assign_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_assign_big_uint() {
    basic_features::endpoints::rem_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_assign_big_uint_ref() {
    basic_features::endpoints::rem_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_big_int() {
    basic_features::endpoints::rem_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_big_int_ref() {
    basic_features::endpoints::rem_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_big_uint() {
    basic_features::endpoints::rem_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn rem_big_uint_ref() {
    basic_features::endpoints::rem_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn require_equals() {
    basic_features::endpoints::require_equals(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_echo() {
    basic_features::endpoints::result_echo(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_echo_2() {
    basic_features::endpoints::result_echo_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_echo_3() {
    basic_features::endpoints::result_echo_3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_err_from_bytes_1() {
    basic_features::endpoints::result_err_from_bytes_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_err_from_bytes_2() {
    basic_features::endpoints::result_err_from_bytes_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_err_from_bytes_3() {
    basic_features::endpoints::result_err_from_bytes_3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_err_from_str() {
    basic_features::endpoints::result_err_from_str(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_err_from_string() {
    basic_features::endpoints::result_err_from_string(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn result_ok() {
    basic_features::endpoints::result_ok(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn return_sc_error() {
    basic_features::endpoints::return_sc_error(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn set_mapper() {
    basic_features::endpoints::set_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn set_mapper_contains() {
    basic_features::endpoints::set_mapper_contains(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn set_mapper_insert() {
    basic_features::endpoints::set_mapper_insert(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn set_mapper_remove() {
    basic_features::endpoints::set_mapper_remove(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shl_assign_big_uint() {
    basic_features::endpoints::shl_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shl_assign_big_uint_ref() {
    basic_features::endpoints::shl_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shl_big_uint() {
    basic_features::endpoints::shl_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shl_big_uint_ref() {
    basic_features::endpoints::shl_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shr_assign_big_uint() {
    basic_features::endpoints::shr_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shr_assign_big_uint_ref() {
    basic_features::endpoints::shr_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shr_big_uint() {
    basic_features::endpoints::shr_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn shr_big_uint_ref() {
    basic_features::endpoints::shr_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sqrt_big_uint() {
    basic_features::endpoints::sqrt_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sqrt_big_uint_ref() {
    basic_features::endpoints::sqrt_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_addr() {
    basic_features::endpoints::store_addr(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_big_int() {
    basic_features::endpoints::store_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_big_uint() {
    basic_features::endpoints::store_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_bool() {
    basic_features::endpoints::store_bool(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_i32() {
    basic_features::endpoints::store_i32(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_i64() {
    basic_features::endpoints::store_i64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_map1() {
    basic_features::endpoints::store_map1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_map2() {
    basic_features::endpoints::store_map2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_map3() {
    basic_features::endpoints::store_map3(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_opt_addr() {
    basic_features::endpoints::store_opt_addr(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_reserved_big_uint() {
    basic_features::endpoints::store_reserved_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_reserved_i64() {
    basic_features::endpoints::store_reserved_i64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_reserved_vec_u8() {
    basic_features::endpoints::store_reserved_vec_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_ser_1() {
    basic_features::endpoints::store_ser_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_ser_2() {
    basic_features::endpoints::store_ser_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_u64() {
    basic_features::endpoints::store_u64(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_usize() {
    basic_features::endpoints::store_usize(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn store_vec_u8() {
    basic_features::endpoints::store_vec_u8(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_assign_big_int() {
    basic_features::endpoints::sub_assign_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_assign_big_int_ref() {
    basic_features::endpoints::sub_assign_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_assign_big_uint() {
    basic_features::endpoints::sub_assign_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_assign_big_uint_ref() {
    basic_features::endpoints::sub_assign_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_big_int() {
    basic_features::endpoints::sub_big_int(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_big_int_ref() {
    basic_features::endpoints::sub_big_int_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_big_uint() {
    basic_features::endpoints::sub_big_uint(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn sub_big_uint_ref() {
    basic_features::endpoints::sub_big_uint_ref(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn take_varags_u32() {
    basic_features::endpoints::take_varags_u32(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_attributes_clear() {
    basic_features::endpoints::token_attributes_clear(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_attributes_get_attributes() {
    basic_features::endpoints::token_attributes_get_attributes(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_attributes_get_nonce() {
    basic_features::endpoints::token_attributes_get_nonce(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_attributes_has_attributes() {
    basic_features::endpoints::token_attributes_has_attributes(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_attributes_set() {
    basic_features::endpoints::token_attributes_set(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_attributes_update() {
    basic_features::endpoints::token_attributes_update(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_identifier_moax() {
    basic_features::endpoints::token_identifier_moax(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_identifier_is_valid_1() {
    basic_features::endpoints::token_identifier_is_valid_1(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn token_identifier_is_valid_2() {
    basic_features::endpoints::token_identifier_is_valid_2(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn vec_concat_const() {
    basic_features::endpoints::vec_concat_const(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn vec_mapper() {
    basic_features::endpoints::vec_mapper(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn vec_mapper_get() {
    basic_features::endpoints::vec_mapper_get(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn vec_mapper_len() {
    basic_features::endpoints::vec_mapper_len(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn vec_mapper_push() {
    basic_features::endpoints::vec_mapper_push(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn verify_bls_signature() {
    basic_features::endpoints::verify_bls_signature(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn verify_custom_secp256k1_signature() {
    basic_features::endpoints::verify_custom_secp256k1_signature(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn verify_ed25519_signature() {
    basic_features::endpoints::verify_ed25519_signature(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn verify_secp256k1_signature() {
    basic_features::endpoints::verify_secp256k1_signature(dharitri_wasm_node::vm_api());
}
