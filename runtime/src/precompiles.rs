// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of DEXCHAIN.
//
// Copyright (c) 2021 Dexio Technologies.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use evm::{Context, ExitError, ExitSucceed};
use pallet_evm::{AddressMapping, Precompile, PrecompileSet};
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_dispatch::Dispatch;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use randomness_precompiles::Randomness;
use sp_core::H160;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

/// The PrecompileSet installed in the runtime.
/// We include the nine Istanbul precompiles
/// (https://github.com/ethereum/go-ethereum/blob/3c46f557/core/vm/contracts.go#L69)
/// as well as a special precompile for dispatching Substrate extrinsics
#[derive(Debug, Clone, Copy)]
pub struct DexioPrecompiles<R>(PhantomData<R>);

impl<R> DexioPrecompiles<R>
where
    R: pallet_evm::Config,
{
    /// Return all addresses that contain precompiles. This can be used to populate dummy code
    /// under the precompile.
    pub fn used_addresses() -> impl Iterator<Item = H160> {
        sp_std::vec![1, 2, 3, 4, 5, 6, 7, 8, 1024, 1025, 1026, 2048]
            .into_iter()
            .map(|x| hash(x))
    }
}

/// The following distribution has been decided for the precompiles
/// 0-1023: Ethereum Mainnet Precompiles
/// 1024-2047 Precompiles that are not in Ethereum Mainnet but are neither Dexchain specific
/// 2048-4095 Moonbeam specific precompiles
impl<R> PrecompileSet for DexioPrecompiles<R>
where
    // TODO remove this first trait bound once https://github.com/paritytech/frontier/pull/472 lands
    R: pallet_evm::Config,
    Dispatch<R>: Precompile,
    Randomness<R>: Precompile,
{
    fn execute(
        address: H160,
        input: &[u8],
        target_gas: Option<u64>,
        context: &Context,
    ) -> Option<Result<(ExitSucceed, Vec<u8>, u64), ExitError>> {
        match address {
            // Ethereum precompiles :
            a if a == hash(1) => Some(ECRecover::execute(input, target_gas, context)),
            a if a == hash(2) => Some(Sha256::execute(input, target_gas, context)),
            a if a == hash(3) => Some(Ripemd160::execute(input, target_gas, context)),
            a if a == hash(4) => Some(Identity::execute(input, target_gas, context)),
            a if a == hash(5) => Some(Modexp::execute(input, target_gas, context)),
            a if a == hash(6) => Some(Bn128Add::execute(input, target_gas, context)),
            a if a == hash(7) => Some(Bn128Mul::execute(input, target_gas, context)),
            a if a == hash(8) => Some(Bn128Pairing::execute(input, target_gas, context)),
            // Non-Dexchain specific nor Ethereum precompiles :
            a if a == hash(1024) => Some(Sha3FIPS256::execute(input, target_gas, context)),
            a if a == hash(1025) => Some(Dispatch::<R>::execute(input, target_gas, context)),
            a if a == hash(1026) => Some(ECRecoverPublicKey::execute(input, target_gas, context)),
            // dexchain specific precompiles :
            a if a == hash(2048) => Some(Randomness::<R>::execute(input, target_gas, context)),
            _ => None,
        }
    }
}

fn hash(a: u64) -> H160 {
    H160::from_low_u64_be(a)
}
