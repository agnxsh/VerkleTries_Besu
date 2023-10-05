/*
 * Copyright Besu Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on
 * an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the
 * specific language governing permissions and limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */
use ark_ff::PrimeField;
use banderwagon::{Fr, multi_scalar_mul};
use ipa_multipoint::crs::CRS;
use verkle_spec::*;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jbyteArray;

// Copied from rust-verkle: https://github.com/crate-crypto/rust-verkle/blob/581200474327f5d12629ac2e1691eff91f944cec/verkle-trie/src/constants.rs#L12
const PEDERSEN_SEED: &'static [u8] = b"eth_verkle_oct_2021";

/// Pedersen hash receives an address and a trie index and returns a hash calculated this way:
/// H(constant || address_low || address_high || trie_index_low || trie_index_high)
/// where constant = 2 + 256*64
/// address_low = lower 16 bytes of the address interpreted as a little endian integer
/// address_high = higher 16 bytes of the address interpreted as a little endian integer
/// trie_index_low = lower 16 bytes of the trie index
/// trie_index_high = higher 16 bytes of the trie index
/// The result is a 256 bit hash
/// This is ported from rust-verkle/verkle-specs
#[no_mangle]
pub extern "system" fn Java_org_hyperledger_besu_nativelib_ipamultipoint_LibIpaMultipoint_pedersenHash(
    env: JNIEnv,
    _class: JClass,
    input: jbyteArray,
) -> jbyteArray {

    let input = env.convert_byte_array(input).unwrap();

    let mut address32 = [0u8; 32];

    address32.copy_from_slice(&input[0..32]);

    let mut trie_index= [0u8; 32];

    trie_index.copy_from_slice(&input[32..64]);
    trie_index.reverse(); // reverse for little endian per specs

    let base_hash = hash_addr_int(&address32, &trie_index);

    let result = base_hash.as_fixed_bytes();
    let output = env.byte_array_from_slice(result).unwrap();
    output
}

// Helper function to hash an address and an integer taken from rust-verkle/verkle-specs.
pub(crate) fn hash_addr_int(addr: &[u8; 32], integer: &[u8; 32]) -> H256 {

    let address_bytes = addr;

    let mut integer_bytes = integer;
    let mut hash_input = [0u8; 64];
    let (first_half, second_half) = hash_input.split_at_mut(32);

    // Copy address and index into slice, then hash it
    first_half.copy_from_slice(address_bytes);
    second_half.copy_from_slice(integer_bytes);

    hash64(hash_input)
}