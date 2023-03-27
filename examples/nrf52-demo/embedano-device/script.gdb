set print asm-demangle on
set pagination off
target extended-remote 172.23.224.1:2331
# target extended-remote :2331 # use this instead of above for localhost connection
monitor semihosting enable
monitor semihosting IOClient 3
monitor reset
load
# b main
continue