#!/usr/bin/python

import argparse
import json
import os
import subprocess
import sys


def copy(package, lib, destination):
    finalDestination = os.path.join(destination, package)
    if os.path.exists(finalDestination):
        return # We don't want to over-write anything

    source = os.path.join(lib, package)
    if os.path.exists(source):
        subprocess.check_call(['cp', '-r', source, destination])

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--file', required=True, help='the file with json blob with the dependencies')
    parser.add_argument('--lib', required=True, help='the directory to look for the packages')
    parser.add_argument('--destination', required=True, help='the destination directory for the packages')
    args = parser.parse_args()


    if not os.path.isfile(args.file):
        # If there were no packages, this file will not exist
        return

    depObject = json.load(open(args.file))

    for package in depObject:
        copy(package, args.lib, args.destination)

        if isinstance(depObject[package], list):
            for dep in depObject[package]:
                copy(dep, args.lib, args.destination)
        elif depObject[package] is not None:
            copy(depObject[package], args.lib, args.destination)

if __name__ == '__main__':
    main()