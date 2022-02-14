import itertools
import ast
%matplotlib inline
import math
import numpy as np
import os,re
from IPython.display import HTML
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from pprint import pprint
import pandas as pd
from pathlib import Path
#from shutil import copyfile
import shutil
# First set up the figure, the axis, and the plot element we want to animate
fig, ax = plt.subplots()
#_____________________Processing datas___________________________________#
try:
    fp =  open(r"C:\Users\2020\RUSTprojects\trburcor\src\treated_datas_0\parameters_nf0.txt", 'r')
except IOError:
    print ("No file")
l = [line.strip() for line in fp]
pprint(l)
dl=0.0
dr=0.0
equation_type = 0
for ll in l:
    if "Initial type" in ll:
        res = re.sub('[^0-9]+', '', ll)
        equation_type = float(res)
        print("equation_type", equation_type)
    if "Margin domain: " in ll:
        k = [float(el) for el in ll[len('margin_domain: '):-1].replace('(','').replace(')','').replace("\\","").split(',')]
        print("Left bound", k[0])
        dl=k[0]
        print("Right bound", k[1])
        dr=k[1]
        ax.set_xlim(( k[0]-0.2, k[1]+0.2))
            #ax[1].set_xlim(( k[0]-0.2, k[1]+0.2))
    if "Initial conditions:" in ll:
        k = [float(el) for el in ll[len("Initial conditions: "):-1].replace('(','').replace("Some","").replace(')','').replace("\\","").split(',')]
        print("Center/MatExpect", k[0])
        print("Height/Dispersion", k[1])
        print("Width", k[2])
        if equation_type == 0 or equation_type == 1:
            ax.set_ylim((0, k[2]))#k[1]
        #ax[1].set_ylim((-1, k[2]))
        if equation_type == 2:
            ax.set_ylim((0, 0.002))#k[1]
        if equation_type == 3 or equation_type == 4:
            ax.set_ylim((-1, k[2]))#k[1]
#________________________________________________________________________#
sizes = [[],[]]
size = []
file_num = 0
euclid_norm = 0.0
uniform_norm = 0.0
temp_dif_colomns = 0.0
arrays = [[],[]]
dif = open(("../src/differential_errors{}.txt").format(file_num), 'w')
array_path = r"C:\Users\2020\RUSTprojects\trburcor\src\treated_datas_0\paraview_datas"
for i,entry in enumerate(os.scandir(array_path)):
    f = os.path.join(array_path, entry)
    if os.path.isfile(f) and entry.name.endswith('.txt'):
        #print(f)
        df = pd.read_csv(f, delimiter = ",")
        print(all(df["numv"].notnull()))
        if any(df["numv"].notnull()):
            #print("second condition: ", all(x !=0 for x in df[" exv"]) and all(x!=0 for x in df[" numv"]))
            print("Any: ",any(x for x in df["numv"]))
            if any(x for x in df["exv"]) or any(x for x in df["numv"]):
                #Calculate norms
                max_ind = (df["exv"] - df["numv"]).idxmax()
                print("Maximum differece with exact and numeric solutions in raw: ",
                    max_ind)
                #x   exv   numv
                print(df.iloc[max_ind])
                uniform_norm = abs(df["exv"].iloc[max_ind] - df["numv"].iloc[max_ind])
                print("So this maximum = ", uniform_norm)
                #Then search euclid norm = sqrt(Sum_k (Unum_k - Uexact_k)^2)
                print("Sum is- ", pow(df["exv"] - df["numv"], 2).sum())#axis=0
                euclid_norm = math.sqrt(pow(df["exv"] - df["numv"], 2).sum())
                print("So euclid norm is ", euclid_norm)
                #Then write in file
                print(("uniform norm in {} txt file: ").format(i), uniform_norm, file = dif)
                print(("euclid norm: in {} txt file: ").format(i), euclid_norm, file = dif)
                #then as usual
                print("Lenght of column: ", len(df["numv"]))
                sizes[0].append(len(df["numv"]))
                sizes[1].append(len(df["exv"]))
                if all(sizes[0][0] == sizes[0][i] for i in range(len(sizes[0]))):
                    print(sizes)
                    array_one = np.asarray(df["exv"])
                    array_two = np.asarray(df["numv"])
                    arrays[0].append(array_one)
                    arrays[1].append(array_two)
                    print("exact vector: ", array_one)
                    print("numeric vector: ", array_two)
            #break
        #l = [line for line in df]
        #pprint(l)
pprint(arrays)
print("array lenght" , len(arrays[0]))
print(len(arrays[1]))

cwd = os.getcwd()
print(cwd)
try:
    os.chdir(os.path.join(cwd , r"RUSTprojects\trburcor\Animations"))
    print("Directory changed")
except OSError:
    print("Can't change the Current Working Directory")   
route = "txt_to_parse"
for dirpath, dirnames, filenames in os.walk(route):
    print('Текущий путь:', dirpath)
    print('Папки:', dirnames)
    print('Файлы:', filenames)
    
#os.chdir(os.path.join(cwd, r"RUSTprojects\trburcor\Animations"))
x = np.linspace(dl, dr, sizes[0][0])
#print(x)
change_cor = True
dir_types = ['runge','triangle','gauss_wave','sinusoid','lines']
out_folder_path = dir_types[0]
crt = "_with correction"
if change_cor:
    png_path = out_folder_path+ r"_two" + crt
for i in range(len(arrays[0])):
    try:
        os.mkdir(out_folder_path)
    except FileExistsError as e:
        print('File already exists')
            #return False
    except OSError as e:
        print(f"An error has occurred. Continuing anyways: {e}")
    dst_param = os.path.join(cwd, out_folder_path, r'parameters_' + str(1) + '.txt')
    if not os.path.exists(dst_param):
        dst_file = open(dst_param, 'w')
    print(dst_param)
    src_param = r"C:\Users\2020\RUSTprojects\trburcor\src\treated_datas_0\parameters_nf0.txt"
    shutil.copyfile(src_param, dst_param)
    png_path_i = png_path + str(i)
    print(png_path_i)
    cur_image_path = os.path.join(out_folder_path, png_path_i)
    print(cur_image_path)
    plt.legend(["Exact solution","Numeric solution"],loc='upper left')
    plt.xlabel('Distance on x axis')
    plt.ylabel('height')           
    if change_cor:
        plt.title(out_folder_path + ' type'+ crt)
    else:
        plt.title(out_folder_path + ' type')
    plt.plot(x, arrays[0][i],'go--', linewidth=2, markersize=3, alpha = 0.6, animated ='false',
        markerfacecoloralt = 'b', fillstyle =  'left')
    plt.plot(x, arrays[1][i],'yo--', linewidth=3, markersize=3, alpha = 0.5, animated ='false',
        markerfacecoloralt = 'b', fillstyle =  'full')
    #plt.pause(0.1)
    try:
        plt.savefig(cur_image_path)
    except FileExistsError as e:
        print('File already exists')
    except OSError as e:
        print(f"An error has occurred. Continuing anyways: {e}")
    plt.show()
    
lines=[[]]
#colour map
cmap = ["green", "blue", "red", "orange"]
dst_file.close()
dif.close()
    
outer_sizes = [172, 33]
inner_sizes = [24,  24, 4,  116,    3,  0, 2,   4,  5,  20  ,1, 1]
outer_colors = ['#D4A6C8', '#A0CBE8']
inner_colors = ['PeachPuff','GhostWhite', 'Gold', 'Thistle', 'LightCyan', 'Wheat']
labels = ['Женщины', 'Мужчины', 'Гуманитарное',
'Естественнонаучное',
'Инженерно-техническое',
'Образование и педагогика',
'Социально-экономическое',
'Творческое']
plt.title('Направление образования (%)')
sex = ['Женщины', 'Мужчины']
plt.pie(outer_sizes,colors=outer_colors, startangle=90,frame=True, radius=4, labels= sex)

plt.pie(inner_sizes,colors=inner_colors,radius=3,startangle=90,autopct='%1.0f%%',
    pctdistance=0.8, textprops={'size':7})


plt.legend(labels, loc='upper left', bbox_to_anchor=(1.0, 1.0) ,facecolor = 'oldlace', edgecolor = 'r')
center_circle = plt.Circle((0,0), 1.5, color='black', fc='white', linewidth=0)
fig = plt.gcf()

fig.gca().add_artist(center_circle)
fig.set_size_inches(10,6)
plt.axis('scaled')
fig.tight_layout()