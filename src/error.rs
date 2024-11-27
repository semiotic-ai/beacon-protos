// Copyright 2024-, Semiotic AI, Inc.
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

/// Custom error variants for Verifiable Extraction protobuffer types.
#[derive(Error, Debug)]
pub enum BeaconProtosError {
    /// Missing attestation data.
    #[error("Null attestation data")]
    AttestationDataMissing,

    /// Error converting protobuffer to block type.
    #[error("Block conversion error")]
    BlockConversionError,

    /// Block response missing block.
    #[error("Null block field in block response")]
    BlockMissingInResponse,

    /// [BLS signature](https://en.wikipedia.org/wiki/BLS_digital_signature) error.
    #[error("BLS error: {0}")]
    Bls(String),

    /// Missing BLS to Execution Change
    #[error("Null BlsToExecutionChange")]
    BlsToExecutionChangeMissing,

    /// Checkpoint missing.
    #[error("Null checkpoint")]
    CheckpointMissing,

    /// [prost] library decode error.
    #[error("Error in decoding block: {0}")]
    DecodeError(#[from] prost::DecodeError),

    /// Missing deposit data.
    #[error("Null deposit data")]
    DepositDataMissing,

    /// Missing execution payload.
    #[error("Null execution payload")]
    ExecutionPayloadMissing,

    /// Graffiti invalid when decoding block.
    #[error("GraffitiInvalid")]
    GraffitiInvalid,

    /// Missing indexed attestation data.
    #[error("Null indexed attestation data")]
    IndexedAttestationDataMissing,

    /// Invalid KZG commitment.
    #[error("KzgCommitmentInvalid")]
    KzgCommitmentInvalid,

    /// Missing signed Beacon block header message.
    #[error("Null SignedBeaconBlockHeader Message")]
    SignedBeaconBlockHeaderMessageMissing,

    /// Missing signer
    #[error("Null signer")]
    SignerMissing,

    /// SSZ Types error.
    #[error("SSZ Types error: {0}")]
    SszTypesError(String),

    /// Missing voluntary exit.
    #[error("Null voluntary exit")]
    VoluntaryExitMissing,
}
