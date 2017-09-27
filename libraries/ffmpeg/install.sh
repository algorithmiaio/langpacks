#!/bin/bash

set -e

add-apt-repository ppa:jonathonf/ffmpeg-3
apt-get update
apt-get install -y ffmpeg
