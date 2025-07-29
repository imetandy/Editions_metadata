#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== App Bundle Test ===${NC}"

APP_NAME="MetadataGenerator"
APP_BUNDLE="${APP_NAME}.app"

# Check if app bundle exists
if [ ! -d "${APP_BUNDLE}" ]; then
    echo -e "${RED}Error: ${APP_BUNDLE} not found!${NC}"
    echo -e "${YELLOW}Run ./scripts/build-app.sh first${NC}"
    exit 1
fi

echo -e "${YELLOW}Testing app bundle structure...${NC}"

# Check app bundle structure
if [ -d "${APP_BUNDLE}/Contents" ]; then
    echo -e "${GREEN}✓ Contents directory exists${NC}"
else
    echo -e "${RED}✗ Contents directory missing${NC}"
    exit 1
fi

if [ -d "${APP_BUNDLE}/Contents/MacOS" ]; then
    echo -e "${GREEN}✓ MacOS directory exists${NC}"
else
    echo -e "${RED}✗ MacOS directory missing${NC}"
    exit 1
fi

if [ -d "${APP_BUNDLE}/Contents/Resources" ]; then
    echo -e "${GREEN}✓ Resources directory exists${NC}"
else
    echo -e "${RED}✗ Resources directory missing${NC}"
    exit 1
fi

if [ -f "${APP_BUNDLE}/Contents/Info.plist" ]; then
    echo -e "${GREEN}✓ Info.plist exists${NC}"
else
    echo -e "${RED}✗ Info.plist missing${NC}"
    exit 1
fi

# Check executable
if [ -f "${APP_BUNDLE}/Contents/MacOS/${APP_NAME}" ]; then
    echo -e "${GREEN}✓ Executable exists${NC}"
    
    # Check if executable
    if [ -x "${APP_BUNDLE}/Contents/MacOS/${APP_NAME}" ]; then
        echo -e "${GREEN}✓ Executable has proper permissions${NC}"
    else
        echo -e "${RED}✗ Executable not executable${NC}"
        chmod +x "${APP_BUNDLE}/Contents/MacOS/${APP_NAME}"
        echo -e "${YELLOW}Fixed permissions${NC}"
    fi
else
    echo -e "${RED}✗ Executable missing${NC}"
    exit 1
fi

# Check file size
EXEC_SIZE=$(stat -f%z "${APP_BUNDLE}/Contents/MacOS/${APP_NAME}")
if [ "$EXEC_SIZE" -gt 1000000 ]; then
    echo -e "${GREEN}✓ Executable size looks good (${EXEC_SIZE} bytes)${NC}"
else
    echo -e "${YELLOW}⚠ Executable seems small (${EXEC_SIZE} bytes)${NC}"
fi

# Check for quarantine attribute
if xattr "${APP_BUNDLE}" 2>/dev/null | grep -q "com.apple.quarantine"; then
    echo -e "${YELLOW}⚠ Quarantine attribute found, removing...${NC}"
    xattr -cr "${APP_BUNDLE}"
    echo -e "${GREEN}✓ Quarantine removed${NC}"
else
    echo -e "${GREEN}✓ No quarantine attribute${NC}"
fi

# Test app bundle validity
echo -e "${YELLOW}Testing app bundle validity...${NC}"
if codesign -dv "${APP_BUNDLE}" 2>/dev/null; then
    echo -e "${GREEN}✓ App bundle is code signed${NC}"
else
    echo -e "${YELLOW}⚠ App bundle is not code signed (this is normal for development)${NC}"
fi

echo -e "${GREEN}=== App Bundle Test Complete ===${NC}"
echo -e "${BLUE}You can now test by double-clicking ${APP_BUNDLE}${NC}"
echo -e "${YELLOW}If it says 'damaged', right-click and choose 'Open'${NC}" 