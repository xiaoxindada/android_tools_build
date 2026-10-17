use pyo3::prelude::*;

#[allow(clippy::wildcard_imports)]
use crate::{
    definitions::*,
    header::{DoipHeader, PayloadType, ProtocolVersion},
};

#[pymodule]
fn doip_definitions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_definitions(m)?;
    register_header(m)?;

    Ok(())
}

fn register_header(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "header")?;

    child_module.add_class::<DoipHeader>()?;
    child_module.add_class::<PayloadType>()?;
    child_module.add_class::<ProtocolVersion>()?;

    parent_module.add_submodule(&child_module)?;

    Ok(())
}

#[allow(clippy::too_many_lines)]
fn register_definitions(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "definitions")?;

    child_module.add_class::<DoipHeader>()?;

    // DoIP Ports
    child_module.add("DOIP_PORT", 13400)?;
    child_module.add("DOIP_PORT", DOIP_PORT)?;
    child_module.add("DOIP_TLS_PORT", DOIP_TLS_PORT)?;

    // Payload Types
    child_module.add("DOIP_GENERIC_NACK", DOIP_GENERIC_NACK)?;
    child_module.add(
        "DOIP_VEHICLE_IDENTIFICATION_REQ",
        DOIP_VEHICLE_IDENTIFICATION_REQ,
    )?;
    child_module.add(
        "DOIP_VEHICLE_IDENTIFICATION_REQ_EID",
        DOIP_VEHICLE_IDENTIFICATION_REQ_EID,
    )?;
    child_module.add(
        "DOIP_VEHICLE_IDENTIFICATION_REQ_VIN",
        DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE",
        DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQUEST",
        DOIP_ROUTING_ACTIVATION_REQUEST,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RESPONSE",
        DOIP_ROUTING_ACTIVATION_RESPONSE,
    )?;
    child_module.add("DOIP_ALIVE_CHECK_REQUEST", DOIP_ALIVE_CHECK_REQUEST)?;
    child_module.add("DOIP_ALIVE_CHECK_RESPONSE", DOIP_ALIVE_CHECK_RESPONSE)?;
    child_module.add("DOIP_ENTITY_STATUS_REQUEST", DOIP_ENTITY_STATUS_REQUEST)?;
    child_module.add("DOIP_ENTITY_STATUS_RESPONSE", DOIP_ENTITY_STATUS_RESPONSE)?;
    child_module.add(
        "DOIP_POWER_INFORMATION_REQUEST",
        DOIP_POWER_INFORMATION_REQUEST,
    )?;
    child_module.add(
        "DOIP_POWER_INFORMATION_RESPONSE",
        DOIP_POWER_INFORMATION_RESPONSE,
    )?;
    child_module.add("DOIP_DIAGNOSTIC_MESSAGE", DOIP_DIAGNOSTIC_MESSAGE)?;
    child_module.add("DOIP_DIAGNOSTIC_MESSAGE_ACK", DOIP_DIAGNOSTIC_MESSAGE_ACK)?;
    child_module.add("DOIP_DIAGNOSTIC_MESSAGE_NACK", DOIP_DIAGNOSTIC_MESSAGE_NACK)?;

    // DoIP Header
    child_module.add("DOIP_VERSION_OFFSET", DOIP_VERSION_OFFSET)?;
    child_module.add("DOIP_VERSION_LEN", DOIP_VERSION_LEN)?;
    child_module.add("DOIP_INV_VERSION_OFFSET", DOIP_INV_VERSION_OFFSET)?;
    child_module.add("DOIP_INV_VERSION_LEN", DOIP_INV_VERSION_LEN)?;
    child_module.add("DOIP_TYPE_OFFSET", DOIP_TYPE_OFFSET)?;
    child_module.add("DOIP_TYPE_LEN", DOIP_TYPE_LEN)?;
    child_module.add("DOIP_LENGTH_OFFSET", DOIP_LENGTH_OFFSET)?;
    child_module.add("DOIP_LENGTH_LEN", DOIP_LENGTH_LEN)?;
    child_module.add("DOIP_HEADER_LEN", DOIP_HEADER_LEN)?;

    // DoIP Payload Version
    child_module.add("RESERVED_VER", RESERVED_VER)?;
    child_module.add("ISO13400_2010", ISO13400_2010)?;
    child_module.add("ISO13400_2012", ISO13400_2012)?;
    child_module.add("ISO13400_2019", ISO13400_2019)?;
    child_module.add("ISO13400_2019_AMD1", ISO13400_2019_AMD1)?;
    child_module.add("DEFAULT_VALUE", DEFAULT_VALUE)?;

    // Generic NACK
    child_module.add("DOIP_GENERIC_NACK_OFFSET", DOIP_GENERIC_NACK_OFFSET)?;
    child_module.add("DOIP_GENERIC_NACK_LEN", DOIP_GENERIC_NACK_LEN)?;

    // Common
    child_module.add("DOIP_COMMON_VIN_LEN", DOIP_COMMON_VIN_LEN)?;
    child_module.add("DOIP_COMMON_EID_LEN", DOIP_COMMON_EID_LEN)?;

    // Vehicle Identification Request
    child_module.add(
        "DOIP_VEHICLE_IDENTIFICATION_EID_OFFSET",
        DOIP_VEHICLE_IDENTIFICATION_EID_OFFSET,
    )?;
    child_module.add(
        "DOIP_VEHICLE_IDENTIFICATION_VIN_OFFSET",
        DOIP_VEHICLE_IDENTIFICATION_VIN_OFFSET,
    )?;

    // Routing Activation Request
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_SRC_OFFSET",
        DOIP_ROUTING_ACTIVATION_REQ_SRC_OFFSET,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN",
        DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET",
        DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V1",
        DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V1,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2",
        DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V1",
        DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V1,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V2",
        DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V2,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN",
        DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_OEM_OFFSET_V1",
        DOIP_ROUTING_ACTIVATION_REQ_OEM_OFFSET_V1,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_OEM_OFFSET_V2",
        DOIP_ROUTING_ACTIVATION_REQ_OEM_OFFSET_V2,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_OEM_LEN",
        DOIP_ROUTING_ACTIVATION_REQ_OEM_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_REQ_LEN",
        DOIP_ROUTING_ACTIVATION_REQ_LEN,
    )?;

    // Routing Activation Response
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_TESTER_OFFSET",
        DOIP_ROUTING_ACTIVATION_RES_TESTER_OFFSET,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN",
        DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_ENTITY_OFFSET",
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_OFFSET,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN",
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_CODE_OFFSET",
        DOIP_ROUTING_ACTIVATION_RES_CODE_OFFSET,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_CODE_LEN",
        DOIP_ROUTING_ACTIVATION_RES_CODE_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_ISO_OFFSET",
        DOIP_ROUTING_ACTIVATION_RES_ISO_OFFSET,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_ISO_LEN",
        DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_OEM_OFFSET",
        DOIP_ROUTING_ACTIVATION_RES_OEM_OFFSET,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_OEM_LEN",
        DOIP_ROUTING_ACTIVATION_RES_OEM_LEN,
    )?;
    child_module.add(
        "DOIP_ROUTING_ACTIVATION_RES_LEN",
        DOIP_ROUTING_ACTIVATION_RES_LEN,
    )?;

    // Vehicle Announcement Message
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_VIN_OFFSET",
        DOIP_VEHICLE_ANNOUNCEMENT_VIN_OFFSET,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_OFFSET",
        DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_OFFSET,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_LEN",
        DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_LEN,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_EID_OFFSET",
        DOIP_VEHICLE_ANNOUNCEMENT_EID_OFFSET,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_GID_OFFSET",
        DOIP_VEHICLE_ANNOUNCEMENT_GID_OFFSET,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN",
        DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_ACTION_OFFSET",
        DOIP_VEHICLE_ANNOUNCEMENT_ACTION_OFFSET,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN",
        DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_SYNC_OFFSET",
        DOIP_VEHICLE_ANNOUNCEMENT_SYNC_OFFSET,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_SYNC_LEN",
        DOIP_VEHICLE_ANNOUNCEMENT_SYNC_LEN,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT",
        DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT,
    )?;
    child_module.add(
        "DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG",
        DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG,
    )?;

    // Alive Check Response
    child_module.add(
        "DOIP_ALIVE_CHECK_RESPONSE_SOURCE_OFFSET",
        DOIP_ALIVE_CHECK_RESPONSE_SOURCE_OFFSET,
    )?;
    child_module.add(
        "DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN",
        DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN,
    )?;

    // Entity Status Response
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_NODE_OFFSET",
        DOIP_ENTITY_STATUS_RESPONSE_NODE_OFFSET,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN",
        DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_MCTS_OFFSET",
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_OFFSET,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN",
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_NCTS_OFFSET",
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_OFFSET,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN",
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_MDS_OFFSET",
        DOIP_ENTITY_STATUS_RESPONSE_MDS_OFFSET,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN",
        DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN,
    )?;
    child_module.add(
        "DOIP_ENTITY_STATUS_RESPONSE_LEN",
        DOIP_ENTITY_STATUS_RESPONSE_LEN,
    )?;

    // Power Mode Response
    child_module.add("DOIP_POWER_MODE_OFFSET", DOIP_POWER_MODE_OFFSET)?;
    child_module.add("DOIP_POWER_MODE_LEN", DOIP_POWER_MODE_LEN)?;

    // Diagnostic Message
    child_module.add(
        "DOIP_DIAG_COMMON_SOURCE_OFFSET",
        DOIP_DIAG_COMMON_SOURCE_OFFSET,
    )?;
    child_module.add("DOIP_DIAG_COMMON_SOURCE_LEN", DOIP_DIAG_COMMON_SOURCE_LEN)?;
    child_module.add(
        "DOIP_DIAG_COMMON_TARGET_OFFSET",
        DOIP_DIAG_COMMON_TARGET_OFFSET,
    )?;
    child_module.add("DOIP_DIAG_COMMON_TARGET_LEN", DOIP_DIAG_COMMON_TARGET_LEN)?;
    child_module.add(
        "DOIP_DIAG_MESSAGE_DATA_OFFSET",
        DOIP_DIAG_MESSAGE_DATA_OFFSET,
    )?;

    // Diagnostic Message ACK
    child_module.add(
        "DOIP_DIAG_MESSAGE_ACK_CODE_OFFSET",
        DOIP_DIAG_MESSAGE_ACK_CODE_OFFSET,
    )?;
    child_module.add(
        "DOIP_DIAG_MESSAGE_ACK_CODE_LEN",
        DOIP_DIAG_MESSAGE_ACK_CODE_LEN,
    )?;
    child_module.add(
        "DOIP_DIAG_MESSAGE_ACK_PREVIOUS_OFFSET",
        DOIP_DIAG_MESSAGE_ACK_PREVIOUS_OFFSET,
    )?;

    // Diagnostic Message NACK
    child_module.add(
        "DOIP_DIAG_MESSAGE_NACK_CODE_OFFSET",
        DOIP_DIAG_MESSAGE_NACK_CODE_OFFSET,
    )?;
    child_module.add(
        "DOIP_DIAG_MESSAGE_NACK_CODE_LEN",
        DOIP_DIAG_MESSAGE_NACK_CODE_LEN,
    )?;
    child_module.add(
        "DOIP_DIAG_MESSAGE_NACK_PREVIOUS_OFFSET",
        DOIP_DIAG_MESSAGE_NACK_PREVIOUS_OFFSET,
    )?;

    parent_module.add_submodule(&child_module)?;

    Ok(())
}
