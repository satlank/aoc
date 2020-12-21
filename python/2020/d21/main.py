import re


def read(filename):
    rows = []
    with open(filename) as f:
        for line in f.readlines():
            m = re.match("(.*)\((.*)\)", line)
            ingredients = m.groups()[0].split()
            allergens = m.groups()[1][9:].split(', ')
            rows.append((ingredients, allergens))
    return rows


def get_allergen_map(food):
    all_allergens = {}
    for (ingredients, allergens) in food:
        for a in allergens:
            if a not in all_allergens:
                all_allergens[a] = set(ingredients)
            else:
                all_allergens[a] = all_allergens[a].intersection(set(ingredients))
    return all_allergens


def get_ingredient_map(food):
    all_ingredients = {}
    for (ingredients, allergens) in food:
        for i in ingredients:
            if i not in all_ingredients:
                all_ingredients[i] = set(allergens)
            else:
                all_ingredients[i] = all_ingredients[i].union(set(allergens))
    return all_ingredients


def get_food_by_allergens(food):
    tmp = {}
    for (i, (ingredients, allergens)) in enumerate(food):
        for a in allergens:
            if a not in tmp:
                tmp[a] = [i]
            else:
                tmp[a].append(i)
    return tmp


def get_food_by_ingredients(food):
    tmp = {}
    for (i, (ingredients, allergens)) in enumerate(food):
        for ing in ingredients:
            if ing not in tmp:
                tmp[ing] = [i]
            else:
                tmp[ing].append(i)
    return tmp


def reduce(a2i):
    resolved = set()
    tmp = a2i.copy()
    while len(resolved) != len(tmp.keys()):
        for k, v in tmp.items():
            if len(v) == 1:
                resolved = resolved.union(v)
            else:
                tmp[k] = v.difference(resolved)
    return tmp


def run(filename):
    food = read(filename)
    am = get_allergen_map(food)
    im = get_ingredient_map(food)
    fbi = get_food_by_ingredients(food)
    ingredients_with_allergens = set()
    for k, v in am.items():
        ingredients_with_allergens = ingredients_with_allergens.union(v)
    ingredients_without_allergens = set(im.keys()).difference(ingredients_with_allergens)
    cnt = 0
    for iwa in ingredients_without_allergens:
        cnt += len(fbi[iwa])
    print(f"Allergen-free ingredients appear {cnt} times")
    final_allergens = reduce(am)
    dangerlist = []
    for k in sorted(final_allergens.keys()):
        dangerlist.append(list(final_allergens[k])[0])
    print(f"Canonical dangerous ingredient list: {','.join(dangerlist)}")


if __name__ == "__main__":
    run('input.txt')
