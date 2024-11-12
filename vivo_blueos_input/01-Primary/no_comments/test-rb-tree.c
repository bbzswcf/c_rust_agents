#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "rb-tree.h"
#include "compare-int.h"
#define NUM_TEST_VALUES 1000
int test_array[NUM_TEST_VALUES];
#if 0
static void print_tree(RBTreeNode *node, int depth)
{
	int *value;
	int i;
	if (node == NULL) {
		return;
	}
	print_tree(rb_tree_node_child(node, RB_TREE_NODE_LEFT), depth + 1);
	for (i=0; i<depth*6; ++i) {
		printf(" ");
	}
	value = rb_tree_node_key(node);
	printf("%i\n", *value);
	print_tree(rb_tree_node_child(node, RB_TREE_NODE_RIGHT), depth + 1);
}
#endif
int find_subtree_height(RBTreeNode *node)
{
	RBTreeNode *left_subtree;
	RBTreeNode *right_subtree;
	int left_height, right_height;
	if (node == NULL) {
		return 0;
	}
	left_subtree = rb_tree_node_child(node, RB_TREE_NODE_LEFT);
	right_subtree = rb_tree_node_child(node, RB_TREE_NODE_RIGHT);
	left_height = find_subtree_height(left_subtree);
	right_height = find_subtree_height(right_subtree);
	if (left_height > right_height) {
		return left_height + 1;
	} else {
		return right_height + 1;
	}
}
void validate_tree(RBTree *tree)
{
}
RBTree *create_tree(void)
{
	RBTree *tree;
	int i;
	tree = rb_tree_new((RBTreeCompareFunc) int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		rb_tree_insert(tree, &test_array[i], &test_array[i]);
	}
	return tree;
}
void test_rb_tree_new(void)
{
	RBTree *tree;
	tree = rb_tree_new((RBTreeCompareFunc) int_compare);
	assert(tree != NULL);
	assert(rb_tree_root_node(tree) == NULL);
	assert(rb_tree_num_entries(tree) == 0);
	rb_tree_free(tree);
	alloc_test_set_limit(0);
	tree = rb_tree_new((RBTreeCompareFunc) int_compare);
	assert(tree == NULL);
}
void test_rb_tree_insert_lookup(void)
{
	RBTree *tree;
	RBTreeNode *node;
	int i;
	int *value;
	tree = rb_tree_new((RBTreeCompareFunc) int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		rb_tree_insert(tree, &test_array[i], &test_array[i]);
		assert(rb_tree_num_entries(tree) == i + 1);
		validate_tree(tree);
	}
	assert(rb_tree_root_node(tree) != NULL);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		node = rb_tree_lookup_node(tree, &i);
		assert(node != NULL);
		value = rb_tree_node_key(node);
		assert(*value == i);
		value = rb_tree_node_value(node);
		assert(*value == i);
	}
	i = -1;
	assert(rb_tree_lookup_node(tree, &i) == NULL);
	i = NUM_TEST_VALUES + 100;
	assert(rb_tree_lookup_node(tree, &i) == NULL);
	rb_tree_free(tree);
}
void test_rb_tree_child(void)
{
	RBTree *tree;
	RBTreeNode *root;
	RBTreeNode *left;
	RBTreeNode *right;
	int values[] = { 1, 2, 3 };
	int *p;
	int i;
	tree = rb_tree_new((RBTreeCompareFunc) int_compare);
	for (i=0; i<3; ++i) {
		rb_tree_insert(tree, &values[i], &values[i]);
	}
	root = rb_tree_root_node(tree);
	p = rb_tree_node_value(root);
	assert(*p == 2);
	left = rb_tree_node_child(root, RB_TREE_NODE_LEFT);
	p = rb_tree_node_value(left);
	assert(*p == 1);
	right = rb_tree_node_child(root, RB_TREE_NODE_RIGHT);
	p = rb_tree_node_value(right);
	assert(*p == 3);
	assert(rb_tree_node_child(root, 10000) == NULL);
	assert(rb_tree_node_child(root, 2) == NULL);
	rb_tree_free(tree);
}
void test_out_of_memory(void)
{
	RBTree *tree;
	RBTreeNode *node;
	int i;
	tree = create_tree();
	alloc_test_set_limit(0);
	for (i=10000; i<20000; ++i) {
		node = rb_tree_insert(tree, &i, &i);
		assert(node == NULL);
		validate_tree(tree);
	}
	rb_tree_free(tree);
}
void test_rb_tree_free(void)
{
	RBTree *tree;
	tree = rb_tree_new((RBTreeCompareFunc) int_compare);
	rb_tree_free(tree);
	tree = create_tree();
	rb_tree_free(tree);
}
void test_rb_tree_lookup(void)
{
	RBTree *tree;
	int i;
	int *value;
	tree = create_tree();
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		value = rb_tree_lookup(tree, &i);
		assert(value != NULL);
		assert(*value == i);
	}
	i = -1;
	assert(rb_tree_lookup(tree, &i) == NULL);
	i = NUM_TEST_VALUES + 1;
	assert(rb_tree_lookup(tree, &i) == NULL);
	i = 8724897;
	assert(rb_tree_lookup(tree, &i) == NULL);
	rb_tree_free(tree);
}
void test_rb_tree_remove(void)
{
	RBTree *tree;
	int i;
	int x, y, z;
	int value;
	int expected_entries;
	tree = create_tree();
	i = NUM_TEST_VALUES + 100;
	assert(rb_tree_remove(tree, &i) == 0);
	i = -1;
	assert(rb_tree_remove(tree, &i) == 0);
	expected_entries = NUM_TEST_VALUES;
	for (x=0; x<10; ++x) {
		for (y=0; y<10; ++y) {
			for (z=0; z<10; ++z) {
				value = z * 100 + (9 - y) * 10 + x;
				assert(rb_tree_remove(tree, &value) != 0);
				validate_tree(tree);
				expected_entries -= 1;
				assert(rb_tree_num_entries(tree)
				       == expected_entries);
			}
		}
	}
	assert(rb_tree_root_node(tree) == NULL);
	rb_tree_free(tree);
}
void test_rb_tree_to_array(void)
{
	RBTree *tree;
	int entries[] = { 89, 23, 42, 4, 16, 15, 8, 99, 50, 30 };
	int sorted[]  = { 4, 8, 15, 16, 23, 30, 42, 50, 89, 99 };
	int num_entries = sizeof(entries) / sizeof(int);
	int i;
	int **array;
	tree = rb_tree_new((RBTreeCompareFunc) int_compare);
	for (i=0; i<num_entries; ++i) {
		rb_tree_insert(tree, &entries[i], NULL);
	}
	assert(rb_tree_num_entries(tree) == num_entries);
	array = (int **) rb_tree_to_array(tree);
	for (i=0; i<num_entries; ++i) {
		assert(*array[i] == sorted[i]);
	}
	free(array);
	alloc_test_set_limit(0);
	array = (int **) rb_tree_to_array(tree);
	assert(array == NULL);
	validate_tree(tree);
	rb_tree_free(tree);
}
static UnitTestFunction tests[] = {
	test_rb_tree_new,
	test_rb_tree_free,
	test_rb_tree_child,
	test_rb_tree_insert_lookup,
	test_rb_tree_lookup,
	test_out_of_memory,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
