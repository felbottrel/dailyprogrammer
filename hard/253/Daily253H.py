height = 10
width = 10
terminal = [list(' ' * width) for x in range(height)]
cursor = [0, 0]  # x, y
mode = 0  # 0 = no mode, 1 = insert, 2 = overwrite
 
def print_console():
    for i in range(height):
        print(''.join(terminal[i]))

def clr():
    global terminal
    terminal = [list(' ' * width) for x in range(height)]
   
def home():
    global cursor
    cursor = [0, 0]
   
def begin():
    cursor[1] = 0
   
def down():
    if cursor[0] < height-1:
        cursor[0] += 1
   
def up():
    if cursor[0] > 0:
        cursor[0] -= 1
 
def left():
    if cursor[1] > 0:
        cursor[1] -= 1
   
def right():
    if cursor[1] < width-1:
        cursor[1] += 1
 
def erase():
    for i in range(cursor[1], width):
        terminal[cursor[0]][i] = ' '
       
def insrt():
    global mode
    mode = 1
   
def ovrwrt():
    global mode
    mode = 2
   
def circ():
    write('^')
   
def mov(x, y):
    cursor[0] = x
    cursor[1] = y
   
def write(char):
    terminal[cursor[0]][cursor[1]] = char
    if mode == 1:
        if cursor[1] < width-1:
            cursor[1] += 1

switch = {
    0: clr,
    1: home,
    2: begin,
    3: down,
    4: up,
    5: left,
    6: right,
    7: erase,
    8: insrt,
    9: ovrwrt,
    10: circ
}

def do_cmd(op, char='', x=0, y=0):
    if op < 0:
        if op == -1:
            mov(x, y)
        else:
            write(char)
    else:
        func = switch.get(op)
        func()
 
def parse(inpt):
    state = 0
    i = 0
    while i < len(inpt):
        procs = True
        while procs:
            if state == 0:
                if inpt[i] == '^':
                    state = 1
                    i += 1
                else:
                    do_cmd(-2, inpt[i])
                    state = 0
                    procs = False
            elif state == 1:
                if inpt[i] == '^':
                    do_cmd(10)
                    state = 0
                    procs = False
                elif '0123456789'.find(inpt[i]) >= 0:

                    x = int(inpt[i])
                    i += 1
                    y = int(inpt[i])
                    do_cmd(-1, '', x, y)
                    state = 0
                    procs = False
                elif 'chbdulreio'.find(inpt[i]) >= 0:
                    do_cmd('chbdulreio'.find(inpt[i]))
                    state = 0
                    procs = False
        i += 1
   
inpt = '^h^c^iHello!^d^b===========^^'
parse(inpt)
print_console()
