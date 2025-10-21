#!/bin/bash

# Build script for Virtua Grotesk fonts using fontc
# https://github.com/googlefonts/fontc

set -e

echo "Building Virtua Grotesk fonts with fontc..."

# Create fonts directory if it doesn't exist
mkdir -p fonts

# Build the font from UFO sources
fontc sources/virtua-grotesk-regular.ufo -o fonts/VirtuaGrotesk-Regular.ttf

echo "Build complete! Fonts are in the fonts/ directory"
