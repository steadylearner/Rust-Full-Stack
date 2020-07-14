def test_print():
    print("Learn how to build a full stack web app with Steadylearner")

if __name__ == "__main__":
    import timeit
    print(timeit.timeit("test_print()", setup="from __main__ import test_print", number=10))


