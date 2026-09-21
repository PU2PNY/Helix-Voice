#![forbid(unsafe_code)]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_is_disabled_by_default() {
        assert_eq!(XlxBridgeConfig::default().mode, IntegrationMode::Disabled);
    }
}
