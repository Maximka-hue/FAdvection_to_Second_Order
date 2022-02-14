import numpy as np
from matplotlib import pyplot as pl
from matplotlib import animation

# Helper functions for gaussian wave-packets

def gauss_x(x, a, x0, k0):
    """
    a gaussian wave packet of width a, centered at x0, with momentum k0
    """ 
    return ((a * np.sqrt(np.pi)) ** (-0.5)
            * np.exp(-0.5 * ((x - x0) * 1. / a) ** 2 + 1j * x * k0))

def gauss_k(k,a,x0,k0):
    """
    analytical of gauss_x(x), above
    """
    return ((a / np.sqrt(np.pi))**0.5
            * np.exp(-0.5 * (a * (k - k0)) ** 2 - 1j * (k - k0) * x0))
def norm1(u,w,h,z,a,bq): #u- numeric, w-exact vector, h- step, z- for tmp t*velocity, a- velocity, bq-dif(u,w) additional aray
    for k in range(0, math.floor(N/2)+1):
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
    for k in range(0, math.floor(N/2)+1):
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
