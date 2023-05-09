
import argparse


def main(filename:str):
    with open(filename, "w") as f:
        f.write("hello world")

if __name__ ==  '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("filename")
    args = parser.parse_args()
    main(args.filename)