import sys
sys.path.append('.')
from Algorithm import configure
# Dont add stuff here, this is important.
if __name__ == "__main__":
    algo = configure()
    algo.run()
