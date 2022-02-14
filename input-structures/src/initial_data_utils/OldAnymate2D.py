import itertools
import ast
import re
%matplotlib inline
import numpy as np
from IPython.display import HTML
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from pprint import pprint
# First set up the figure, the axis, and the plot element we want to animate
fig, ax = plt.subplots()
timetext = ax.text(0.6, 70, '')
plotlays, plotcols = [3,6], ["black","red"]

lines = []
for index, lay in enumerate(plotlays):
    lobj = ax.plot([], [], lw=2, color=plotcols[index])[0]
    lines.append(lobj)
try:
    fp =  open(r"C:\Users\2020\RUSTFirstOrderEquation\src\treated_datas_0\parameters_0.txt", 'r')
except IOError:
    print ("No file")
l = [line.strip() for line in fp]
pprint(l)
dl=0.0
dr=0.0
for ll in l:
    if "Margin domain: " in ll:
        k = [float(el) for el in ll[len('margin_domain: '):-1].replace('(','').replace(')','').replace("\\","").split(',')]
        print("Left bound", k[0])
        dl=k[0]
        print("Right bound", k[1])
        dr=k[1]
        ax.set_xlim(( k[0]-0.2, k[1]+0.2))
    if "Initial conditions:" in ll:
        k = [float(el) for el in ll[len("Initial conditions: "):-1].replace('(','').replace("Some","").replace(')','').replace("\\","").split(',')]
        print("", k[0])
        print("", k[1])
        print("", k[2])
        ax.set_ylim((-1, k[2]))#k[1]
    fp.close()

size1 = 0
size2 = 0
p = re.compile('\[[0\\n \,\.]*\s*\]')

def is_zeros_only(line):
    if re.match(p, line) is not None:
        return True #not only zeros  
    return False

with open(r"C:\Users\2020\RUSTFirstOrderEquation\src\treated_datas_0\to_python_021.txt") as file:
    new_line = []
    for i, line in enumerate(file):
        print("Line number: ", i)
        for chars in line:
            new_line.append(chars)
        print(line)
        print(new_line)
        line_ = line[:-1]
        print(type(line))
        if is_zeros_only(line):
            break
            size1= i
            size2= len(line)
            print("size of file was: ", len(list(file)))
            print(len(line))
            break 
        else:
            print(line)
            continue    
    new = np.ndarray(shape=(size1, size2), dtype=float)
    print(new,"len of lines: ", len(new[0]))
    for i, line in enumerate(file):
        new[i] = np.fromstring(line, dtype = float, sep = ',')
        print(new[i])
def init():
    for line in lines:
        line.set_data([] , [])
    return lines

def animate(i):
    timetext.set_text(i+1)
    lz= len(new[i])#zz[i]
    print("Frames: ", lz)
    x = np.linspace(dl, dr, lz)
    #print('',x)
    y = zz[i]
    line.set_data(x, y)
    return (line,)
writergif = animation.PillowWriter()#fps=30 
writervideo = animation.FFMpegWriter() 
anim = animation.FuncAnimation(fig, animate, init_func=init,
                               frames=len(zz), interval=20, blit=True)
HTML(anim.to_html5_video())
import numpy
from graphics import *
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from IPython.display import HTML
from pprint import pprint
from celluloid import Camera

npdata = numpy.random.randint(100, size=(5,6,12))
plotlays, plotcols = [3,6], ["black","red"]

fig = plt.figure()
ax = plt.axes(xlim=(0, numpy.shape(npdata)[0]), ylim=(0, numpy.max(npdata)))
timetext = ax.text(0.6, 70, '')

lines = []
for index, lay in enumerate(plotlays):
    lobj = ax.plot([],[],lw=2,color=plotcols[index])[0]
    lines.append(lobj)

def init():
    for line in lines:
        line.set_data([],[])
    return lines

def animate(i):
    timetext.set_text(i+1)
    x = numpy.array(range(1,npdata.shape[0]+1))
    for lnum, line in enumerate(lines):
        line.set_data(x, npdata[:, plotlays[lnum]-1, i])
    print(lines)
    print("Text: ", timetext)
    return tuple(lines) + (timetext,)

anim = animation.FuncAnimation(fig, animate, init_func=init,
                               frames=numpy.shape(npdata)[1], interval=100, blit=True)

plt.show()
HTML(anim.to_html5_video())
#print(zz)
#type(zz)
#global i_y = np.linspace(0, yy.len(),yy.len()+2)
# initialization function: plot the background of each frame
def init():
    line.set_data([], [])
    return (line,)


def animate(i):
    print(".......")
    print("y coordinate: ", new[i])
    lz= len(new[i])#zz[i]
    print("Frames: ", lz)
    x = np.linspace(dl, dr, lz)#print_npy+1 instead last digit
    #print('', x)
    #pprint(len(x))
    #zz[i]
    print(".......")
    line.set_data(x, new[i])
    return (line,)

# call the animator. blit=True means only re-draw the parts that have changed.
anim = animation.FuncAnimation(fig, animate, init_func=init,
                               frames=len(new), interval=30, blit=True)
writergif = animation.PillowWriter()#fps=30 
writervideo = animation.FFMpegWriter() 

HTML(anim.to_html5_video())
anim.save( r'C:\Users\2020\RUSTFirstOrderEquation\src\treated_datas_0\anim_012.gif', writer=writergif, fps=None, dpi=None, codec=None, bitrate=None, extra_args=None,
             metadata=None, extra_anim=None)
