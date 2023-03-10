# set print asm-demangle on
# target extended-remote 172.23.224.1:2331
# monitor semihosting enable
# monitor semihosting IOClient 3
# monitor reset
# load
# break main
# continue

set print asm-demangle on
set pagination off
target extended-remote 172.23.224.1:2331
# target extended-remote :2331
# monitor semihosting enable
# monitor semihosting IOClient 3
monitor reset
load
# b main