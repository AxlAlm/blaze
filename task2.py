
import argparse
import json


def main(data:dict):
    print("HERE IS THE DATA", data)

if __name__ ==  '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("json_data", type=str)
    args = parser.parse_args()
    print("ARGS", args.json_data)
    main(json.loads(args.json_data))