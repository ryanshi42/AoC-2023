from sympy import symbols, solve

# This is code taken off GitHub, and not my work. This is the solution that worked for my input, it seems that the Rust solutions
# suffer from some floating point errors.
# https://topaz.github.io/paste/#XQAAAQAiBQAAAAAAAAAzHIoib6qwDfWv9YL2iyT0oyFsOL00fu/fAxr38eU00u5xJInC2pv+zhMOr9Z5v6wEmlkCXBg3Zb6IoH049og896ZmILSEl1lcCq+8MDtxD54QTySnUwlez43Bbr20uK5MQY84xhPNa4H9URi9lCXOkVN3gFfR/Ayla8eEbWDhnaXWml5FhgkfOhiXYX9kgWZYt5oCTiy6HUt3BqEJGPIaLScKn+W/zBXgds3PZPHvt5/bbv/R6u3ftckigtXkk+8Bgbup58LO9Y/mw+uPOuyXHRfeQAaWichFd8uHG92/EwfZtCXrWHQPf42PEETun0oXDiS+WJqnN5nn13g6V+CLXtNK6ig8xFrmLnv8zJ+ozGbqWqVWYjzNqJbTQ2CtWdve0HRw9KN30G1ZEb7quF/Fka6xo1RkGP4YkV+/A4H9JIrU3ySzbE9NfajYtwVSqI2vOWmKIiZBVGekRrNAwsCg0hc2a3SPSk0G8knA0xhsYdeJTwkVRbiEkitMv167KCYN57ilATH2pxWU6hbn1Tz/NDq2L2Xw9osVppBeW+45h4FPgf/PSDONOxMAeEQVZbGveTEFNmu3vvgAeuuKy22ZcQLNq8zj0FCCYqTyB9bdKSlcJQcyYbsxHyiqU2EAldhIYpFqyu/pJ9jB4mZOpzmmUimPBFEk7mMcCOhK9TnclMytBPAO4def1tFhlqfov1clPa4FsPaJqXI+VhxSsLX01eC1WbZ9sDfbh1rgl8jGelAj/9xBoUA=

hails = []
for line in open("../aoc_2023/input/2023/day24.txt").readlines()[:3]:
    line = line.strip().split(" @ ")
    pos = [int(i) for i in line[0].split(", ")]
    vel = [int(i) for i in line[1].split(", ")]
    hails.append((pos, vel))
    
X1, V1 = hails[0]
X2, V2 = hails[1]
X3, V3 = hails[2]

# components of hail position vectors - known parameters
x1, y1, z1 = X1
x2, y2, z2 = X2
x3, y3, z3 = X3
# components of hail velocity vectors - known parameters
vx1, vy1, vz1 = V1
vx2, vy2, vz2 = V2
vx3, vy3, vz3 = V3

# position vector components - unknowns
x = symbols('x')
y = symbols('y')
z = symbols('z')
# velocity vector components - unknowns
vx = symbols('vx')
vy = symbols('vy')
vz = symbols('vz')

# equations for the cross products being the null vector
equations = [
    # first hail
    (y1-y)*(vz1-vz)-(z1-z)*(vy1-vy), 
    (z1-z)*(vx1-vx)-(x1-x)*(vz1-vz), 
    (x1-x)*(vy1-vy)-(y1-y)*(vx1-vx),
    
    # second hail
    (y2-y)*(vz2-vz)-(z2-z)*(vy2-vy), 
    (z2-z)*(vx2-vx)-(x2-x)*(vz2-vz), 
    (x2-x)*(vy2-vy)-(y2-y)*(vx2-vx),
    
    # third hail
    (y3-y)*(vz3-vz)-(z3-z)*(vy3-vy), 
    (z3-z)*(vx3-vx)-(x3-x)*(vz3-vz), 
    (x3-x)*(vy3-vy)-(y3-y)*(vx3-vx)
]

solution = solve(equations, [x, y, z, vx, vy, vz], dict=True)[0]
print(solution[x] + solution[y] + solution[z])