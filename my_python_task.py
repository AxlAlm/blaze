import uuid


def main():

    raise Exception("oh No!")

    file_name = f"test_data/{uuid.uuid4()}.txt"
    print(file_name)

    with open(file_name, "w") as f:
        f.write("hello world")

if __name__ ==  '__main__':
    main()