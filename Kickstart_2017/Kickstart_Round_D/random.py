import hashlib
line = "password"
line.encode('utf-8')
print(line)
m = hashlib.md5(line)
ord(m[5:6]) * ord(m[10:11]) * m[3:4] - (ord("F")*100+ord("K"))