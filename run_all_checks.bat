# Test script for smith-windows
# Updated for Result get_state() handling

@echo off
echo Running cargo check...
cargo check
if %ERRORLEVEL% NEQ 0 (
    echo Cargo check failed!
    exit /b %ERRORLEVEL%
)

echo.
echo Running cargo test...
cargo test 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Cargo test failed!
    exit /b %ERRORLEVEL%
)

echo.
echo Running cargo clippy...
cargo clippy -- -D warnings 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Cargo clippy failed!
    exit /b %ERRORLEVEL%
)

echo.
echo Running cargo fmt check...
cargo fmt --check 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Cargo fmt check failed!
    exit /b %ERRORLEVEL%
)

echo.
echo All checks passed!
