@echo off
REM Build script for WASM version of brkrs (Windows)
REM This generates the necessary JavaScript files from the compiled WASM binary

echo Building brkrs for WASM...

REM Check if wasm-bindgen is installed
where wasm-bindgen >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Error: wasm-bindgen is not installed.
    echo Install it with: cargo install wasm-bindgen-cli
    exit /b 1
)

REM Check if WASM binary exists
set WASM_BINARY=target\wasm32-unknown-unknown\release\brkrs.wasm
if not exist "%WASM_BINARY%" (
    echo Error: WASM binary not found at %WASM_BINARY%
    echo Build it first with: cargo build --release --target wasm32-unknown-unknown
    exit /b 1
)

REM Generate JavaScript bindings
echo Generating JavaScript bindings...
wasm-bindgen --out-dir . --target web "%WASM_BINARY%"
if errorlevel 1 (
    echo Error: wasm-bindgen failed. Aborting build.
    exit /b 1
)

REM Copy assets if they exist
if exist "..\assets" (
    echo Copying assets...
    xcopy /E /I /Y ..\assets assets >nul 2>&1
)

echo Build complete! Generated files:
echo   - brkrs.js
echo   - brkrs_bg.wasm
echo.
echo You can now open index.html in a web server
echo Example: python -m http.server 8080

