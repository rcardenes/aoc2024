#!/usr/bin/env python

import re
import sys

rule = re.compile(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")

text = ''.join(line.strip() for line in sys.stdin.read().split('\n'))

found = rule.findall(text)
enabled = True
value = 0
for element in found:
    if element == 'do()':
        enabled = True
    elif element == "don't()":
        enabled = False
    elif enabled:
        parts = element.split(',')
        n1 = int(parts[0].split('(')[1])
        n2 = int(parts[1][:-1])
        value += n1 * n2
print(value)

