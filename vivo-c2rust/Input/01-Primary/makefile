CC = gcc
CFLAGS = -std=c99 -w -Wextra -g -I src -I test
# CFLAGS = -std=gnu99 -w -Wextra -g -I src -I test
SRC_DIR = src
TEST_DIR = test
# MACRO = -DALLOC_TESTING_C
MACRO = -DALLOC_TESTING

# 源文件和框架文件
SRC_FILES = \
	$(SRC_DIR)/arraylist.c \
	$(SRC_DIR)/avl-tree.c \
	$(SRC_DIR)/binary-heap.c \
	$(SRC_DIR)/binomial-heap.c \
	$(SRC_DIR)/bloom-filter.c \
	$(SRC_DIR)/hash-table.c \
	$(SRC_DIR)/list.c \
	$(SRC_DIR)/queue.c \
	$(SRC_DIR)/rb-tree.c \
	$(SRC_DIR)/set.c \
	$(SRC_DIR)/slist.c \
	$(SRC_DIR)/sortedarray.c \
	$(SRC_DIR)/trie.c

COMPARE_FILES = \
	$(SRC_DIR)/compare-int.c \
	$(SRC_DIR)/compare-pointer.c \
	$(SRC_DIR)/compare-string.c 

HASH_FILES = \
	$(SRC_DIR)/hash-int.c \
	$(SRC_DIR)/hash-pointer.c \
	$(SRC_DIR)/hash-string.c 

COMPARE_TEST_FILE = $(TEST_DIR)/test-compare-functions.c
HASH_TEST_FILE = $(TEST_DIR)/test-hash-functions.c

# FRAMEWORK = $(TEST_DIR)/alloc-testing.c $(TEST_DIR)/framework.c
FRAMEWORK = $(TEST_DIR)/alloc-testing.c $(TEST_DIR)/framework.c
# FRAMEWORK = 

# 测试文件列表
OTHER_TEST_FILES = \
	$(TEST_DIR)/test-arraylist.c \
	$(TEST_DIR)/test-avl-tree.c \
	$(TEST_DIR)/test-binary-heap.c \
	$(TEST_DIR)/test-binomial-heap.c \
	$(TEST_DIR)/test-bloom-filter.c \
	$(TEST_DIR)/test-hash-table.c \
	$(TEST_DIR)/test-list.c \
	$(TEST_DIR)/test-queue.c \
	$(TEST_DIR)/test-rb-tree.c \
	$(TEST_DIR)/test-set.c \
	$(TEST_DIR)/test-slist.c \
	$(TEST_DIR)/test-sortedarray.c \
	$(TEST_DIR)/test-trie.c


# 生成可执行文件列表
OTHER_EXEC_FILES = $(OTHER_TEST_FILES:$(TEST_DIR)/%.c=%)

EXEC_FILES = $(OTHER_EXEC_FILES) compare-functions hash-functions

# 默认目标：编译所有测试文件
all: $(EXEC_FILES)

# 通用规则：每个可执行文件依赖于源文件、框架文件和对应的测试文件
$(OTHER_EXEC_FILES): %:  $(FRAMEWORK) $(TEST_DIR)/%.c $(SRC_FILES) $(COMPARE_FILES) $(HASH_FILES)
	$(CC) $(CFLAGS) $(MACRO) -o $@ $(FRAMEWORK) $(TEST_DIR)/$@.c $(SRC_DIR)/$(@:test-%=%).c $(COMPARE_FILES) $(HASH_FILES)

compare-functions: $(COMPARE_FILES) $(FRAMEWORK) $(COMPARE_TEST_FILE)
	$(CC) $(CFLAGS) $(MACRO) -o compare-functions $^

hash-functions: $(HASH_FILES) $(FRAMEWORK) $(HASH_TEST_FILE)
	$(CC) $(CFLAGS) $(MACRO) -o hash-functions $^

 
# 运行所有测试文件
run: $(EXEC_FILES)
	@for exec in $^; do \
        echo "Running $$exec..."; \
        ./$$exec; \
    done

# 清理目标文件
clean:
	rm -f $(EXEC_FILES)
