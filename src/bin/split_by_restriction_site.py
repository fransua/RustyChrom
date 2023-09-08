import sys

fnam = sys.argv[1]
renz = sys.argv[2]

cutoff = 20
fh = open(fnam)
out = open("foo_py.txt", "w")

for line in fh:
    h = line
    s = next(fh).strip()
    _ = next(fh)
    q = next(fh)
    try:
        i = s.index(renz)
    except ValueError:
        continue
    s1  = s[:i]
    if len(s1) > cutoff:
        out.write(h)
        out.write(f"{s1}\n")
        out.write("+\n")
        out.write(f"{q[:i]}\n")
    s2  = s[i:]
    if len(s2) > cutoff:
        out.write(h)
        out.write(f"{s2}\n")
        out.write("+\n")
        out.write(q[i:])
out.close()
    