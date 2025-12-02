/// Internal frame structure for RBK protocol
#[derive(Debug, Clone)]
pub struct RbkFrame {
    pub flow_no: u16,
    #[allow(dead_code)]
    pub api_no: u16,
    pub body: String,
}
