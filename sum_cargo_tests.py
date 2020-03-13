#!/usr/bin/env python3

# A script to add summaries of test results to the output of 'cargo test'

import re
import sys


def main():
    passedSum = 0
    failedSum = 0
    ignoredSum = 0
    measuredSum = 0
    filteredOutSum = 0

    for line in sys.stdin:
        sys.stdout.write(line)
        result = re.match(
            r'test result: .*\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out',
            line)
        if not result:
            continue
        
        (passed, failed, ignored, measured, filteredOut) = result.groups()
        passedSum += int(passed)
        failedSum += int(failed)
        ignoredSum += int(ignored)
        measuredSum += int(measured)
        filteredOutSum += int(filteredOut)
        print(f'            sum: {passedSum} passed; {failedSum} failed; {ignoredSum} ignored; {measuredSum} measured; {filteredOutSum} filtered out')


if __name__ == "__main__":
    main()
