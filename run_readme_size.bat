@echo off
for /f "tokens=*" %%a in ('type README.md ^| find /c /v ""') do echo Lines: %%a
