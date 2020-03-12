# https://realpython.com/python-testing/

import unittest as tester
from termcolor import colored

# How to test it?
# 1. $python unittest_example.py 
# 2. $cp unittest_example.py test_unittest.py && pip install nose2 && python -m nose2
# Refer to https://github.com/nose-devs/nose2

class TestSum(tester.TestCase):

    def test_sum(self):
        message = "Should be True"
        colored_message = colored(message, "yellow", attrs=["bold"])
        self.assertEqual(sum([1, 2, 3]), 6, colored_message)

    def test_sum_tuple(self):
        message = "Should be False and show this message."
        colored_message = colored(message, "yellow", attrs=["bold"])
        self.assertEqual(sum((1, 2, 2)), 6, colored_message)

if __name__ == "__main__":
    tester.main()
