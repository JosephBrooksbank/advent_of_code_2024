import math

def main():
    with open("sample") as f:
        lines = f.readlines()
        # before: after
        rules = {}
        read_all_rules = False
        valid_lines = []
        invalid_lines = []
        for line in lines:
            if line == "\n":
                read_all_rules = True
                print(rules)
                continue
            if read_all_rules:
                number_strings = line.split(",")
                numbers = []
                for number_string in number_strings:
                    numbers.append(int(number_string.strip()))

                if is_problem_line(numbers, rules):
                    invalid_lines.append(numbers)
                else:
                    valid_lines.append(numbers)

            else:
                before, after = line.split("|")
                before = int(before.strip())
                after = int(after.strip())
                if rules.get(before):
                    rules[before].append(after)
                else:
                    rules[before] = [after]
        total = 0
        for line in valid_lines:
            total += get_middle_value(line)

        print(total)

        fixed_total = 0
        for line in invalid_lines:
            fixed_line = fix_line(line, rules)
            print(fixed_line)
            fixed_total += get_middle_value(fixed_line)

        print(fixed_total)

def is_problem_line(line: [int], rules: dict):
    for i in range(len(line)):
        if rules.get(line[i]):
            must_appear_after = rules.get(line[i])
            for j in range(0, i):
                if line[j] in must_appear_after:
                    return True
    return False


def fix_line(line: [int], rules: dict):
    print("fixing line ", line)
    for i in range(len(line)):
        if rules.get(line[i]):
            must_appear_after = rules.get(line[i])
            for j in range(0, i):
                if line[j] in must_appear_after:
                    print("swapping ", line[j], " and ", line[i])
                    line[j], line[i] = line[i], line[j]
    return line


def get_middle_index(line: [int]):
    return math.floor(len(line)/2)

def get_middle_value(line: [int]):
    return line[get_middle_index(line)]

if __name__ == "__main__":
    main()