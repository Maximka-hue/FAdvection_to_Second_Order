import os
import numpy as np
import math
from array import array
#import smooth

m = os.path.getsize('two.txt')
#f = open('time.txt','w')
o = open('output.txt','w')

def maxi(u,u1):
    if abs(u)>=abs(u1):
        return abs(u)
    else:
        return abs(u1)
def norm1(u,w,h,z,a,bq):
    for k in range(0,math.floor(N/2)+1):
        l = k - math.floor(z / h)
        if a<=0:
            if l >= N:
                bq[k] = abs(u[k] - w[l%N])
            else:
                bq[k] = abs(u[k] - w[l])
        else:
            if l <= 0:
                bq[k] = abs(u[k] - w[abs(l % N)])
            else:
                bq[k] = abs(u[k] - w[l])
    return max(bq)
def norm2(u,w,h,z,a,bq):
    for k in range(0,math.floor(N/2)+1):
        l = k - math.floor(z / h)
        if a<=0:
            if l >= N:
                bq[k] = (u[k] - w[l%N])**2
            else:
                bq[k] = (u[k] - w[l])**2
        else:
            if l <= 0:
                bq[k] = (u[k] - w[abs(l % N)])**2
            else:
                bq[k] = (u[k] - w[l])**2
    Sum = sum(bq)
    return math.sqrt(Sum)





with open("two.txt") as d:
   # for line in d:
    a = d.read()
    b = a.replace("\n", " ")
    c = b.split()
      #  a = data.append( d.split(' '))
       # print(line,'\n')
print(a)
print(c)
eqtype = int(c[0])
x_min = float(c[2])
x_max = float(c[3])
N = int(c[11])

T = float(c[4])
t_exp = float(c[5])
btype = int(c[6])
u0type = int(c[7])
u0_p1 = float(c[8])
u0_p2 = float(c[9])
u0_p3 = float(c[10])
C = float(c[12])
a = float(c[1])
i = t_exp
h = (x_max - x_min)/N
tau = C*h/abs(a)
q = a*tau/h
t = 0
n = 1
r = u0_p3/2
print('btype = ',btype)
print('x_min = ',x_min,'x_max = ', x_max, 'N = ',N,'\n', 'h = ',h)
print('T = ',T,'C_0 = ',C,'a = ',a,'\n','tau = ',tau,'\n')
u = [0.5]*(N+1)
w = [0.5]*(N+1)
data = [0]*(N+1)
data_max = [0]*(N+1)
tm = [0]*(N+1)
bq=[0]*(N+1)
x_left = u0_p1 - u0_p3/2
x = x_min
sigma = u0_p2**2
qrt=0
p = 1/(math.sqrt(2*math.pi))
tmp = open('tmp_'+ str(0) +'.txt','w')
print('x',',','u',',','w',file = tmp)
out = open('output.txt','w')
print('t',',','n',',','tau',file=out)
u_max = a
for k in range(0,N+1):
    x = x_min + k*h
    if u0type == 2:
        u[k] = math.exp(-((x - u0_p1) ** 2) / (2 * sigma)) * p / u0_p2
        w[k] = u[k]
        tm[k] = -math.exp(-((x - u0_p1) ** 2) / (2 * sigma)) *p/(u0_p2**3)
    elif u0type == 3:
        u[k] = math.sin(x*(2*math.pi)/(x_max-x_min))
        w[k] = u[k]
    elif u0type ==4:
        u[k]=u0_p2*x+u0_p3
        w[k]=u[k]
    else:
        if x >= x_left:
            if x <= u0_p1:
                if u0type == 0:
                    u[k] = u0_p3
                    w[k] = u[k]
                if u0type == 1:
                    u[k] = u0_p2*(x - x_left)/r + 0.5
                    w[k] = u[k]
                    tm[k] = u0_p2/r
            if x > u0_p1 and x <= u0_p1 + r:
                if u0type == 0:
                    u[k] = u0_p3
                    w[k] = u[k]
                if u0type == 1:
                    u[k] = u0_p2 * (u0_p1 + r - x)/r + 0.5
                    w[k] = u[k]
                    tm[k] = -u0_p2/r
            #else:
               # u[k] = 0.5
               # w[k] = u[k]


    print(x,',',u[k],',',w[k],file = tmp)
    print('k = ',k,'u[k] = ',u[k],'\n')
tmp.close()
diff = open('differential.txt','w')
print('t',',','norm1',',','norm2',file = diff)
print(t,',',0,',',0,file=diff)
print('min tm = ',min(tm))
#t_max = -1/min(tm)
#print('t_max',t_max)
u_max=max(u)
print('u_max_int = ',u_max)
if eqtype ==0:
    if a<=0:
        while t<=T:
            for k in range(0, N+1):
                x = x_min + k * h
                if x < x_max:
                    data[k] = u[k] - q*(u[k+1] - u[k])
            if btype == 1:
                data[0] = data[N-1]
                data[N] = data[1]
            z = a*t
            print(t,',',norm1(u,w,h,z,a,bq),',',norm2(u,w,h,z,a,bq), file=diff)
            if t>=t_exp:
                print(t,' ',u,'\n', file = o)
                tmp = open('tmp_'+ str(n) +'.txt','w')
                n+=1
                print('x',',','u',',','w',file = tmp)
                for k in range(0, N+1):
                    x = x_min + k * h
                    l = k - math.floor(z/h)
                    print('x= , l = ',x,l)
                #w[0] = w[N-1]
                #w[N] = w[1]
                    if l>=N:
                        print(x,',',u[k],',',w[l%N],file = tmp)
                    else:
                        print(x,',',u[k],',',w[l],file = tmp)
                tmp.close()
                t_exp +=i


            u = data.copy()
            t += tau
    if a>0:
        while t<=T:
            for k in range(1, N+1):
                x = x_min + k * h
                if x < x_max:
                    data[k] = u[k] - q*(u[k] - u[k-1])
            if btype == 1:
                data[0] = data[N-1]
                data[N] = data[1]
            z = a * t
            print(t,',',norm1(u,w,h,z,a,bq),',',norm2(u,w,h,z,a,bq),file=diff)
            if t>=t_exp:
                print(t,' ',u,'\n', file = o)
                tmp = open('tmp_'+ str(n) +'.txt','w')
                n+=1
                print('x',',','u',',','w',file = tmp)
                for k in range(0, N+1):
                    x = x_min + k * h
                    l = k - math.floor(z/h)
                    print('x= , l = ', x, l)
               # w[0] = w[N - 1]
               # w[N] = w[1]
                    if l<=0:
                        print(x,',',u[k],',',w[abs(l%N)],file = tmp)
                    else:
                        print(x,',',u[k],',',w[l],file = tmp)
                #if l<1:
                 #   print(x,',',u[k],',',w[N+l-1],file = tmp)
               # elif l>=N:
               #     print(x,',',u[k],',',w[l-N+1],file = tmp)
               # else:
               #     print(x,',',u[k],',',w[l],file = tmp)
                tmp.close()
                t_exp +=i

            u = data.copy()
            t += tau

if eqtype == 1:
    tau = C*h/abs(u_max)
    print('tau0 = ',tau)
    print(0,',',0,',',tau,file=out)
    while t<=T:
        for k in range(1, N):
            x = x_min + k * h
            if x <= x_max:
                if u[k]<=0:
                    data[k] = u[k] - 0.5*(u[k+1]*u[k+1]-u[k]*u[k])*tau/h
                else:
                    data[k] = u[k] - 0.5*(u[k]*u[k] - u[k-1]*u[k-1])*tau/h
            data_max[k] = maxi(data[k],data[1])
            #u_max = maxi(data_max,u_max)
            #print('x = ',x,'data_max= ',data_max)
        if btype == 1:
            data[0] = data[N-1]
            data[N] = data[1]
        else:
            data[0] = data[1]
            data[N] = data[N-1]
        data_max[0] = maxi(data[0],data[1])
        data_max[N] = maxi(data[N],data[1])
        #print('data_max: ',data_max)
        u_max = max(data_max)
        #print('u_max = ',u_max)
        tau = C*h/abs(u_max)
        #print('tau_now = ',tau)
        print(t,',',n,',',tau,file=out)
        if t>=t_exp:

           # z = a*t
           # print(t,' ',u,'\n', file = o)
            tmp = open('tmp_'+ str(n) +'.txt','w')
            print('x',',','u',',','w',file = tmp)
            n+=1
            for k in range(0, N+1):
                x = x_min + k * h
                w[k] = (u0_p2*x + u0_p3)/(u0_p2*t+1)
               # l = k - math.floor(z/h)
                #print('x= , l = ',x,l)
                #w[0] = w[N-1]
                #w[N] = w[1]
               # if l>=N:
                  #  print(x,',',u[k],file = tmp)
               # else:
                  #  print(x,',',u[k],file = tmp)
                print(x, ',', u[k],',',w[k], file=tmp)
            tmp.close()
            t_exp +=i
        u = data.copy()
        t += tau
out.close()
diff.close()
#while  t <= T:
   # while k <= x_max:
 #       t+=tau
 #       if i%t_exp==0:
 #           print(t,file = f)
 #       i+=1

s = open("one.txt","w")
for line in c:
    print(line,file = s)
#print('data: ',data, '\n')
#print(data[0])
#for i in range(len(data)):
  #  for j in range(len(data[i])):
  #      print(data[i][j], end = ' ')
   #     b.append(data[i][j])
s.close()
#f.close()
d.close()
o.close()
