//! API request types for RBK robot communication
//!
//! This module defines the API request enum that categorizes all RBK APIs
//! into their respective modules based on the RBK protocol specification.

/// API request enum representing all RBK robot APIs
///
/// The RBK protocol organizes APIs into modules, each with its own port:
/// - State APIs (1000-1999): Robot state queries on port 19204
/// - Control APIs (2000-2999): Robot control commands on port 19205
/// - Navigation APIs (3000-3999): Navigation commands on port 19206
/// - Config APIs (4000-5999): Configuration management on port 19207
/// - Kernel APIs (7000-7999): Kernel operations on port 19208
/// - Misc APIs (6000-6998): Miscellaneous operations on port 19210
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiRequest {
    /// State module APIs (1000-1999)
    State(StateApi),
    /// Control module APIs (2000-2999)
    Control(ControlApi),
    /// Navigation module APIs (3000-3999)
    Nav(NavApi),
    /// Config module APIs (4000-5999)
    Config(ConfigApi),
    /// Kernel module APIs (7000-7999)
    Kernel(KernelApi),
    /// Misc module APIs (6000-6998)
    Misc(MiscApi),
}

impl ApiRequest {
    /// Get the API number for this request
    pub fn api_no(&self) -> u16 {
        match self {
            ApiRequest::State(api) => api.api_no(),
            ApiRequest::Control(api) => api.api_no(),
            ApiRequest::Nav(api) => api.api_no(),
            ApiRequest::Config(api) => api.api_no(),
            ApiRequest::Kernel(api) => api.api_no(),
            ApiRequest::Misc(api) => api.api_no(),
        }
    }
}

/// State module APIs (1000-1999)
///
/// These APIs query the robot's current state and status information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateApi {
    /// Query robot battery level (API 1007)
    QueryBattery,
    /// Query robot pose and position (API 1004)
    QueryPose,
    /// Query robot speed (API 1005)
    QuerySpeed,
    /// Query robot status (API 1001)
    QueryStatus,
    /// Custom state API with explicit API number (must be in range 1000-1999)
    Custom(u16),
}

impl StateApi {
    pub fn api_no(&self) -> u16 {
        match self {
            StateApi::QueryBattery => 1007,
            StateApi::QueryPose => 1004,
            StateApi::QuerySpeed => 1005,
            StateApi::QueryStatus => 1001,
            StateApi::Custom(no) => {
                debug_assert!(
                    (1000..=1999).contains(no),
                    "Custom State API number {} is outside valid range 1000-1999",
                    no
                );
                *no
            }
        }
    }
}

/// Control module APIs (2000-2999)
///
/// These APIs control robot movement and behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlApi {
    /// Set robot speed (API 2001)
    SetSpeed,
    /// Stop robot (API 2002)
    Stop,
    /// Custom control API with explicit API number (must be in range 2000-2999)
    Custom(u16),
}

impl ControlApi {
    pub fn api_no(&self) -> u16 {
        match self {
            ControlApi::SetSpeed => 2001,
            ControlApi::Stop => 2002,
            ControlApi::Custom(no) => {
                debug_assert!(
                    (2000..=2999).contains(no),
                    "Custom Control API number {} is outside valid range 2000-2999",
                    no
                );
                *no
            }
        }
    }
}

/// Navigation module APIs (3000-3999)
///
/// These APIs manage robot navigation and path planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavApi {
    /// Start navigation (API 3001)
    StartNav,
    /// Stop navigation (API 3002)
    StopNav,
    /// Pause navigation (API 3003)
    PauseNav,
    /// Resume navigation (API 3004)
    ResumeNav,
    /// Custom navigation API with explicit API number (must be in range 3000-3999)
    Custom(u16),
}

impl NavApi {
    pub fn api_no(&self) -> u16 {
        match self {
            NavApi::StartNav => 3001,
            NavApi::StopNav => 3002,
            NavApi::PauseNav => 3003,
            NavApi::ResumeNav => 3004,
            NavApi::Custom(no) => {
                debug_assert!(
                    (3000..=3999).contains(no),
                    "Custom Nav API number {} is outside valid range 3000-3999",
                    no
                );
                *no
            }
        }
    }
}

/// Config module APIs (4000-5999)
///
/// These APIs manage robot configuration settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigApi {
    /// Get configuration (API 4001)
    GetConfig,
    /// Set configuration (API 4002)
    SetConfig,
    /// Custom config API with explicit API number (must be in range 4000-5999)
    Custom(u16),
}

impl ConfigApi {
    pub fn api_no(&self) -> u16 {
        match self {
            ConfigApi::GetConfig => 4001,
            ConfigApi::SetConfig => 4002,
            ConfigApi::Custom(no) => {
                debug_assert!(
                    (4000..=5999).contains(no),
                    "Custom Config API number {} is outside valid range 4000-5999",
                    no
                );
                *no
            }
        }
    }
}

/// Kernel module APIs (7000-7999)
///
/// These APIs interact with the robot's kernel layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelApi {
    /// Custom kernel API with explicit API number (must be in range 7000-7999)
    Custom(u16),
}

impl KernelApi {
    pub fn api_no(&self) -> u16 {
        match self {
            KernelApi::Custom(no) => {
                debug_assert!(
                    (7000..=7999).contains(no),
                    "Custom Kernel API number {} is outside valid range 7000-7999",
                    no
                );
                *no
            }
        }
    }
}

/// Misc module APIs (6000-6998)
///
/// These APIs provide miscellaneous functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscApi {
    /// Custom misc API with explicit API number (must be in range 6000-6998)
    Custom(u16),
}

impl MiscApi {
    pub fn api_no(&self) -> u16 {
        match self {
            MiscApi::Custom(no) => {
                debug_assert!(
                    (6000..=6998).contains(no),
                    "Custom Misc API number {} is outside valid range 6000-6998",
                    no
                );
                *no
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_numbers() {
        assert_eq!(ApiRequest::State(StateApi::QueryBattery).api_no(), 1007);
        assert_eq!(ApiRequest::State(StateApi::QueryPose).api_no(), 1004);
        assert_eq!(ApiRequest::Control(ControlApi::SetSpeed).api_no(), 2001);
        assert_eq!(ApiRequest::Nav(NavApi::StartNav).api_no(), 3001);
        assert_eq!(ApiRequest::Config(ConfigApi::GetConfig).api_no(), 4001);
    }

    #[test]
    fn test_custom_apis() {
        assert_eq!(ApiRequest::State(StateApi::Custom(1999)).api_no(), 1999);
        assert_eq!(ApiRequest::Control(ControlApi::Custom(2500)).api_no(), 2500);
    }
}
