
import os

import requests


if __name__ == '__main__':
    r = requests.get("https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt")

    path = os.path.join(os.path.dirname(__file__), "assets", "unicode", "DerivedCoreProperties.txt")
    with open(path, 'wb') as f:
        f.write(r.content())
