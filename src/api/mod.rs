//! External API
//! 
//! This files defined the external API available for the user, the project is 
//! structured in the following way. 
//! 
//! Each subfolder represent the funcionality associated with one endpoint, inside 
//! of them we will defined the format of the messages. Also each module will have
//! a different file for each verb that the endpoint support
//! 
pub mod login;
pub mod session;
pub mod oauth;

use std::net::{IpAddr, SocketAddr};

use envconfig::Envconfig;

/// Port
/// Alias for the port type
type Port = u16;

/// ConfigMap
/// Define the environmental configuration, this structure will define the 
/// different configuration variables that our service will be able to 
/// accept
#[derive(Envconfig, Clone, Debug)]
pub struct ConfigMap {
    /// Port where the service should be deployed
    #[envconfig(from = "SERVICE_PORT")]
    pub port: Port,
    /// Address of where the service gonna be bound
    #[envconfig(from = "SERVICE_ADDR")]
    pub address: IpAddr,
}

impl ConfigMap {
    /// Return the socket address associated with this configuration
    pub fn get_socket(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }
}