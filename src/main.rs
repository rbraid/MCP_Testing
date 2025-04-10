use mcp_testing::{MCPServer, ServerStatus};

fn main() {
    // Define a list of MCP servers to check
    let server_configs = [
        ("MCP-Alpha", "192.168.1.100", 8080),
        ("MCP-Beta", "192.168.1.101", 8080),
        ("MCP-Gamma", "192.168.1.102", 8080),
        ("MCP-Delta", "192.168.1.103", 8080),
    ];
    
    println!("MCP Server Status Check");
    println!("======================");
    
    // Create and check each server
    for (name, address, port) in server_configs.iter() {
        let mut server = MCPServer::new(name, address, *port);
        server.check_status();
        
        println!("Server: {}", server.name);
        println!("Address: {}:{}", server.address, server.port);
        println!("Status: {}", server.status_description());
        println!("----------------------");
    }
}
