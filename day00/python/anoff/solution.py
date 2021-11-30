print("hello world")


def get_message() -> str:
    return "hello world"


def star_1() -> None:
    print(get_message())


def star_2() -> None:
    pass


if __name__ == "__main__":
    star_1()
    star_2()
