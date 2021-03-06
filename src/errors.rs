// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use qp2p::Error as QuicP2pError;
use sn_data_types::{Error as DtError, PublicKey};
pub use sn_messaging::client::Error as ErrorMessage;
use sn_messaging::client::{CmdError, Event, QueryResponse, TransferError};
pub use sn_messaging::Error as MessagingError;
pub use sn_transfers::Error as TransfersError;
use std::io;

use thiserror::Error;

/// Client Errors
#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Asymmetric Key Decryption Failed.
    #[error("Asymmetric key decryption failure")]
    AsymmetricDecipherFailure,
    /// Symmetric Key Decryption Failed.
    #[error("Symmetric key decryption failure")]
    SymmetricDecipherFailure,
    /// Received unexpected data.
    #[error("Unexpected data received")]
    ReceivedUnexpectedData,
    /// Received unexpected event.
    #[error("Unexpected event received")]
    ReceivedUnexpectedEvent,
    /// Config path is not valid.
    #[error("Config path is not a valid UTF-8 string")]
    InvalidConfigPath,

    /// Could not query elder.
    #[error("Problem querying elder")]
    ElderQuery,
    /// Could not connect to elder.
    #[error("Problem connecting to elder")]
    ElderConnection,

    /// Client has not gone trhough qp2p bootstrap process yet
    #[error("Client has failed to bootstrap yet")]
    NotBootstrapped,

    /// Could not connect to sufficient elder to retrieve reliable responses.
    #[error("Problem connecting to sufficient elder")]
    InsufficientElderConnections,

    /// Could not query elder.
    #[error("Problem receiving query via qp2p")]
    ReceivingQuery,

    /// Could not query elder.
    #[error("Failed to obtain a response")]
    NoResponse,
    /// No transfer validation listener .
    #[error("Failed to obtain a response")]
    NoTransferValidationListener,

    /// Unexpected message type receivied while joining.
    #[error("Unexpected message type receivied while joining: {0}")]
    UnexpectedMessageOnJoin(String),
    /// Permission set provided is not a PublicPermissionSet.
    #[error("Expected public permission set")]
    NotPublicPermissions,
    /// Permission set provided is not a PrivatePermissionSet.
    #[error("Expected private permission set")]
    NotPrivatePermissions,
    /// Did not receive an incoming connection listener from qp2p
    #[error("Could not listen on elder connection")]
    NoElderListenerEstablished,
    /// Incorrect user permissions were returned
    #[error(" Incorrect user permissions were returned")]
    IncorrectPermissions,

    /// Unexpcted transfer event received
    #[error("Unexpcted transfer event received {0:?}")]
    UnexpectedTransferEvent(Event),

    /// Unexpcted response received
    #[error("Unexpected response received when querying {0:?}")]
    UnexpectedQueryResponse(QueryResponse),

    /// Unexpected response received
    #[error("Unexpected response received when querying balance history {0:?}")]
    UnexpectedHistoryResponse(QueryResponse),

    /// Unexpected response received
    #[error("Unexpected response received when querying store cost: {0:?}")]
    UnexpectedStoreCostResponse(QueryResponse),

    /// Unexpected response received
    #[error("Unexpected response received when querying replica keys for PublicKey: {0:?}")]
    UnexpectedReplicaKeysResponse(PublicKey),

    /// Transfer actor failed generating a transfer
    #[error("No transfer generated by transfer actor")]
    NoTransferGenerated,
    /// Transfer actor did not find any events to register locally
    #[error("Transfer actor did not find any events to register locally")]
    NoTransferEventsForLocalActor,
    /// Could not determine system home dir
    #[error("Could not determine system home dir")]
    NoHomeDir,
    /// Not in testnet "simulated payout" mode
    #[error("Simulated payouts unavailable without 'simualted-payouts' feature flag at build")]
    NotBuiltWithSimulatedPayouts,

    /// Other sn_data_types errors
    #[error(transparent)]
    NetworkDataError(#[from] DtError),
    /// Transfers errors
    #[error(transparent)]
    Transfer(#[from] TransfersError),

    /// Errors received from the network via sn_messaging
    #[error(transparent)]
    ErrorMessage(#[from] ErrorMessage),

    /// Errors occurred when serialising or deserialising messages
    #[error(transparent)]
    MessagingProtocol(#[from] MessagingError),

    /// self_enryption errors
    #[error(transparent)]
    SelfEncryption(#[from] self_encryption::SelfEncryptionError),

    /// Other sn_data_types errors
    #[error(transparent)]
    ConfigError(#[from] serde_json::Error),

    /// Io error.
    #[error(transparent)]
    IoError(#[from] io::Error),

    /// QuicP2p error.
    #[error(transparent)]
    QuicP2p(#[from] QuicP2pError),

    /// Bincode error
    #[error(transparent)]
    Bincode(#[from] Box<bincode::ErrorKind>),
}

impl From<CmdError> for Error {
    fn from(error: CmdError) -> Self {
        let err = match error {
            CmdError::Data(data_err) => data_err,
            CmdError::Transfer(err) => match err {
                TransferError::TransferValidation(err) => err,
                TransferError::TransferRegistration(err) => err,
            },
            CmdError::Auth(auth_error) => auth_error,
        };
        Error::ErrorMessage(err)
    }
}
