@echo off
echo Running cargo build...
cargo build %*
if %ERRORLEVEL% NEQ 0 (
    echo Build FAILED with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)
echo Build completed successfully!
