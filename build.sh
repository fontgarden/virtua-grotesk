#!/bin/bash

# Build script for Virtua Grotesk fonts using fontc
# https://github.com/googlefonts/fontc

set -e

echo "Building Virtua Grotesk fonts with fontc..."

# Create fonts directory if it doesn't exist
mkdir -p fonts

# Build variable font and all instances from designspace
echo "Building variable font and instances..."
fontc sources/VirtuaGrotesk.designspace -o fonts/

echo "Build complete! Fonts are in the fonts/ directory"
echo "- Variable font: VirtuaGrotesk[wght].ttf"
echo "- Static instances: VirtuaGrotesk-Regular.ttf, VirtuaGrotesk-Medium.ttf, VirtuaGrotesk-SemiBold.ttf, VirtuaGrotesk-Bold.ttf"
