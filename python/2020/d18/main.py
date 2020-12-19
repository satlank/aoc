ops = [1, '+', 3, '*', '(', 1, '+', 1, ')']
line = '1 + 3 * (1 + 1)'


def calc(ops):
    #print("Working on:", ops)
    res = None
    cur_op = None
    i = 0
    while i < len(ops):
        val = None
        #print(ops[i], res, cur_op, val)
        if ops[i] == '(':
            ob = 1
            j = i + 1
            while ob >= 1:
                if ops[j] == '(':
                    ob += 1
                elif ops[j] == ')':
                    ob -= 1
                j += 1
            val = calc(ops[i+1:j-1])
            i = j
        elif ops[i] in ('+', '*'):
            cur_op = ops[i]
            val = None
            i += 1
        else:
            val = ops[i]
            i += 1
        if val:
            if res:
                if cur_op == '+':
                    res += val
                else:
                    res *= val
            else:
                res = val
        #print("---->", res)
    return res

def tokenize(line):
    toks = []
    for x in line.split():
        if x.startswith('('):
            while x.startswith('('):
                toks.append('(')
                x = x[1:]
            toks.append(int(x))
        elif x.endswith(')'):
            tmp = []
            while x.endswith(')'):
                tmp.append(')')
                x = x[:-1]
            toks.append(int(x))
            toks = toks + tmp
        elif x in ('+', '*'):
            toks.append(x)
        else:
            toks.append(int(x))
    return toks


def group(toks):
    groups = []
    i = 0
    if len(toks) <= 3:
        return toks
    while i < len(toks):
        if isinstance(toks[i], int):
            groups.append(toks[i])
            i += 1
        elif isinstance(toks[i], list):
            groups.append(toks[i] if len(toks[i]) > 1 else toks[1][0])
            i += 1
        elif toks[i] in ('+', '*'):
            groups.append(toks[i])
            i += 1
        elif toks[i] == '(':
            depth = 1
            j = i + 1
            while depth >= 1:
                if toks[j] == '(':
                    depth += 1
                elif toks[j] == ')':
                    depth -= 1
                j += 1
            groups.append(group(toks[i+1:j-1]))
            i = j
    while len(groups) > 3:
        i = 0
        while i < len(groups):
            if groups[i] == "*":
                groups = [
                    group(groups[:i]) if i > 1 else groups[0],
                    groups[i],
                    group(groups[i+1:]) if i < len(groups) - 1 else groups[-1]
                ]
                break
            i += 1
        if len(groups) != 3:
            i = 0
            while i < len(groups):
                if groups[i] == "+":
                    groups = [
                        group(groups[:i]) if i > 1 else groups[0],
                        groups[i],
                        group(groups[i+1:]) if i < len(groups) - 1 else groups[-1]
                    ]
                    break
                i += 1
    return groups


def calc2(groups):
    if not isinstance(groups, list):
        return groups
    if len(groups) == 1:
        return calc2(groups[0])
    if groups[1] == '*':
        return calc2(groups[0]) * calc2(groups[2])
    if groups[1] == "+":
        return calc2(groups[0]) + calc2(groups[2])


LINES = [
    "(1 + 3)",
    "2 * 3 + (4 * 5)",
    "5 + (8 * 3 + 9 + 3 * 4 * 3)",
    "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
    "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
]


print(calc(tokenize(line)))
for line in LINES:
    print(line, "=", calc(tokenize(line)))

with open("input.txt") as f:
    s = 0
    for line in f.readlines():
        s += calc(tokenize(line))
print("RESULT:", s)

print("-------------")

for line in LINES:
    g = group(tokenize(line))
    print(g)
    print(line, "=", calc2(g))

with open("input.txt") as f:
    s = 0
    for line in f.readlines():
        s += calc2(group(tokenize(line)))
print("RESULT:", s)
