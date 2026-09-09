# AEA-STATE/1 — Example C Caller Makefile
#
# IMPORTANT ABI NOTE:
# A static archive (.a / .lib) must match the architecture of the C compiler linking it.
# - Unix host -> libaea.a (gcc/clang main.c -L../../target/release -laea -o aea_caller)
# - MSVC host -> aea.lib (cl main.c /I../../include /link /LIBPATH:../../target/release aea.lib advapi32.lib ws2_32.lib userenv.lib ntdll.lib /OUT:aea_caller.exe)
# - ARM Cortex-M flight silicon -> libaea.a in lib/thumbv7em-none-eabihf/ (arm-none-eabi-gcc)

CC ?= gcc
CFLAGS ?= -std=c11 -Wall -Wextra -O2 -I../../include

# Host paths
HOST_LIB_DIR ?= ../../target/release
HOST_BIN ?= aea_caller

# Flight cross-compiler paths (e.g. arm-none-eabi-gcc)
CROSS_CC ?= arm-none-eabi-gcc
FLIGHT_LIB ?= ../../lib/thumbv7em-none-eabihf/libaea.a
FLIGHT_BIN ?= aea_caller_thumb.elf
FLIGHT_CFLAGS ?= -mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=hard --specs=nosys.specs

.PHONY: all host flight clean run

all: host

# Build and link against the host-compiled library
host:
	@echo "Linking main.c against host static library..."
	$(CC) $(CFLAGS) main.c -L$(HOST_LIB_DIR) -laea -o $(HOST_BIN)

run: host
	./$(HOST_BIN)

# Cross-compile for ARM Cortex-M4F target silicon
flight:
	@echo "Cross-compiling main.c for thumbv7em-none-eabihf..."
	$(CROSS_CC) $(CFLAGS) $(FLIGHT_CFLAGS) main.c $(FLIGHT_LIB) -o $(FLIGHT_BIN)

clean:
	rm -f $(HOST_BIN) $(FLIGHT_BIN) *.o
