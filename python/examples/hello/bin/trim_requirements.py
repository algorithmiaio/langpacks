 #!/usr/bin/env python2

import argparse
import pkg_resources

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--input', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--dependencies', required=True)
    args = parser.parse_args()

    required_dependencies = {}
    with open(args.input) as f:
        lines = f.readlines()
        for line in lines:
            line = line.strip()
            if '==' in line:
                name, _, version = line.partition('==')
                name = name.strip()
                version = version.split().pop().strip()
                required_dependencies[name] = ['==', version]
            elif '>=' in line:
                name, _, version = line.partition('>=')
                name = name.strip()
                version = version.split().pop().strip()
                required_dependencies[name] = ['>=', version]
            else:
                name = line.split().pop()
                required_dependencies[name] = []

    env = pkg_resources.Environment([args.dependencies])
    for project in env:
        for dist in env[project]:
            if dist.project_name in required_dependencies:
                version = required_dependencies[dist.project_name]
                if len(version) == 0 or \
                    version[0] == '==' and version[1] == dist.version or \
                    version[0] == '>=' and checkVersion(dist.version, version[1]):

                    # We have a version that works
                    del required_dependencies[dist.project_name]
                    continue
                else:
                    raise Exception("Downgrading from {}=={}".format(dist.project_name, dist.version))

    # Whatever is left in required_dependencies is now something we are lacking
    with open(args.output, 'w') as f:
        for lib in required_dependencies.keys():
            f.write("{}{}\n".format(lib, ''.join(required_dependencies[lib])))


def checkVersion(current, desired):
    current = current.split('.')
    desired = desired.split('.')

    if len(current) != len(desired):
        return False

    for i in range(len(desired)):
        c = int(current[i])
        d = int(desired[i])

        if c < d:
            return False
    return True

if __name__ == '__main__':
    main()
