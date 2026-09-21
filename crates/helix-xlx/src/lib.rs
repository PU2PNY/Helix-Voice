#![forbid(unsafe_code)]

/// XLXD's public transcoder control port.
pub const XLXD_TRANSCODER_CONTROL_PORT: u16 = 10_100;

/// The public ambed controller allocates stream ids 1..=99 and listens on
/// control_port + stream_id.
pub const XLXD_MAX_STREAMS: u16 = 99;

const TAG_PING: &[u8; 9] = b"AMBEDPING";
const TAG_PONG: &[u8; 9] = b"AMBEDPONG";
const TAG_OPEN: &[u8; 7] = b"AMBEDOS";
const TAG_STREAM: &[u8; 8] = b"AMBEDSTD";
const TAG_BUSY: &[u8; 9] = b"AMBEDBUSY";
const TAG_CLOSE: &[u8; 7] = b"AMBEDCS";

/// XLX integration starts disabled and must be explicitly enabled in a lab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntegrationMode {
    #[default]
    Disabled,
    ObserveOnly,
    ExternalLicensedBackend,
}

/// No proprietary codec implementation belongs in this crate.
///
/// This crate contains only the public interoperability boundary needed
/// to connect Helix to XLX/XLXD and route audio to explicitly configured
/// codec backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XlxBridgeConfig {
    pub mode: IntegrationMode,
    pub max_streams: u16,
}

impl Default for XlxBridgeConfig {
    fn default() -> Self {
        Self {
            mode: IntegrationMode::Disabled,
            max_streams: 8,
        }
    }
}

/// Public codec identifiers used by the XLXD/ambed control boundary.
///
/// These identifiers describe an external legacy backend. Helix does not
/// implement either proprietary codec in this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LegacyCodecId {
    AmbePlus = 1,
    Ambe2Plus = 2,
}

impl TryFrom<u8> for LegacyCodecId {
    type Error = ControlPacketError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::AmbePlus),
            2 => Ok(Self::Ambe2Plus),
            _ => Err(ControlPacketError::UnsupportedCodec(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlPacketError {
    InvalidLength,
    InvalidTag,
    InvalidCallsign,
    UnsupportedCodec(u8),
    InvalidStreamId,
    InvalidStreamPort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XlxCallsign([u8; 8]);

impl XlxCallsign {
    pub fn new(value: &str) -> Result<Self, ControlPacketError> {
        let trimmed = value.trim();
        if trimmed.is_empty() || trimmed.len() > 8 || !trimmed.is_ascii() {
            return Err(ControlPacketError::InvalidCallsign);
        }

        if !trimmed
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'/')
        {
            return Err(ControlPacketError::InvalidCallsign);
        }

        let mut bytes = [b' '; 8];
        bytes[..trimmed.len()].copy_from_slice(trimmed.as_bytes());
        Ok(Self(bytes))
    }

    pub fn from_wire(bytes: [u8; 8]) -> Result<Self, ControlPacketError> {
        let text = core::str::from_utf8(&bytes).map_err(|_| ControlPacketError::InvalidCallsign)?;
        Self::new(text)
    }

    #[must_use]
    pub const fn as_wire(&self) -> &[u8; 8] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlPacket {
    Ping {
        callsign: XlxCallsign,
    },
    Pong,
    OpenStream {
        callsign: XlxCallsign,
        codec_in: LegacyCodecId,
        codec_out: LegacyCodecId,
    },
    StreamDescriptor {
        stream_id: u16,
        port: u16,
        codec_in: LegacyCodecId,
        codec_out: LegacyCodecId,
    },
    Busy,
    CloseStream {
        stream_id: u16,
    },
}

impl ControlPacket {
    pub fn decode(bytes: &[u8]) -> Result<Self, ControlPacketError> {
        match bytes.len() {
            17 if bytes.starts_with(TAG_PING) => {
                let mut callsign = [0_u8; 8];
                callsign.copy_from_slice(&bytes[9..17]);
                Ok(Self::Ping {
                    callsign: XlxCallsign::from_wire(callsign)?,
                })
            }
            9 if bytes == TAG_PONG => Ok(Self::Pong),
            17 if bytes.starts_with(TAG_OPEN) => {
                let mut callsign = [0_u8; 8];
                callsign.copy_from_slice(&bytes[7..15]);
                Ok(Self::OpenStream {
                    callsign: XlxCallsign::from_wire(callsign)?,
                    codec_in: LegacyCodecId::try_from(bytes[15])?,
                    codec_out: LegacyCodecId::try_from(bytes[16])?,
                })
            }
            14 if bytes.starts_with(TAG_STREAM) => {
                let stream_id = u16::from_le_bytes([bytes[8], bytes[9]]);
                let port = u16::from_le_bytes([bytes[10], bytes[11]]);
                validate_stream(stream_id, port)?;
                Ok(Self::StreamDescriptor {
                    stream_id,
                    port,
                    codec_in: LegacyCodecId::try_from(bytes[12])?,
                    codec_out: LegacyCodecId::try_from(bytes[13])?,
                })
            }
            9 if bytes == TAG_BUSY => Ok(Self::Busy),
            9 if bytes.starts_with(TAG_CLOSE) => {
                let stream_id = u16::from_le_bytes([bytes[7], bytes[8]]);
                if !(1..=XLXD_MAX_STREAMS).contains(&stream_id) {
                    return Err(ControlPacketError::InvalidStreamId);
                }
                Ok(Self::CloseStream { stream_id })
            }
            9 | 14 | 17 => Err(ControlPacketError::InvalidTag),
            _ => Err(ControlPacketError::InvalidLength),
        }
    }

    pub fn encode(&self, output: &mut [u8; 17]) -> Result<usize, ControlPacketError> {
        output.fill(0);

        match *self {
            Self::Ping { callsign } => {
                output[..9].copy_from_slice(TAG_PING);
                output[9..17].copy_from_slice(callsign.as_wire());
                Ok(17)
            }
            Self::Pong => {
                output[..9].copy_from_slice(TAG_PONG);
                Ok(9)
            }
            Self::OpenStream {
                callsign,
                codec_in,
                codec_out,
            } => {
                output[..7].copy_from_slice(TAG_OPEN);
                output[7..15].copy_from_slice(callsign.as_wire());
                output[15] = codec_in as u8;
                output[16] = codec_out as u8;
                Ok(17)
            }
            Self::StreamDescriptor {
                stream_id,
                port,
                codec_in,
                codec_out,
            } => {
                validate_stream(stream_id, port)?;
                output[..8].copy_from_slice(TAG_STREAM);
                output[8..10].copy_from_slice(&stream_id.to_le_bytes());
                output[10..12].copy_from_slice(&port.to_le_bytes());
                output[12] = codec_in as u8;
                output[13] = codec_out as u8;
                Ok(14)
            }
            Self::Busy => {
                output[..9].copy_from_slice(TAG_BUSY);
                Ok(9)
            }
            Self::CloseStream { stream_id } => {
                if !(1..=XLXD_MAX_STREAMS).contains(&stream_id) {
                    return Err(ControlPacketError::InvalidStreamId);
                }
                output[..7].copy_from_slice(TAG_CLOSE);
                output[7..9].copy_from_slice(&stream_id.to_le_bytes());
                Ok(9)
            }
        }
    }
}

fn validate_stream(stream_id: u16, port: u16) -> Result<(), ControlPacketError> {
    if !(1..=XLXD_MAX_STREAMS).contains(&stream_id) {
        return Err(ControlPacketError::InvalidStreamId);
    }

    let expected_port = XLXD_TRANSCODER_CONTROL_PORT
        .checked_add(stream_id)
        .ok_or(ControlPacketError::InvalidStreamPort)?;
    if port != expected_port {
        return Err(ControlPacketError::InvalidStreamPort);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_is_disabled_by_default() {
        assert_eq!(XlxBridgeConfig::default().mode, IntegrationMode::Disabled);
    }

    #[test]
    fn callsign_is_fixed_width_and_validated() {
        let callsign = XlxCallsign::new("PU2PNY").expect("valid callsign");
        assert_eq!(callsign.as_wire(), b"PU2PNY  ");
        assert_eq!(
            XlxCallsign::new(""),
            Err(ControlPacketError::InvalidCallsign)
        );
        assert_eq!(
            XlxCallsign::new("TOO-LONG9"),
            Err(ControlPacketError::InvalidCallsign)
        );
    }

    #[test]
    fn public_control_packets_round_trip() {
        let callsign = XlxCallsign::new("PU2PNY").expect("callsign");
        let packets = [
            ControlPacket::Ping { callsign },
            ControlPacket::Pong,
            ControlPacket::OpenStream {
                callsign,
                codec_in: LegacyCodecId::AmbePlus,
                codec_out: LegacyCodecId::Ambe2Plus,
            },
            ControlPacket::StreamDescriptor {
                stream_id: 7,
                port: 10_107,
                codec_in: LegacyCodecId::AmbePlus,
                codec_out: LegacyCodecId::Ambe2Plus,
            },
            ControlPacket::Busy,
            ControlPacket::CloseStream { stream_id: 7 },
        ];

        for packet in packets {
            let mut wire = [0_u8; 17];
            let len = packet.encode(&mut wire).expect("encode");
            let decoded = ControlPacket::decode(&wire[..len]).expect("decode");
            assert_eq!(decoded, packet);
        }
    }

    #[test]
    fn open_stream_wire_format_is_stable() {
        let packet = ControlPacket::OpenStream {
            callsign: XlxCallsign::new("PU2PNY").expect("callsign"),
            codec_in: LegacyCodecId::AmbePlus,
            codec_out: LegacyCodecId::Ambe2Plus,
        };

        let mut wire = [0_u8; 17];
        let len = packet.encode(&mut wire).expect("encode");
        assert_eq!(len, 17);
        assert_eq!(&wire, b"AMBEDOSPU2PNY  \x01\x02");
    }

    #[test]
    fn captured_lab_ping_decodes() {
        let packet = ControlPacket::decode(b"AMBEDPINGXLX999  ").expect("decode captured ping");
        assert_eq!(
            packet,
            ControlPacket::Ping {
                callsign: XlxCallsign::new("XLX999").expect("callsign"),
            }
        );
    }

    #[test]
    fn malformed_control_packets_are_rejected() {
        assert_eq!(
            ControlPacket::decode(b"bad"),
            Err(ControlPacketError::InvalidLength)
        );

        let mut invalid_codec = *b"AMBEDOSPU2PNY  \x01\x09";
        assert_eq!(
            ControlPacket::decode(&invalid_codec),
            Err(ControlPacketError::UnsupportedCodec(9))
        );

        invalid_codec[16] = 2;
        assert!(ControlPacket::decode(&invalid_codec).is_ok());

        let mut bad_stream = [0_u8; 14];
        bad_stream[..8].copy_from_slice(TAG_STREAM);
        bad_stream[8..10].copy_from_slice(&7_u16.to_le_bytes());
        bad_stream[10..12].copy_from_slice(&10_108_u16.to_le_bytes());
        bad_stream[12] = 1;
        bad_stream[13] = 2;
        assert_eq!(
            ControlPacket::decode(&bad_stream),
            Err(ControlPacketError::InvalidStreamPort)
        );
    }
}
