import jinja2
import argparse
































# Idk if you wanna use argparse but it's p easy to use.

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Creates a packageset dockerfile, by combining package templates together.')
    parser.add_argument('--base-image', dest='base_image', type=str, required=True)
    parser.add_argument('-p', '--package', action='append',  dest='packages', required=True)
    args = parser.parse_args()
    print(args.base_image)
    print(args.packages)
