/*
    Multisig ed25519

    Copyright 2018 by Kzen Networks

    This file is part of Multisig Schnorr library
    (https://github.com/KZen-networks/multisig-schnorr)

    Multisig Schnorr is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.

    @license GPL-3.0+ <https://github.com/KZen-networks/multi-party-ed25519/blob/master/LICENSE>
*/

extern crate curv;

extern crate hex;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;
extern crate sha2;

#[cfg(test)]
extern crate ed25519_dalek;
#[cfg(test)]
extern crate itertools;
#[cfg(test)]
extern crate rand_xoshiro;

pub mod protocols;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum Error {
    InvalidKey,
    InvalidSS,
    InvalidCom,
    InvalidSig,
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

impl std::error::Error for Error {}
