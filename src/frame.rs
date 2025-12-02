/// Result kind for RBK requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RbkResultKind {
    /// Request succeeded
    Ok,
    /// Robot not found
    NoSuchRobot,
    /// Connection failed
    ConnectFail,
    /// Write error
    WriteError,
    /// Client disposed
    Disposed,
    /// Bad API number
    BadApiNo,
    /// Request timeout
    Timeout,
    /// Request interrupted
    Interrupted,
}

/// Internal frame structure for RBK protocol
#[derive(Debug, Clone)]
pub struct RbkFrame {
    pub flow_no: u16,
    #[allow(dead_code)]
    pub api_no: u16,
    pub body: String,
}
