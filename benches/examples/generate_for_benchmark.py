import random
import string

# for tbox related tasks:
# generate tboxes with three parameters:
# size, size of conflicts, max lenght of chains

# for abox related tasks:
# clean tbox with two parameters:
# size, max length of chains
# dirty aboxes with two parameters:
# size, number of conflicts

# first utitlity functions for the creation of all files

symbol_types = ["concept", "role"]
begintag_symbol = "BEGINSYMBOL"
endtag_symbol = "ENDSYMBOL"
begintag_tbox = "BEGINTBOX"
endtag_tbox = "ENDTBOX"
begintag_abox = "BEGINABOX"
endtag_abox = "ENDABOX"

name_size_range = [i for i in range(5, 101)]
file_size_range = [10, 100, 1000, 10000, 100000, 1000000]


def generate_random_string(n):
    res = ''.join(random.choices(string.ascii_letters + string.digits, k=n))
    return res


def generate_random_symbol():
    symbol_size = random.choice(name_size_range)
    symbol_name = generate_random_string(symbol_size)
    symbol_type = random.choice(symbol_types)

    return ''.join([symbol_type, ' : ', symbol_name])


def generate_dict_of_symbols(n):
    d = {"concept": [], "role": []}

    for i in range(n):
        new_symbol = generate_random_symbol()
        if new_symbol.split(":")[0].strip() == "concept":
            d["concept"].append(new_symbol)
        else:
            d["role"].append(new_symbol)

    return d


def generate_tbi_side_from_symbol(symb, is_role):
    put_not = bool(random.getrandbits(1))
    put_not = False
    put_exists = bool(random.getrandbits(1))
    put_inv = bool(random.getrandbits(1))
    if is_role:
        if put_inv:
            symb = "INV " + symb
        if put_not:
            symb = "NOT " + symb
        if put_exists and (not put_not):
            symb = "EXISTS " + symb
    else:
        if put_not:
            symb = "NOT " + symb

    return symb


def generate_paths(dict_of_symbols, number_of_paths, max_lenght, add_conflict=False):
    paths = []
    for _ in range(number_of_paths):
        path_dict = {"tbis": [], "start": "", "end": "", "length": ""}
        already_used = []
        my_path = []
        type_used = random.choice(symbol_types)
        list = dict_of_symbols[type_used]

        start = random.choice(list)

        start = start.split(":")[1].strip()
        already_used.append(start)

        end = start
        lenght_of_path = random.randint(0, max_lenght)

        for _ in range(lenght_of_path):
            current_list = [x for x in list if x not in already_used]
            if len(my_path) == 0:
                end = random.choice(current_list)
                is_role = end.split(":")[0].strip() == "role"
                end = end.split(":")[1].strip()

                already_used.append(end)
                end = generate_tbi_side_from_symbol(end, is_role)

                my_path.append((start, end))
            else:
                # I don't care about performance here
                current_end = my_path[-1][1]
                new_end = random.choice(current_list)
                is_role = new_end.split(":")[0].strip() == "role"

                new_end = new_end.split(":")[1].strip()
                already_used.append(new_end)
                new_end = generate_tbi_side_from_symbol(new_end, is_role)

                my_path.append((current_end, new_end))
                end = new_end

        if add_conflict and len(my_path) != 0:
            put_not = bool(random.getrandbits(1))
            if put_not:
                s = my_path[0][0]
                e = my_path[-1][1]

                new_s = "NOT " + s
                my_path.append((e, new_s))

        path_dict["tbis"] = my_path
        path_dict["start"] = start
        path_dict["end"] = end
        path_dict["length"] = lenght_of_path

        paths.append(path_dict)

    return paths


def generate_random_tbox_file(number_of_symbols, number_of_tbis, max_length, add_conflicts):
    pass

def generate_random_symbol_file(size):
    res = begintag_symbol + "\n"

    for i in range(size):
        res += generate_random_symbol() + "\n"

    res += endtag_symbol + "\n"

    return res


def generate_and_write_symbols(size):
    res = generate_random_symbol_file(size)
    name = "symbols" + str(size)
    with open(name, 'w') as f:
        f.write(res)

    return 1


def pretty_print_list(l, sep):
    print(" " * sep, "[")
    for i in l:
        print(" "*(sep+1), i)
    print(" " * sep, "]")


def pretty_print_dict(d, sep=0):
    for key in d:
        value = d[key]

        print(" " * sep, "{")
        if isinstance(value, dict):
            sep += 2
            pretty_print_dict(value, sep)
        else:
            if isinstance(value, list):
                print(" "*sep, key, ":")
                pretty_print_list(value, sep+1)
            else:
                print(" "*sep, key, ":", value)
        print(" " * sep, "},")


if __name__ == "__main__":
    n = 20
    number_of_paths = 5
    max_length = 4
    symbol_d = generate_dict_of_symbols(n)

    paths = generate_paths(symbol_d, number_of_paths, max_length, add_conflict=True)

    pretty_print_dict(symbol_d)
    print("--------------------------")

    for p in paths:
        pretty_print_dict(p)
