// region:      --- Modules
pub mod alive_check_request;
pub mod alive_check_response;
pub mod diagnostic_message;
pub mod diagnostic_message_ack;
pub mod diagnostic_message_nack;
pub mod entity_status_request;
pub mod entity_status_response;
pub mod generic_nack;
pub mod power_information_request;
pub mod power_information_response;
pub mod routing_activation_request;
pub mod routing_activation_response;
pub mod vehicle_announcement_message;
pub mod vehicle_identification_request;
pub mod vehicle_identification_request_eid;
pub mod vehicle_identification_request_vin;

// -- Flatten

pub use alive_check_request::*;
pub use alive_check_response::*;
pub use diagnostic_message::*;
pub use diagnostic_message_ack::*;
pub use diagnostic_message_nack::*;
pub use entity_status_request::*;
pub use entity_status_response::*;
pub use generic_nack::*;
pub use power_information_request::*;
pub use power_information_response::*;
pub use routing_activation_request::*;
pub use routing_activation_response::*;
pub use vehicle_announcement_message::*;
pub use vehicle_identification_request::*;
pub use vehicle_identification_request_eid::*;
pub use vehicle_identification_request_vin::*;

use crate::definitions::{
    DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
    DOIP_DIAG_COMMON_TARGET_LEN, DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN,
    DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN, DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN,
    DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN,
    DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN, DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
    DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN, DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
};

// -- Public

pub mod action_code;
pub mod activation_code;
pub mod activation_type;
pub mod diagnostic_ack;
pub mod diagnostic_nack;
pub mod nack_code;
pub mod node_type;
pub mod power_mode;
pub mod sync_status;

// endregion:      --- Modules

/// Implemented across `DoIP` Payload Types for consistent encoding and decoding of buffers.
///
/// `DoipPayload` is implemented for all the `DoIP` Payload Types for the
/// purpose of consistent encoding and decoding as well as identification within
/// a buffer.
#[cfg(not(feature = "std"))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DoipPayload<const N: usize> {
    /// `GenericNack` variant to hold `GenericNack` struct
    GenericNack(GenericNack),

    /// `VehicleIdentificationRequest` variant to hold `VehicleIdentificationRequest` struct
    VehicleIdentificationRequest(VehicleIdentificationRequest),

    /// `VehicleIdentificationRequestEid` variant to hold `VehicleIdentificationRequestEid` struct
    VehicleIdentificationRequestEid(VehicleIdentificationRequestEid),

    /// `VehicleIdentificationRequestVin` variant to hold `VehicleIdentificationRequestVin` struct
    VehicleIdentificationRequestVin(VehicleIdentificationRequestVin),

    /// `VehicleAnnouncementMessage` variant to hold `VehicleAnnouncementMessage` struct
    VehicleAnnouncementMessage(VehicleAnnouncementMessage),

    /// `RoutingActivationRequest` variant to hold `RoutingActivationRequest` struct
    RoutingActivationRequest(RoutingActivationRequest),

    /// `RoutingActivationResponse` variant to hold `RoutingActivationResponse` struct
    RoutingActivationResponse(RoutingActivationResponse),

    /// `AliveCheckRequest` variant to hold `AliveCheckRequest` struct
    AliveCheckRequest(AliveCheckRequest),

    /// `AliveCheckResponse` variant to hold `AliveCheckResponse` struct
    AliveCheckResponse(AliveCheckResponse),

    /// `EntityStatusRequest` variant to hold `EntityStatusRequest` struct
    EntityStatusRequest(EntityStatusRequest),

    /// `EntityStatusResponse` variant to hold `EntityStatusResponse` struct
    EntityStatusResponse(EntityStatusResponse),

    /// `PowerInformationRequest` variant to hold `PowerInformationRequest` struct
    PowerInformationRequest(PowerInformationRequest),

    /// `PowerInformationResponse` variant to hold `PowerInformationResponse` struct
    PowerInformationResponse(PowerInformationResponse),

    /// `DiagnosticMessage` variant to hold `DiagnosticMessage` struct
    DiagnosticMessage(DiagnosticMessage<N>),

    /// `DiagnosticMessageAck` variant to hold `DiagnosticMessageAck` struct
    DiagnosticMessageAck(DiagnosticMessageAck),

    /// `DiagnosticMessageNack` variant to hold `DiagnosticMessageNack` struct
    DiagnosticMessageNack(DiagnosticMessageNack),
}

#[cfg(not(feature = "std"))]
impl<const N: usize> From<DoipPayload<N>> for [u8; N] {
    fn from(value: DoipPayload<N>) -> Self {
        fn copy_into_buffer<const N: usize, const M: usize>(payload: [u8; M]) -> [u8; N] {
            let mut buffer = [0u8; N];
            buffer[..M].copy_from_slice(&payload);
            buffer
        }

        match value {
            DoipPayload::GenericNack(payload) => copy_into_buffer::<N, 1>(payload.into()),
            DoipPayload::VehicleIdentificationRequest(payload) => {
                copy_into_buffer::<N, 0>(payload.into())
            }
            DoipPayload::VehicleIdentificationRequestEid(payload) => {
                copy_into_buffer::<N, DOIP_COMMON_EID_LEN>(payload.into())
            }
            DoipPayload::VehicleIdentificationRequestVin(payload) => {
                copy_into_buffer::<N, DOIP_COMMON_VIN_LEN>(payload.into())
            }
            DoipPayload::VehicleAnnouncementMessage(payload) => copy_into_buffer::<
                N,
                {
                    DOIP_COMMON_VIN_LEN
                        + DOIP_DIAG_COMMON_SOURCE_LEN
                        + DOIP_COMMON_EID_LEN
                        + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
                        + 2
                },
            >(payload.into()),
            DoipPayload::RoutingActivationRequest(payload) => copy_into_buffer::<
                N,
                { DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN + 1 + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN },
            >(payload.into()),
            DoipPayload::RoutingActivationResponse(payload) => copy_into_buffer::<
                N,
                {
                    DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN
                        + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN
                        + 1
                        + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN
                },
            >(payload.into()),
            DoipPayload::AliveCheckRequest(payload) => copy_into_buffer::<N, 0>(payload.into()),
            DoipPayload::AliveCheckResponse(payload) => {
                copy_into_buffer::<N, DOIP_DIAG_COMMON_SOURCE_LEN>(payload.into())
            }
            DoipPayload::EntityStatusRequest(payload) => copy_into_buffer::<N, 0>(payload.into()),
            DoipPayload::EntityStatusResponse(payload) => copy_into_buffer::<
                N,
                {
                    1 + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
                        + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
                        + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN
                },
            >(payload.into()),
            DoipPayload::PowerInformationRequest(payload) => {
                copy_into_buffer::<N, 0>(payload.into())
            }
            DoipPayload::PowerInformationResponse(payload) => {
                copy_into_buffer::<N, 1>(payload.into())
            }
            DoipPayload::DiagnosticMessage(payload) => copy_into_buffer::<N, N>(payload.into()),
            DoipPayload::DiagnosticMessageAck(payload) => copy_into_buffer::<
                N,
                { DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1 },
            >(payload.into()),
            DoipPayload::DiagnosticMessageNack(payload) => copy_into_buffer::<
                N,
                { DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1 },
            >(payload.into()),
        }
    }
}

/// Implemented across `DoIP` Payload Types for consistent encoding and decoding of buffers.
///
/// `DoipPayload` is implemented for all the `DoIP` Payload Types for the
/// purpose of consistent encoding and decoding as well as identification within
/// a buffer.
#[cfg(feature = "std")]
#[derive(Debug, PartialEq, Clone)]
pub enum DoipPayload {
    /// `GenericNack` variant to hold `GenericNack` struct
    GenericNack(GenericNack),

    /// `VehicleIdentificationRequest` variant to hold `VehicleIdentificationRequest` struct
    VehicleIdentificationRequest(VehicleIdentificationRequest),

    /// `VehicleIdentificationRequestEid` variant to hold `VehicleIdentificationRequestEid` struct
    VehicleIdentificationRequestEid(VehicleIdentificationRequestEid),

    /// `VehicleIdentificationRequestVin` variant to hold `VehicleIdentificationRequestVin` struct
    VehicleIdentificationRequestVin(VehicleIdentificationRequestVin),

    /// `VehicleAnnouncementMessage` variant to hold `VehicleAnnouncementMessage` struct
    VehicleAnnouncementMessage(VehicleAnnouncementMessage),

    /// `RoutingActivationRequest` variant to hold `RoutingActivationRequest` struct
    RoutingActivationRequest(RoutingActivationRequest),

    /// `RoutingActivationResponse` variant to hold `RoutingActivationResponse` struct
    RoutingActivationResponse(RoutingActivationResponse),

    /// `AliveCheckRequest` variant to hold `AliveCheckRequest` struct
    AliveCheckRequest(AliveCheckRequest),

    /// `AliveCheckResponse` variant to hold `AliveCheckResponse` struct
    AliveCheckResponse(AliveCheckResponse),

    /// `EntityStatusRequest` variant to hold `EntityStatusRequest` struct
    EntityStatusRequest(EntityStatusRequest),

    /// `EntityStatusResponse` variant to hold `EntityStatusResponse` struct
    EntityStatusResponse(EntityStatusResponse),

    /// `PowerInformationRequest` variant to hold `PowerInformationRequest` struct
    PowerInformationRequest(PowerInformationRequest),

    /// `PowerInformationResponse` variant to hold `PowerInformationResponse` struct
    PowerInformationResponse(PowerInformationResponse),

    /// `DiagnosticMessage` variant to hold `DiagnosticMessage` struct
    DiagnosticMessage(DiagnosticMessage),

    /// `DiagnosticMessageAck` variant to hold `DiagnosticMessageAck` struct
    DiagnosticMessageAck(DiagnosticMessageAck),

    /// `DiagnosticMessageNack` variant to hold `DiagnosticMessageNack` struct
    DiagnosticMessageNack(DiagnosticMessageNack),
}

#[cfg(feature = "std")]
impl From<DoipPayload> for Vec<u8> {
    fn from(value: DoipPayload) -> Self {
        match value {
            DoipPayload::GenericNack(payload) => {
                let payload: [u8; 1] = payload.into();
                payload.to_vec()
            }
            DoipPayload::VehicleIdentificationRequest(payload) => {
                let payload: [u8; 0] = payload.into();
                payload.to_vec()
            }
            DoipPayload::VehicleIdentificationRequestEid(payload) => {
                let payload: [u8; DOIP_COMMON_EID_LEN] = payload.into();
                payload.to_vec()
            }
            DoipPayload::VehicleIdentificationRequestVin(payload) => {
                let payload: [u8; DOIP_COMMON_VIN_LEN] = payload.into();
                payload.to_vec()
            }
            DoipPayload::VehicleAnnouncementMessage(payload) => {
                if payload.vin_gid_sync.is_some() {
                    let payload: [u8; DOIP_COMMON_VIN_LEN
                        + DOIP_DIAG_COMMON_SOURCE_LEN
                        + DOIP_COMMON_EID_LEN
                        + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
                        + 2] = payload.into();
                    payload.to_vec()
                } else {
                    let payload: [u8; DOIP_COMMON_VIN_LEN
                        + DOIP_DIAG_COMMON_SOURCE_LEN
                        + DOIP_COMMON_EID_LEN
                        + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
                        + 1] = payload.into();
                    payload.to_vec()
                }
            }
            DoipPayload::RoutingActivationRequest(payload) => {
                let payload: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN
                    + 1
                    + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN] = payload.into();
                payload.to_vec()
            }
            DoipPayload::RoutingActivationResponse(payload) => {
                let payload: [u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN
                    + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN
                    + 1
                    + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN] = payload.into();
                payload.to_vec()
            }
            DoipPayload::AliveCheckRequest(payload) => {
                let payload: [u8; 0] = payload.into();
                payload.to_vec()
            }
            DoipPayload::AliveCheckResponse(payload) => {
                let payload: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] = payload.into();
                payload.to_vec()
            }
            DoipPayload::EntityStatusRequest(payload) => {
                let payload: [u8; 0] = payload.into();
                payload.to_vec()
            }
            DoipPayload::EntityStatusResponse(payload) => {
                let payload: [u8; 1
                    + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
                    + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
                    + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN] = payload.into();
                payload.to_vec()
            }
            DoipPayload::PowerInformationRequest(payload) => {
                let payload: [u8; 0] = payload.into();
                payload.to_vec()
            }
            DoipPayload::PowerInformationResponse(payload) => {
                let payload: [u8; 1] = payload.into();
                payload.to_vec()
            }
            DoipPayload::DiagnosticMessage(payload) => payload.into(),
            DoipPayload::DiagnosticMessageAck(payload) => {
                let payload: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] =
                    payload.into();
                payload.to_vec()
            }
            DoipPayload::DiagnosticMessageNack(payload) => {
                let payload: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] =
                    payload.into();
                payload.to_vec()
            }
        }
    }
}
