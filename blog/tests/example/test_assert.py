def test_sum():
    assert sum([1, 2, 3]) == 6, "Should be True"


def test_sum_tuple():
    assert sum((1, 2, 2)) == 6, "Should be False and show this message."


if __name__ == "__main__":
    test_sum()
    test_sum_tuple()
    print("Shouldn't pass")
