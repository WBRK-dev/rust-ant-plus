pub struct Constants {}

impl Constants {
    pub const MESSAGE_RF: u8 = 0x01;

    pub const MESSAGE_TX_SYNC: u8 = 0xA4;
    pub const DEFAULT_NETWORK_NUMBER: u8 = 0x00;

    // Configuration messages
    pub const MESSAGE_CHANNEL_UNASSIGN: u8 = 0x41;
    pub const MESSAGE_CHANNEL_ASSIGN: u8 = 0x42;
    pub const MESSAGE_CHANNEL_ID: u8 = 0x51;
    pub const MESSAGE_CHANNEL_PERIOD: u8 = 0x43;
    pub const MESSAGE_CHANNEL_SEARCH_TIMEOUT: u8 = 0x44;
    pub const MESSAGE_CHANNEL_FREQUENCY: u8 = 0x45;
    pub const MESSAGE_CHANNEL_TX_POWER: u8 = 0x60;
    pub const MESSAGE_NETWORK_KEY: u8 = 0x46;
    pub const MESSAGE_TX_POWER: u8 = 0x47;
    pub const MESSAGE_PROXIMITY_SEARCH: u8 = 0x71;
    pub const MESSAGE_ENABLE_RX_EXT: u8 = 0x66;
    pub const MESSAGE_LIB_CONFIG: u8 = 0x6E;
    pub const MESSAGE_CHANNEL_OPEN_RX_SCAN: u8 = 0x5B;

    // Notification messages
    pub const MESSAGE_STARTUP: u8 = 0x6F;

    // Control messages
    pub const MESSAGE_SYSTEM_RESET: u8 = 0x4A;
    pub const MESSAGE_CHANNEL_OPEN: u8 = 0x4B;
    pub const MESSAGE_CHANNEL_CLOSE: u8 = 0x4C;
    pub const MESSAGE_CHANNEL_REQUEST: u8 = 0x4D;

    // Data messages
    pub const MESSAGE_CHANNEL_BROADCAST_DATA: u8 = 0x4E;
    pub const MESSAGE_CHANNEL_ACKNOWLEDGED_DATA: u8 = 0x4F;
    pub const MESSAGE_CHANNEL_BURST_DATA: u8 = 0x50;

    // Channel event messages
    pub const MESSAGE_CHANNEL_EVENT: u8 = 0x40;

    // Requested response messages
    pub const MESSAGE_CHANNEL_STATUS: u8 = 0x52;
    pub const MESSAGE_VERSION: u8 = 0x3E;
    pub const MESSAGE_CAPABILITIES: u8 = 0x54;
    pub const MESSAGE_SERIAL_NUMBER: u8 = 0x61;

    // Message parameters
    pub const CHANNEL_TYPE_TWOWAY_RECEIVE: u8 = 0x00;
    pub const CHANNEL_TYPE_TWOWAY_TRANSMIT: u8 = 0x10;
    pub const CHANNEL_TYPE_SHARED_RECEIVE: u8 = 0x20;
    pub const CHANNEL_TYPE_SHARED_TRANSMIT: u8 = 0x30;
    pub const CHANNEL_TYPE_ONEWAY_RECEIVE: u8 = 0x40;
    pub const CHANNEL_TYPE_ONEWAY_TRANSMIT: u8 = 0x50;

    pub const RADIO_TX_POWER_MINUS20DB: u8 = 0x00;
    pub const RADIO_TX_POWER_MINUS10DB: u8 = 0x01;
    pub const RADIO_TX_POWER_0DB: u8 = 0x02;
    pub const RADIO_TX_POWER_PLUS4DB: u8 = 0x03;

    pub const RESPONSE_NO_ERROR: u8 = 0x00;
    pub const EVENT_RX_SEARCH_TIMEOUT: u8 = 0x01;
    pub const EVENT_RX_FAIL: u8 = 0x02;
    pub const EVENT_TX: u8 = 0x03;
    pub const EVENT_TRANSFER_RX_FAILED: u8 = 0x04;
    pub const EVENT_TRANSFER_TX_COMPLETED: u8 = 0x05;
    pub const EVENT_TRANSFER_TX_FAILED: u8 = 0x06;
    pub const EVENT_CHANNEL_CLOSED: u8 = 0x07;
    pub const EVENT_RX_FAIL_GO_TO_SEARCH: u8 = 0x08;
    pub const EVENT_CHANNEL_COLLISION: u8 = 0x09;
    pub const EVENT_TRANSFER_TX_START: u8 = 0x0A;

    pub const CHANNEL_IN_WRONG_STATE: u8 = 0x15;
    pub const CHANNEL_NOT_OPENED: u8 = 0x16;
    pub const CHANNEL_ID_NOT_SET: u8 = 0x18;
    pub const CLOSE_ALL_CHANNELS: u8 = 0x19;
    pub const TRANSFER_IN_PROGRESS: u8 = 0x1F;
    pub const TRANSFER_SEQUENCE_NUMBER_ERROR: u8 = 0x20;
    pub const TRANSFER_IN_ERROR: u8 = 0x21;

    pub const MESSAGE_SIZE_EXCEEDS_LIMIT: u8 = 0x27;
    pub const INVALID_MESSAGE: u8 = 0x28;
    pub const INVALID_NETWORK_NUMBER: u8 = 0x29;
    pub const INVALID_LIST_ID: u8 = 0x30;
    pub const INVALID_SCAN_TX_CHANNEL: u8 = 0x31;
    pub const INVALID_PARAMETER_PROVIDED: u8 = 0x33;

    pub const EVENT_QUEUE_OVERFLOW: u8 = 0x35;
    pub const USB_STRING_WRITE_FAIL: u8 = 0x70;

    pub const CHANNEL_STATE_UNASSIGNED: u8 = 0x00;
    pub const CHANNEL_STATE_ASSIGNED: u8 = 0x01;
    pub const CHANNEL_STATE_SEARCHING: u8 = 0x02;
    pub const CHANNEL_STATE_TRACKING: u8 = 0x03;

    pub const CAPABILITIES_NO_RECEIVE_CHANNELS: u8 = 0x01;
    pub const CAPABILITIES_NO_TRANSMIT_CHANNELS: u8 = 0x02;
    pub const CAPABILITIES_NO_RECEIVE_MESSAGES: u8 = 0x04;
    pub const CAPABILITIES_NO_TRANSMIT_MESSAGES: u8 = 0x08;
    pub const CAPABILITIES_NO_ACKNOWLEDGED_MESSAGES: u8 = 0x10;
    pub const CAPABILITIES_NO_BURST_MESSAGES: u8 = 0x20;

    pub const CAPABILITIES_NETWORK_ENABLED: u8 = 0x02;
    pub const CAPABILITIES_SERIAL_NUMBER_ENABLED: u8 = 0x08;
    pub const CAPABILITIES_PER_CHANNEL_TX_POWER_ENABLED: u8 = 0x10;
    pub const CAPABILITIES_LOW_PRIORITY_SEARCH_ENABLED: u8 = 0x20;
    pub const CAPABILITIES_SCRIPT_ENABLED: u8 = 0x40;
    pub const CAPABILITIES_SEARCH_LIST_ENABLED: u8 = 0x80;

    pub const CAPABILITIES_LED_ENABLED: u8 = 0x01;
    pub const CAPABILITIES_EXT_MESSAGE_ENABLED: u8 = 0x02;
    pub const CAPABILITIES_SCAN_MODE_ENABLED: u8 = 0x04;
    pub const CAPABILITIES_PROX_SEARCH_ENABLED: u8 = 0x10;
    pub const CAPABILITIES_EXT_ASSIGN_ENABLED: u8 = 0x20;
    pub const CAPABILITIES_FS_ANTFS_ENABLED: u8 = 0x40;

    pub const TIMEOUT_NEVER: u8 = 0xFF;
}