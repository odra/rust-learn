#!/usr/bin/env python
import ctypes

path = 'target/debug/liblib.so'
lib = ctypes.cdll.LoadLibrary(path) 
print(lib.rsum(1, 2))
