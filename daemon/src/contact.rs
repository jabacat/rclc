use std::net::IpAddr;

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
