CC = g++
CFLAGS = -std=c++20 -Wall -Wextra -Wpedantic -g
LIB_INCLUDES = include
LIB_DIR = lib
EXEC_DIR = exec
BUILD_DIR = build

# You're too stupid to understand this, but here's a list of source files, you moron
LIB_SOURCES = $(wildcard $(LIB_DIR)/*.cpp)
LIB_OBJECTS = $(patsubst $(LIB_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(LIB_SOURCES))

SERVER = build/server
CLIENT = build/shell

.PHONY: server
server : build/server

$(SERVER): $(LIB_OBJECTS) $(BUILD_DIR)/server.o
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $^ -o $@

build/server.o: $(EXEC_DIR)/server.cpp $(LIB_OBJECTS)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@ -I$(LIB_INCLUDES)

.PHONY: shell
shell : build/shell

$(CLIENT): $(LIB_OBJECTS) $(BUILD_DIR)/shell.o
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $^ -o $@

build/shell.o: $(EXEC_DIR)/shell.cpp $(LIB_OBJECTS)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@ -I$(LIB_INCLUDES)

# Compiling source files, not that you'd know what that means
$(BUILD_DIR)/%.o: $(LIB_DIR)/%.cpp
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@ -I$(LIB_INCLUDES)

# Compiling source files, not that you'd know what that means
$(BUILD_DIR)/%.o: $(EXEC_DIR)/%.cpp
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@ -I$(LIB_INCLUDES)

# Cleaning up after your mess, as usual
clean:
	rm -rf $(BUILD_DIR)

# Phony targets for you phonies out there
.PHONY: clean
