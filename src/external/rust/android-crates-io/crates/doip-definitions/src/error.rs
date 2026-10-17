/// Custom Result type which allows for easier typing of errors across the API.
pub type Result<T> = core::result::Result<T, Error>;

/// Custom Error enum for deriving error types across the application and API.
///
/// Expand to fit new dependencies using `#[from]` and implement custom error types
/// with context.
#[derive(Debug)]
pub enum Error {
    /// When utilising the builder pattern the payload is not present
    PayloadNotBuilt,
    /// When utilising the builder pattern the header is not present
    HeaderNotBuilt,

    /// Errors when accessing a range out of the slices scope.
    OutOfBounds {
        /// Source struct
        source: &'static str,

        /// Destination variable
        variable: &'static str,
    },

    /// When a slice is too small to be written to
    SliceTooSmall,

    /// Invalid `ProtocolVersion`
    InvalidProtocolVersion {
        /// Value
        value: u8,
    },

    /// Invalid `NackCode`
    InvalidNackCode {
        /// Value
        value: u8,
    },

    /// Invalid `ActionCode`
    InvalidActionCode {
        /// Value
        value: u8,
    },

    /// Invalid `SyncStatus`
    InvalidSyncStatus {
        /// Value
        value: u8,
    },

    /// Invalid `ActivationCode`
    InvalidActivationCode {
        /// Value
        value: u8,
    },

    /// Invalid `ActivationType`
    InvalidActivationType {
        /// Value
        value: u8,
    },

    /// Invalid `NodeType`
    InvalidNodeType {
        /// Value
        value: u8,
    },

    /// Invalid `PowerMode`
    InvalidPowerMode {
        /// Value
        value: u8,
    },

    /// Invalid `DiagnosticNackCode`
    InvalidDiagnosticNackCode {
        /// Value
        value: u8,
    },

    /// Invalid `DiagnosticAckCode`
    InvalidDiagnosticAckCode {
        /// Value
        value: u8,
    },

    /// Invalid `PayloadType`
    InvalidPayloadType {
        /// Value
        value: [u8; 2],
    },

    /// When the destination buffer is too small
    BufferTooSmall {
        /// Size of the buffer
        size: usize,
    },

    /// Standard library IO errors
    #[allow(clippy::enum_variant_names)]
    SliceError(core::array::TryFromSliceError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<core::array::TryFromSliceError> for Error {
    fn from(err: core::array::TryFromSliceError) -> Self {
        Error::SliceError(err)
    }
}
