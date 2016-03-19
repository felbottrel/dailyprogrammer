cypher = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

def atbash(string):
    ret = ''
    for c in string:
        if cypher.find(c) >= 0:
            ret += cypher[ ( 25-cypher.find(c), -1*(27+cypher.find(c)) )[cypher.find(c) < 26] ]
        else:
            ret += c
    return ret

print(atbash("fooBAR"))