#!/usr/bin/env bash

set -euxo pipefail

git subtree pull --prefix themes/terminal https://github.com/panr/hugo-theme-terminal.git master --squash