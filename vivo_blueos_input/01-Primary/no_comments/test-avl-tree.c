#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "avl-tree.h"
#include "compare-int.h"
#define NUM_TEST_VALUES 1000
int test_array[NUM_TEST_VALUES];
#if 0
static void print_tree(AVLTreeNode *node, int depth)
{
	int *value;
	int i;
	if (node == NULL) {
		return;
	}
	print_tree(avl_tree_node_child(node, AVL_TREE_NODE_LEFT), depth + 1);
	for (i=0; i<depth*6; ++i) {
		printf(" ");
	}
	value = avl_tree_node_key(node);
	printf("%i\n", *value);
	print_tree(avl_tree_node_child(node, AVL_TREE_NODE_RIGHT), depth + 1);
}
#endif
int find_subtree_height(AVLTreeNode *node)
{
	AVLTreeNode *left_subtree;
	AVLTreeNode *right_subtree;
	int left_height, right_height;
	if (node == NULL) {
		return 0;
	}
	left_subtree = avl_tree_node_child(node, AVL_TREE_NODE_LEFT);
	right_subtree = avl_tree_node_child(node, AVL_TREE_NODE_RIGHT);
	left_height = find_subtree_height(left_subtree);
	right_height = find_subtree_height(right_subtree);
	if (left_height > right_height) {
		return left_height + 1;
	} else {
		return right_height + 1;
	}
}
int counter;
int validate_subtree(AVLTreeNode *node)
{
	AVLTreeNode *left_node, *right_node;
	int left_height, right_height;
	int *key;
	if (node == NULL) {
		return 0;
	}
	left_node = avl_tree_node_child(node, AVL_TREE_NODE_LEFT);
	right_node = avl_tree_node_child(node, AVL_TREE_NODE_RIGHT);
	if (left_node != NULL) {
		assert(avl_tree_node_parent(left_node) == node);
	}
	if (right_node != NULL) {
		assert(avl_tree_node_parent(right_node) == node);
	}
	left_height = validate_subtree(left_node);
	key = (int *) avl_tree_node_key(node);
	assert(*key > counter);
	counter = *key;
	right_height = validate_subtree(right_node);
	assert(avl_tree_subtree_height(left_node) == left_height);
	assert(avl_tree_subtree_height(right_node) == right_height);
	assert(left_height - right_height < 2 &&
	       right_height - left_height < 2);
	if (left_height > right_height) {
		return left_height + 1;
	} else {
		return right_height + 1;
	}
}
void validate_tree(AVLTree *tree)
{
	AVLTreeNode *root_node;
	int height;
	root_node = avl_tree_root_node(tree);
	if (root_node != NULL) {
		height = find_subtree_height(root_node);
		assert(avl_tree_subtree_height(root_node) == height);
	}
	counter = -1;
	validate_subtree(root_node);
}
AVLTree *create_tree(void)
{
	AVLTree *tree;
	int i;
	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		avl_tree_insert(tree, &test_array[i], &test_array[i]);
	}
	return tree;
}
void test_avl_tree_new(void)
{
	AVLTree *tree;
	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);
	assert(tree != NULL);
	assert(avl_tree_root_node(tree) == NULL);
	assert(avl_tree_num_entries(tree) == 0);
	avl_tree_free(tree);
	alloc_test_set_limit(0);
	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);
	assert(tree == NULL);
}
void test_avl_tree_insert_lookup(void)
{
	AVLTree *tree;
	AVLTreeNode *node;
	unsigned int i;
	int *value;
	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = (int) i;
		avl_tree_insert(tree, &test_array[i], &test_array[i]);
		assert(avl_tree_num_entries(tree) == i + 1);
		validate_tree(tree);
	}
	assert(avl_tree_root_node(tree) != NULL);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		node = avl_tree_lookup_node(tree, &i);
		assert(node != NULL);
		value = avl_tree_node_key(node);
		assert(*value == (int) i);
		value = avl_tree_node_value(node);
		assert(*value == (int) i);
	}
	i = NUM_TEST_VALUES + 100;
	assert(avl_tree_lookup_node(tree, &i) == NULL);
	avl_tree_free(tree);
}
void test_avl_tree_child(void)
{
	AVLTree *tree;
	AVLTreeNode *root;
	AVLTreeNode *left;
	AVLTreeNode *right;
	int values[] = { 1, 2, 3 };
	int *p;
	int i;
	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);
	for (i=0; i<3; ++i) {
		avl_tree_insert(tree, &values[i], &values[i]);
	}
	root = avl_tree_root_node(tree);
	p = avl_tree_node_value(root);
	assert(*p == 2);
	left = avl_tree_node_child(root, AVL_TREE_NODE_LEFT);
	p = avl_tree_node_value(left);
	assert(*p == 1);
	right = avl_tree_node_child(root, AVL_TREE_NODE_RIGHT);
	p = avl_tree_node_value(right);
	assert(*p == 3);
	assert(avl_tree_node_child(root, 10000) == NULL);
	assert(avl_tree_node_child(root, 2) == NULL);
	avl_tree_free(tree);
}
void test_out_of_memory(void)
{
	AVLTree *tree;
	AVLTreeNode *node;
	int i;
	tree = create_tree();
	alloc_test_set_limit(0);
	for (i=10000; i<20000; ++i) {
		node = avl_tree_insert(tree, &i, &i);
		assert(node == NULL);
		validate_tree(tree);
	}
	avl_tree_free(tree);
}
void test_avl_tree_free(void)
{
	AVLTree *tree;
	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);
	avl_tree_free(tree);
	tree = create_tree();
	avl_tree_free(tree);
}
void test_avl_tree_lookup(void)
{
	AVLTree *tree;
	int i;
	int *value;
	tree = create_tree();
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		value = avl_tree_lookup(tree, &i);
		assert(value != NULL);
		assert(*value == i);
	}
	i = -1;
	assert(avl_tree_lookup(tree, &i) == NULL);
	i = NUM_TEST_VALUES + 1;
	assert(avl_tree_lookup(tree, &i) == NULL);
	i = 8724897;
	assert(avl_tree_lookup(tree, &i) == NULL);
	avl_tree_free(tree);
}
void test_avl_tree_remove(void)
{
	AVLTree *tree;
	int i;
	int x, y, z;
	int value;
	unsigned int expected_entries;
	tree = create_tree();
	i = NUM_TEST_VALUES + 100;
	assert(avl_tree_remove(tree, &i) == 0);
	i = -1;
	assert(avl_tree_remove(tree, &i) == 0);
	expected_entries = NUM_TEST_VALUES;
	for (x=0; x<10; ++x) {
		for (y=0; y<10; ++y) {
			for (z=0; z<10; ++z) {
				value = z * 100 + (9 - y) * 10 + x;
				assert(avl_tree_remove(tree, &value) != 0);
				validate_tree(tree);
				expected_entries -= 1;
				assert(avl_tree_num_entries(tree)
				       == expected_entries);
			}
		}
	}
	assert(avl_tree_root_node(tree) == NULL);
	avl_tree_free(tree);
}
void test_avl_tree_to_array(void)
{
	AVLTree *tree;
	int entries[] = { 89, 23, 42, 4, 16, 15, 8, 99, 50, 30 };
	int sorted[]  = { 4, 8, 15, 16, 23, 30, 42, 50, 89, 99 };
	unsigned int num_entries = sizeof(entries) / sizeof(int);
	unsigned int i;
	int **array;
	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);
	for (i=0; i<num_entries; ++i) {
		avl_tree_insert(tree, &entries[i], NULL);
	}
	assert(avl_tree_num_entries(tree) == num_entries);
	array = (int **) avl_tree_to_array(tree);
	for (i=0; i<num_entries; ++i) {
		assert(*array[i] == sorted[i]);
	}
	free(array);
	alloc_test_set_limit(0);
	array = (int **) avl_tree_to_array(tree);
	assert(array == NULL);
	validate_tree(tree);
	avl_tree_free(tree);
}
static UnitTestFunction tests[] = {
	test_avl_tree_new,
	test_avl_tree_free,
	test_avl_tree_child,
	test_avl_tree_insert_lookup,
	test_avl_tree_lookup,
	test_avl_tree_remove,
	test_avl_tree_to_array,
	test_out_of_memory,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
