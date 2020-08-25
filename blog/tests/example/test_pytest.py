# $pytest to test all files that it finds testable
# $pytest <filename> to test a file. For example, $pytest test_pytest.py

from termcolor import colored
import time

def test_sum():
    message = "Shoud be True."
    colored_message = colored(message, "yellow", attrs=["bold"])
    assert sum([1, 2, 3]) == 6, colored_message

def test_sum_tuple():
    message = "Should be False and show this message."
    colored_message = colored(message, "yellow", attrs=["bold"])
    assert sum((1, 2, 3)) == 6, colored_message

# You don't have to import benchamark here
# https://github.com/ionelmc/pytest-benchmark/

def test_pytest_time_plugin(duration=0.00000000000000000000000001):
    """
    Function that needs some serious benchmarking.
    """
    time.sleep(duration)
    # You may return anything you want, like the result of a computation
    return 123

def test_my_stuff(benchmark):
    # benchmark something
    result = benchmark(test_pytest_time_plugin)

    # Extra code, to verify that the run completed correctly.
    # Sometimes you may want to check the result, fast functions
    # are no good if they return incorrect results :-)
    assert result == 123
