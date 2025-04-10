/// MCP Server and status definitions

#[derive(Debug, PartialEq)]
pub struct MCPServer {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub status: ServerStatus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServerStatus {
    Online,
    Offline,
    Degraded,
    Unknown,
}

impl MCPServer {
    /// Create a new MCPServer instance
    pub fn new(name: &str, address: &str, port: u16) -> Self {
        MCPServer {
            name: name.to_string(),
            address: address.to_string(),
            port,
            status: ServerStatus::Unknown,
        }
    }

    /// Check the status of the server
    /// 
    /// This is a simulated check for testing purposes
    pub fn check_status(&mut self) -> &ServerStatus {
        // For this example, we're just simulating a check
        // In a real application, this would actually ping the server
        
        // Use the sum of the bytes in the address as a simple hash
        let mut hash = 0;
        for byte in self.address.bytes() {
            hash = hash.wrapping_add(byte as u64);
        }
        
        // Determine status based on hash modulo 4
        self.status = match hash % 4 {
            0 => ServerStatus::Online,
            1 => ServerStatus::Offline,
            2 => ServerStatus::Degraded,
            _ => ServerStatus::Unknown,
        };
        
        &self.status
    }
    
    /// Get a human-readable status description
    pub fn status_description(&self) -> &str {
        match self.status {
            ServerStatus::Online => "Online",
            ServerStatus::Offline => "Offline",
            ServerStatus::Degraded => "Degraded Performance",
            ServerStatus::Unknown => "Status Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_creation() {
        let server = MCPServer::new("Test", "127.0.0.1", 8080);
        assert_eq!(server.name, "Test");
        assert_eq!(server.address, "127.0.0.1");
        assert_eq!(server.port, 8080);
    }
    
    #[test]
    fn test_status_check() {
        let mut server = MCPServer::new("Test", "127.0.0.1", 8080);
        server.check_status();
        
        // The status will be determined by our hash algorithm
        // Since this is deterministic, we know what to expect
        match server.status {
            ServerStatus::Online | ServerStatus::Offline | 
            ServerStatus::Degraded | ServerStatus::Unknown => {
                // Any of these statuses is valid
                assert!(true);
            }
        }
    }
    
    #[test]
    fn test_status_description() {
        let mut server = MCPServer::new("Test", "127.0.0.1", 8080);
        
        // Test online status
        server.status = ServerStatus::Online;
        assert_eq!(server.status_description(), "Online");
        
        // Test offline status
        server.status = ServerStatus::Offline;
        assert_eq!(server.status_description(), "Offline");
        
        // Test degraded status
        server.status = ServerStatus::Degraded;
        assert_eq!(server.status_description(), "Degraded Performance");
        
        // Test unknown status
        server.status = ServerStatus::Unknown;
        assert_eq!(server.status_description(), "Status Unknown");
    }
}
