import re

def read():
    rules = {}
    messages = []

    with open("input.txt") as fp:
        rules_done = False
        for line in fp.readlines():
            if len(line) <= 1:
                rules_done = True
            else:
                if rules_done:
                    messages.append(line[:-1])
                else:
                    rule_id, rule = re.split(": ", line[:-1])
                    rules[int(rule_id)] = rule

    print(f"{len(rules)} rules and {len(messages)} messages read")
    return rules, messages


def resolve(rules, rule_id):
    rule = rules[rule_id]
    resolved = ""
    for token in rule.split(" "):
        if token[0].isdigit():
            resolved += "(" + resolve(rules, int(token)) + ")"
        else:
            resolved += token
    return resolved


def run1():
    rules, messages = read()
    patt = "^" + resolve(rules, 0).replace('"', '') + "$"
    # print(patt)
    s = 0
    for msg in messages:
        if re.match(patt, msg):
            s += 1
    print(s)


def run2():
    rules, messages = read()
    rules[8] = '42 +'
    rules[11] = "42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 | 42 42 42 42 42 31 31 31 31 31"
    patt = "^" + resolve(rules, 0).replace('"', '') + "$"
    # print(patt)
    s = 0
    for msg in messages:
        if re.match(patt, msg):
            s += 1
    print(s)


if __name__ == "__main__":
    run1()
    run2()
