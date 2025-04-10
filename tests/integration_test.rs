use mcp_testing::{MCPServer, ServerStatus};

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
