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

    rscript = []
    with open(fileName) as f:
        for line in f.readlines():
            line = line.strip()

            # Skip comments
            if line.startswith('#') or len(line) == 0:
                continue

            tokens = line.split()
            if len(tokens) == 1:
                rscript.append('install.packages("{}")'.format(line))
            elif tokens[0] == '-t' and len(tokens) == 2:
                rscript.append('install.packages("{}", repos=NULL, type="source")'.format(tokens[1]))
            elif tokens[0] == '-g' and len(tokens) == 4:
                rscript.append('p_install_gh(c("{}", "{}", "{}"))'.format(tokens[1], tokens[2], tokens[3]))
            else:
                print 'Unexpected line: "{}"'.format(line)
                sys.exit(1)

    if len(rscript) == 0:
        print 'There is nothing to install'
        return

    commandArgs = ['Rscript', '-e', '; '.join(rscript)]
    print 'Running: {}'.format(' '.join(commandArgs))
    subprocess.check_call(commandArgs)


if __name__ == '__main__':
    main()