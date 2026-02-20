#!/usr/bin/env bash
set -euo pipefail
clang-format -i $(rg --files core_cpp bindings | rg '\\.(h|hpp|c|cc|cpp)$')
(cd app_flutter && dart format lib test)
