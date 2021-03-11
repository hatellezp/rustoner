import string
import random

symbol_types = ["concept", "role"]
begintag = "BEGINSYMBOL"
endtag = "ENDSYMBOL"

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


def generate_random_symbol_file(size):
    res = begintag + "\n"

    for i in range(size):
        res += generate_random_symbol() + "\n"
    
    res += endtag + "\n"

    return res

def generate_and_write_symbols(size):
    res = generate_random_symbol_file(size)
    name = "symbols" + str(size)
    with open(name, 'w') as f:
        f.write(res)

    return 1

if __name__ == '__main__':

    for size in file_size_range:
        generate_and_write_symbols(size)

        print("created symbols{}".format(size))