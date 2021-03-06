#!/usr/bin/python
# rip - kinda like python's pip but for R

import argparse
import os
import subprocess
import sys

def installLatestCranIfNecessary(package):
    return 'if (!(("{package}"  %in% installed.packages()[, c("Package")]) && (available.packages()["{package}",2] == packageVersion("{package}")))) {{ install.packages("{package}") }}; utils::packageVersion("{package}")'.format(package=package)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--file', required=True, help='the file with the list of packages')
    parser.add_argument('--output-dependencies', help='file with a json blob storing the package dependencies')
    parser.add_argument('--cran-latest', action='store_true', help='only install latest CRAN packages')
    parser.add_argument('--skip-cran-latest', action='store_true', help='only install packages that are not latest CRAN packages')
    parser.add_argument('--library', help="used with --output-dependencies to determine which library to look at")
    args = parser.parse_args()


    if not os.path.isfile(args.file):
        print('ERROR: The package list file does not exist')
        sys.exit(1)

    rscript = ['library("pacman")']
    normalPackages = []
    with open(args.file) as f:
        for line in f.readlines():
            line = line.strip()

            # Skip comments
            if line.startswith('#') or len(line) == 0:
                continue

            tokens = line.split()
            if len(tokens) == 1: # installs the latest package from CRAN
                if not args.skip_cran_latest:
                    rscript.append(installLatestCranIfNecessary(line))
                    normalPackages.append(line)
            elif tokens[0] == '-t' and len(tokens) == 2: # installs a specific archive from CRAN (most likely)
                if not args.cran_latest:
                    http_url = tokens[1]  # Something like https://cran.r-project.org/src/contrib/R6_2.2.2.tar.gz
                    gzip_package_name = http_url.split('/')[-1] # R6_2.2.2.tar.gz
                    if '_' in gzip_package_name: # Scanning CRAN, R packages don't _ in the name, but version needs to be stripped
                        package_name = '_'.join(gzip_package_name.split('_')[:-1])  # Just in case some package _does_ have an _
                    else:
                        package_name = gzip_package_name
                    package_name = package_name.replace('.tar.gz', '') # remove the .tar.gz if it exist
                    rscript.append('install.packages("{package}", repos=NULL, type="source"); utils::packageVersion("{package_name}")'.format(package=tokens[1], package_name=package_name))
            elif tokens[0] == '-g' and len(tokens) == 2: # installs from github of the form: username/repo[/subdir][@ref|#pull]
                if not args.cran_latest:
                    rscript.append('p_install_gh(c("{package}"));'.format(package=tokens[1]))
            elif tokens[0] == '-e' and len(line) > 3:
                if not args.cran_latest:
                    rscript.append(line[3:])
            else:
                print('Unexpected line: "{}"'.format(line))
                sys.exit(1)

    if len(rscript) == 1:
        print('There is nothing to install')
        return

    commandArgs = ['Rscript', '-e', '; '.join(rscript)]
    print('Running: {}'.format(' '.join(commandArgs)))
    subprocess.check_call(commandArgs)

    if args.output_dependencies is not None:
        if args.library is not None:
            subprocess.check_call(['Rscript', '-e', 'library(tools); library(rjson); packages <- rownames(installed.packages("{}")); if (!is.null(packages)){{ writeLines(toJSON(package_dependencies(packages, recursive=TRUE)), con="{}") }}'.format(args.library, args.output_dependencies)])
        elif len(normalPackages) > 0:
            packagesStr = ', '.join('"{}"'.format(p) for p in normalPackages)
            subprocess.check_call(['Rscript', '-e', 'library(tools); library(rjson); writeLines(toJSON(package_dependencies(c({}), recursive=TRUE)), con="{}")'.format(packagesStr, args.output_dependencies)])



if __name__ == '__main__':
    main()
