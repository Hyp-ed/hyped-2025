#!/bin/bash
set -e # Exit with nonzero exit code if anything fails

# Use docker env files
cp /usr/src/app/packages/server/.env.docker /usr/src/app/packages/server/.env

cp /usr/src/app/packages/ui/.env.docker /usr/src/app/packages/ui/.env

pnpm run $PNPM_SCRIPT
