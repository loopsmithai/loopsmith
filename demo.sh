#!/bin/bash
# Demo: run `smith init` against a local Gitea instance.
#
# Gitea does not implement the GitHub App manifest flow, so this exercises the
# PAT-auth / org-select / repo-select path only. Generate a token in your local
# Gitea (Settings -> Applications) and export it before running.
#
#   export GITEA_TOKEN=<your local gitea token>

: "${GITEA_TOKEN:?Set GITEA_TOKEN to a token from your local Gitea instance}"

echo "Run:"
echo "  SMITH_GITHUB_API_BASE=http://localhost:3000/api/v1 \\"
echo "  SMITH_GITHUB_WEB_BASE=http://localhost:3000 \\"
echo "  cargo run --bin smith -- init"
echo ""
echo "Paste \$GITEA_TOKEN when prompted."
