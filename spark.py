import json
import uuid


def main():
    for _  in range(100):
        print(f"examples/test_data/{uuid.uuid4()}.txt")

if __name__ ==  '__main__':
    main()