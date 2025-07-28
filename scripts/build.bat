@echo off
setlocal enabledelayedexpansion

REM Colors for output (Windows 10+)
set "GREEN=[92m"
set "YELLOW=[93m"
set "RED=[91m"
set "NC=[0m"

REM Function to print colored output
:print_status
echo %GREEN%[INFO]%NC% %~1
goto :eof

:print_warning
echo %YELLOW%[WARNING]%NC% %~1
goto :eof

:print_error
echo %RED%[ERROR]%NC% %~1
goto :eof

REM Check if we're in the right directory
if not exist "metadata-generator\Cargo.toml" (
    call :print_error "Please run this script from the project root directory"
    exit /b 1
)

call :print_status "Starting build process..."

REM Change to the metadata-generator directory
cd metadata-generator

REM Clean previous builds
call :print_status "Cleaning previous builds..."
cargo clean

REM Create output directory
if not exist "..\dist" mkdir "..\dist"

REM Define targets to build
set "targets=x86_64-unknown-linux-gnu x86_64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin"

REM Build CLI version for all targets
call :print_status "Building CLI version..."
for %%t in (%targets%) do (
    call :print_status "Building CLI for target: %%t"
    cargo build --release --target %%t --features cli
    if !errorlevel! equ 0 (
        call :print_status "CLI build successful for %%t"
        if "%%t"=="x86_64-pc-windows-msvc" (
            copy "target\%%t\release\cli.exe" "..\dist\cli-%%t.exe" >nul
        ) else (
            copy "target\%%t\release\cli" "..\dist\cli-%%t" >nul
        )
    ) else (
        call :print_warning "CLI build failed for %%t"
    )
)

REM Build GUI version for all targets
call :print_status "Building GUI version..."
for %%t in (%targets%) do (
    call :print_status "Building GUI for target: %%t"
    cargo build --release --target %%t --features gui
    if !errorlevel! equ 0 (
        call :print_status "GUI build successful for %%t"
        if "%%t"=="x86_64-pc-windows-msvc" (
            copy "target\%%t\release\MetadataGenerator.exe" "..\dist\MetadataGenerator-%%t.exe" >nul
        ) else (
            copy "target\%%t\release\MetadataGenerator" "..\dist\MetadataGenerator-%%t" >nul
        )
    ) else (
        call :print_warning "GUI build failed for %%t"
    )
)

REM Create zip files for distribution
call :print_status "Creating distribution packages..."
cd ..\dist

REM Create zip files for each platform
for %%t in (%targets%) do (
    set "platform_name="
    if "%%t"=="x86_64-unknown-linux-gnu" set "platform_name=linux-x64"
    if "%%t"=="x86_64-pc-windows-msvc" set "platform_name=windows-x64"
    if "%%t"=="x86_64-apple-darwin" set "platform_name=macos-x64"
    if "%%t"=="aarch64-apple-darwin" set "platform_name=macos-arm64"
    
    if exist "cli-%%t.exe" (
        powershell -Command "Compress-Archive -Path 'cli-%%t.exe','MetadataGenerator-%%t.exe' -DestinationPath 'MetadataGenerator-!platform_name!.zip' -Force"
        call :print_status "Created package: MetadataGenerator-!platform_name!.zip"
    ) else if exist "cli-%%t" (
        powershell -Command "Compress-Archive -Path 'cli-%%t','MetadataGenerator-%%t' -DestinationPath 'MetadataGenerator-!platform_name!.zip' -Force"
        call :print_status "Created package: MetadataGenerator-!platform_name!.zip"
    )
)

call :print_status "Build process completed!"
call :print_status "Distribution packages are available in the 'dist' directory"

pause 