def gen_primes():
    """ Generate an infinite sequence of prime numbers.
    """
    D = {}
    q = 2

    while True:
        if q not in D:
            yield q
            D[q * q] = [q]
        else:
            for p in D[q]:
                D.setdefault(p + q, []).append(p)
            del D[q]

        q += 1

def id_to_kilo(n):
    mult = 1
    count = 0
    for i in gen_primes():
        next_mult = mult* i
        if next_mult>n:
            return count
        mult = next_mult
        count+=1

print(id_to_kilo(30))
