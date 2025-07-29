#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== MetadataGenerator Quick Fix Tool ===${NC}"
echo -e "${YELLOW}This script will automatically fix common installation issues${NC}"

# Check if app exists
APP_PATH="/Applications/MetadataGenerator.app"
if [ ! -d "$APP_PATH" ]; then
    echo -e "${RED}Error: MetadataGenerator.app not found in /Applications${NC}"
    echo -e "${YELLOW}Please install the app first by dragging it to Applications${NC}"
    exit 1
fi

echo -e "${GREEN}✓ App found${NC}"

# Fix 1: Remove quarantine attributes
echo -e "${YELLOW}Fixing quarantine attributes...${NC}"
xattr -cr "$APP_PATH" 2>/dev/null || true
echo -e "${GREEN}✓ Quarantine attributes removed${NC}"

# Fix 2: Fix permissions
echo -e "${YELLOW}Fixing permissions...${NC}"
chmod +x "$APP_PATH/Contents/MacOS/MetadataGenerator" 2>/dev/null || true
chmod 755 "$APP_PATH" 2>/dev/null || true
echo -e "${GREEN}✓ Permissions fixed${NC}"

# Fix 3: Check for FFmpeg
echo -e "${YELLOW}Checking FFmpeg installation...${NC}"

# First check if FFmpeg is already available
if command -v ffmpeg &> /dev/null; then
    echo -e "${GREEN}✓ FFmpeg is already available in PATH${NC}"
    FFMPEG_VERSION=$(ffmpeg -version | head -n1)
    echo -e "${BLUE}FFmpeg version: $FFMPEG_VERSION${NC}"
else
    # Check if FFmpeg is installed in Applications
    if [ -d "/Applications/FFmpeg.app" ]; then
        echo -e "${GREEN}✓ FFmpeg found in Applications folder${NC}"
        FFMPEG_APP_PATH="/Applications/FFmpeg.app"
        FFMPEG_BIN_PATH="$FFMPEG_APP_PATH/Contents/Resources/bin"
        
        if [ -d "$FFMPEG_BIN_PATH" ] && [ -f "$FFMPEG_BIN_PATH/ffmpeg" ]; then
            echo -e "${BLUE}FFmpeg binaries found at: $FFMPEG_BIN_PATH${NC}"
            # Add to PATH for current session
            export PATH="$FFMPEG_BIN_PATH:$PATH"
            echo -e "${GREEN}✓ Added FFmpeg to PATH for current session${NC}"
        fi
    elif [ -f "/Applications/ffmpeg" ]; then
        echo -e "${GREEN}✓ FFmpeg executable found directly in Applications${NC}"
        # Add to PATH for current session
        export PATH="/Applications:$PATH"
        echo -e "${GREEN}✓ Added Applications folder to PATH for current session${NC}"
    else
        # Try to install via Homebrew
        echo -e "${YELLOW}FFmpeg not found, attempting to install via Homebrew...${NC}"
        if command -v brew &> /dev/null; then
            if brew list ffmpeg &> /dev/null; then
                echo -e "${GREEN}✓ FFmpeg is already installed via Homebrew${NC}"
            else
                echo -e "${YELLOW}Installing FFmpeg via Homebrew...${NC}"
                brew install ffmpeg
                if [ $? -eq 0 ]; then
                    echo -e "${GREEN}✓ FFmpeg installed successfully via Homebrew${NC}"
                else
                    echo -e "${RED}✗ Failed to install FFmpeg via Homebrew${NC}"
                    echo -e "${YELLOW}You may need to install Homebrew first:${NC}"
                    echo -e "${BLUE}/bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"${NC}"
                fi
            fi
        else
            echo -e "${YELLOW}Homebrew not found. Installing...${NC}"
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            if [ $? -eq 0 ]; then
                echo -e "${GREEN}✓ Homebrew installed${NC}"
                echo -e "${YELLOW}Installing FFmpeg...${NC}"
                brew install ffmpeg
                if [ $? -eq 0 ]; then
                    echo -e "${GREEN}✓ FFmpeg installed successfully${NC}"
                else
                    echo -e "${RED}✗ Failed to install FFmpeg${NC}"
                fi
            else
                echo -e "${RED}✗ Failed to install Homebrew${NC}"
            fi
        fi
    fi
fi

# Fix 4: Test the app
echo -e "${YELLOW}Testing the app...${NC}"
"$APP_PATH/Contents/MacOS/MetadataGenerator" --help 2>/dev/null &
APP_PID=$!
sleep 2
if kill -0 $APP_PID 2>/dev/null; then
    echo -e "${GREEN}✓ App launches successfully${NC}"
    kill $APP_PID 2>/dev/null || true
else
    echo -e "${YELLOW}⚠ App may still have issues${NC}"
    echo -e "${YELLOW}Try running from Terminal to see error messages:${NC}"
    echo -e "${BLUE}$APP_PATH/Contents/MacOS/MetadataGenerator${NC}"
fi

echo -e "${GREEN}=== Quick fix complete! ===${NC}"
echo -e "${BLUE}Try launching MetadataGenerator now${NC}"
echo -e "${YELLOW}If it still doesn't work, run the full diagnostic:${NC}"
echo -e "${BLUE}./diagnose-app.sh${NC}" 