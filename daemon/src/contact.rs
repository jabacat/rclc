use super::DiscoveryResponse;
use std::net::{IpAddr, Ipv4Addr};

/// This structure is responsible for storing the contact information of a peer.
/// It can be obtained either through peer discovery (REST API) or be manually imported
pub struct Contact {
    pub prefered_name: Option<String>,
    pub github_username: Option<String>,
    pub verification_method: VerificationMethod,
    pub ip: IpAddr,
    pub port: u16,
    pub public_key: String,
}

/// Indicates the method used to verify the contact's identity.
/// Unverified: The contact's identity is not verified.
/// Github: The contact's identity is verified through Github (gpg public key lookup).
/// Signature: The contact's identity is verified through a user provided signature.
/// Manual: The contact's identity is verified because the user manually imported the contact.
pub enum VerificationMethod {
    Unverified,
    Github,
    Signature,
    Manual,
}

pub fn from_discovery(response: &DiscoveryResponse) -> Contact {
    Contact {
        prefered_name: None,
        github_username: None,
        verification_method: VerificationMethod::Unverified,
        ip: response
            .discovery
            .as_ref()
            .expect("Cannot parse a discovery response with no discovery request object")
            .ip
            .expect("Cannot parse a discovery response with no discovery request ip"),
        port: response
            .discovery
            .as_ref()
            .expect("Cannot parse a discovery response with no discovery request object")
            .port,
        public_key: response
            .discovery
            .as_ref()
            .expect("Cannot parse a discovery response with no discovery request object")
            .public_key
            .clone(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_discovery() {
        use crate::{DiscoveryRequest, Status};

        let request = DiscoveryRequest {
            ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            port: 2121,
            requested_by: "A".to_string(),
            looking_for: "B".to_string(),
            public_key: "qwertyuiop".to_string(),
        };

        let response = DiscoveryResponse {
            status: Status::Match,
            error: None,
            discovery: Some(request),
            message: "Hi >_<".to_string(),
        };

        let contact = from_discovery(&response);
        assert_eq!(contact.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
}
