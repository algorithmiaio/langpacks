#!/usr/bin/python
# rip - kinda like python's pip but for R

import sys
import os
import subprocess

def main():
    if len(sys.argv) != 2:
        print 'ERROR: Takes 1 the file name as the only argument'
        sys.exit(1)

    fileName = sys.argv[-1]

    if not os.path.isfile(fileName):
        print 'ERROR: The package list file does not exist'
        sys.exit(1)

    rscript = ['library("pacman")']
    with open(fileName) as f:
        for line in f.readlines():
            line = line.strip()

            # Skip comments
            if line.startswith('#') or len(line) == 0:
                continue

            tokens = line.split()
            if len(tokens) == 1: # installs the latest package from CRAN
                rscript.append('install.packages("{}")'.format(line))
            elif tokens[0] == '-t' and len(tokens) == 2: # installs a specific archive from CRAN (most likely)
                rscript.append('install.packages("{}", repos=NULL, type="source")'.format(tokens[1]))
            elif tokens[0] == '-g' and len(tokens) == 2: # installs from github of the form: username/repo[/subdir][@ref|#pull]
                rscript.append('p_install_gh(c("{}"))'.format(tokens[1]))
            else:
                print 'Unexpected line: "{}"'.format(line)
                sys.exit(1)

    if len(rscript) == 1:
        print 'There is nothing to install'
        return

    commandArgs = ['Rscript', '-e', '; '.join(rscript)]
    print 'Running: {}'.format(' '.join(commandArgs))
    subprocess.check_call(commandArgs)


if __name__ == '__main__':
    main()