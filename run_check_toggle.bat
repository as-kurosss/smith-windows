@echo off
cargo check 2>&1 > D:\Alexey\rust\smith-windows\cargo_check_toggle.txt
echo Exit code: %errorlevel%
