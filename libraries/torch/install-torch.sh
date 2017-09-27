#!/bin/bash

set -e

# When torch runs the install-deps script it has a crap-load of "sudo ..." commands
# Rather than do some weird sed-magic just install sudo here
apt-get update
apt-get install -y sudo

TORCH_DISTRO_COMMIT=f41316ea2895fd3462375c7c92350c646b25166b
git clone https://github.com/torch/distro.git /opt/torch --recursive \
    && cd /opt/torch\
    && git checkout "$TORCH_DISTRO_COMMIT" \
    && bash install-deps \
    && ./install.sh \

