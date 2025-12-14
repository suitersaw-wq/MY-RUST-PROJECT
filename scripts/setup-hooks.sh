#!/bin/sh

# Configure git to use our hooks directory
git config core.hooksPath .hooks

echo "Git hooks configured successfully!"
