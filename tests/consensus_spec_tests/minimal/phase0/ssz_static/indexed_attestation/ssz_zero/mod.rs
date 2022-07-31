// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

#![cfg(not(feature = "bls"))]
use crate::spec_test_runners::ssz_static::IndexedAttestationTestCase;
use ethereum_consensus::phase0::minimal as spec;
use ssz_rs::prelude::*;

#[test]
fn test_case_0() {
    let test_case = IndexedAttestationTestCase::from(
        "consensus-spec-tests/tests/minimal/phase0/ssz_static/IndexedAttestation/ssz_zero/case_0",
    );

    test_case.execute(|encoding| {
        let mut data: spec::IndexedAttestation = ssz_rs::deserialize(encoding).unwrap();
        let serialized = ssz_rs::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}
