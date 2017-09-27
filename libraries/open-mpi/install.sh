#!/bin/bash

set -e

OPENMPI_VERSION=1.10.3
wget -q -O - https://www.open-mpi.org/software/ompi/v1.10/downloads/openmpi-${OPENMPI_VERSION}.tar.gz | tar -xzf -
cd openmpi-${OPENMPI_VERSION}
./configure --prefix=/usr/local/mpi
make -j"$(nproc)" install
rm -rf /openmpi-${OPENMPI_VERSION}
