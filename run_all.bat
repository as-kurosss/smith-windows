cd /d D:\Alexey\rust\smith-windows
cargo check
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%
cargo test 2>&1
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%
cargo clippy -- -D warnings 2>&1
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%
cargo fmt --check 2>&1
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%
