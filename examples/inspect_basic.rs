//! InspectTool basic example: Demonstrates core functionality
//!
//! This example shows how to:
//! 1. Create InspectConfig with timeout and cancellation
//! 2. Validate configuration
//! 3. Handle errors and edge cases

use std::time::Duration;

use smith_windows::core::inspect::{validate_inspect_config, InspectConfig, InspectError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== InspectTool Basic Example ===\n");

    // Example 1: Valid configuration
    println!("1. Valid configuration:");
    let cancellation = tokio_util::sync::CancellationToken::new();
    let config = InspectConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    match validate_inspect_config(&config) {
        Ok(()) => println!("   ✓ Configuration is valid (timeout: 5s)"),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Example 2: Invalid configuration - zero timeout
    println!("\n2. Invalid configuration (zero timeout):");
    let config_zero = InspectConfig {
        timeout: Duration::ZERO,
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    match validate_inspect_config(&config_zero) {
        Ok(()) => println!("   ✓ Configuration is valid"),
        Err(InspectError::InvalidConfig(msg)) => {
            println!("   ✗ Zero timeout correctly rejected: {}", msg)
        }
        Err(e) => println!("   ✗ Unexpected error: {}", e),
    }

    // Example 3: Invalid configuration - timeout too large
    println!("\n3. Invalid configuration (timeout > 1 hour):");
    let config_large = InspectConfig {
        timeout: Duration::from_secs(3601), // > 1 hour
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    match validate_inspect_config(&config_large) {
        Ok(()) => println!("   ✓ Configuration is valid"),
        Err(InspectError::InvalidConfig(msg)) => {
            println!("   ✗ Timeout > 1 hour correctly rejected: {}", msg)
        }
        Err(e) => println!("   ✗ Unexpected error: {}", e),
    }

    // Example 4: Cancellation token usage
    println!("\n4. Cancellation token:");
    let cancellation = tokio_util::sync::CancellationToken::new();
    let _config_with_cancel = InspectConfig {
        timeout: Duration::from_secs(10),
        cancellation: cancellation.clone(),
    };

    println!("   Token is valid: {}", !cancellation.is_cancelled());
    println!("   Config created with cancellation support");

    // Example 5: Error types
    println!("\n5. InspectError types:");
    let errors = [
        InspectError::ElementNotFound,
        InspectError::ElementNotEnabled,
        InspectError::ElementOffscreen,
        InspectError::Timeout,
        InspectError::Cancelled,
        InspectError::InvalidConfig("test".to_string()),
        InspectError::ComError("COM error".to_string()),
        InspectError::InvalidSelector,
    ];

    for error in &errors {
        println!("   - {}", error);
    }

    println!("\n=== Summary ===");
    println!("InspectTool provides:");
    println!("✓ Configuration validation");
    println!("✓ Timeout handling via tokio::time::timeout");
    println!("✓ Cancellation support via CancellationToken");
    println!("✓ Full hierarchy path building via UITreeWalker");
    println!("✓ Error types for all failure scenarios");

    Ok(())
}
