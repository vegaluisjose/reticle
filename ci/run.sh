#!/bin/bash

set -eo pipefail

# create a directory for cargo registry, so
# dependencies can be cached when running rust
# inside docker
mkdir -p $PWD/.cargo/registry

# create virtual environment
python3 -m venv venv
source venv/bin/activate

# install python dependencies
pip install wheel
pip install pytest==6.0.1
pip install pytest-pycodestyle==2.2.0

# run ci tests
pytest --pycodestyle -x -v ci
pytest -x -v ci/run.py --use-docker

# deactivate virtual environment
deactivate
