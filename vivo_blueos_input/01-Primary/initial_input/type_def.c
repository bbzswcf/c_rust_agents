type_prompt="""
You are a proficient C and Rust advanced developer.
Here are some translation experiences for your reference.
1. For void* in C, use Option<T> in Rust.
Example:
C: 
typedef void *ArrayListValue;
Rust: 
pub type ArrayListValue = Option<T>;
2. For array pointers in C, use Vec in Rust.
Example:
C: 
typedef void *ArrayListValue;
typedef struct _ArrayList ArrayList;
struct _ArrayList {
	ArrayListValue *data;
	unsigned int length;
	unsigned int _alloced;
};
Rust: 
struct ArrayList<T> {
    pub data: Vec<Option<T>>,
    pub length: u32,
    pub _alloced: u32,
}
3. For recursive structures in C, that is, structures that contain pointers to the structure, use Option<Rc<RefCell<T>>> in Rust/
Example:
C: 
typedef void *QueueValue;
typedef struct _QueueEntry QueueEntry;
struct _QueueEntry {
	QueueValue data;
	QueueEntry *prev;
	QueueEntry *next;
};
Rust:
pub sturct QueueEntry<T> {
    data: Option<T>,
    prev: Option<Rc<RefCell<QueueEntry<T>>>>,
    next: Option<Rc<RefCell<QueueEntry<T>>>>,
}
Translate the following C definitions of types or structs to Rust.
C definitions:
{c_code}
Remember to output only the converted Rust code without any explanations.
Declare all items(strctures, enums, functions, constants, etc.) using pub(public) to allow importing.
"""

// typedef void *ListValue;
// struct _ListEntry {
// 	ListValue data;
// 	ListEntry *prev;
// 	ListEntry *next;
// };
// typedef struct _ListEntry ListEntry;

pub type ListValue = Option<T>;
pub struct ListEntry<T> {
    pub data: ListValue<T>,
    pub prev: Option<Rc<RefCell<ListEntry<T>>>>,
    pub next: Option<Rc<RefCell<ListEntry<T>>>>,
}


// arraylist
typedef void *ArrayListValue;
typedef struct _ArrayList ArrayList;
struct _ArrayList {
	ArrayListValue *data;
	unsigned int length;
	unsigned int _alloced;
};
typedef int (*ArrayListEqualFunc)(ArrayListValue value1,
                                  ArrayListValue value2);
typedef int (*ArrayListCompareFunc)(ArrayListValue value1,
                                    ArrayListValue value2);



// avl-tree
struct _AVLTreeNode {
	AVLTreeNode *children[2];
	AVLTreeNode *parent;
	AVLTreeKey key;
	AVLTreeValue value;
	int height;
};

struct _AVLTree {
	AVLTreeNode *root_node;
	AVLTreeCompareFunc compare_func;
	unsigned int num_nodes;
};
typedef struct _AVLTree AVLTree;
typedef void *AVLTreeKey;
typedef void *AVLTreeValue;
typedef struct _AVLTreeNode AVLTreeNode;
typedef enum {
	AVL_TREE_NODE_LEFT = 0,
	AVL_TREE_NODE_RIGHT = 1
} AVLTreeNodeSide;
typedef int (*AVLTreeCompareFunc)(AVLTreeValue value1, AVLTreeValue value2);
#define AVL_TREE_NULL ((void *) 0)

//binary-heap
struct _BinaryHeap {
	BinaryHeapType heap_type;
	BinaryHeapValue *values;
	unsigned int num_values;
	unsigned int alloced_size;
	BinaryHeapCompareFunc compare_func;
};
typedef enum {
	BINARY_HEAP_TYPE_MIN,
	BINARY_HEAP_TYPE_MAX
} BinaryHeapType;
typedef void *BinaryHeapValue;
#define BINARY_HEAP_NULL ((void *) 0)
typedef int (*BinaryHeapCompareFunc)(BinaryHeapValue value1,
typedef struct _BinaryHeap BinaryHeap;

//binomial-heap
typedef struct _BinomialTree BinomialTree;
struct _BinomialTree
{
	BinomialHeapValue value;
	unsigned short order;
	unsigned short refcount;
	BinomialTree **subtrees;
};
struct _BinomialHeap
{
	BinomialHeapType heap_type;
	BinomialHeapCompareFunc compare_func;
	unsigned int num_values;
	BinomialTree **roots;
	unsigned int roots_length;
};
typedef enum {
	BINOMIAL_HEAP_TYPE_MIN,
	BINOMIAL_HEAP_TYPE_MAX
} BinomialHeapType;
typedef void *BinomialHeapValue;
#define BINOMIAL_HEAP_NULL ((void *) 0)
typedef int (*BinomialHeapCompareFunc)(BinomialHeapValue value1,
                                       BinomialHeapValue value2);
typedef struct _BinomialHeap BinomialHeap;



//bloom-filter
struct _BloomFilter {
	BloomFilterHashFunc hash_func;
	unsigned char *table;
	unsigned int table_size;
	unsigned int num_functions;
};
typedef struct _BloomFilter BloomFilter;
typedef void *BloomFilterValue;
typedef unsigned int (*BloomFilterHashFunc)(BloomFilterValue data);


// compare

// hash
struct _HashTableEntry {
	HashTablePair pair;
	HashTableEntry *next;
};
struct _HashTable {
	HashTableEntry **table;
	unsigned int table_size;
	HashTableHashFunc hash_func;
	HashTableEqualFunc equal_func;
	HashTableKeyFreeFunc key_free_func;
	HashTableValueFreeFunc value_free_func;
	unsigned int entries;
	unsigned int prime_index;
};
typedef struct _HashTable HashTable;
typedef struct _HashTableIterator HashTableIterator;
typedef struct _HashTableEntry HashTableEntry;
typedef void *HashTableKey;
typedef void *HashTableValue;
typedef struct _HashTablePair{
	HashTableKey key;
	HashTableValue value;
} HashTablePair;
struct _HashTableIterator {
	HashTable *hash_table;
	HashTableEntry *next_entry;
	unsigned int next_chain;
};
#define HASH_TABLE_NULL ((void *) 0)
typedef unsigned int (*HashTableHashFunc)(HashTableKey value);
typedef int (*HashTableEqualFunc)(HashTableKey value1, HashTableKey value2);
typedef void (*HashTableKeyFreeFunc)(HashTableKey value);
typedef void (*HashTableValueFreeFunc)(HashTableValue value);




//list
struct _ListEntry {
	ListValue data;
	ListEntry *prev;
	ListEntry *next;
};
typedef struct _ListEntry ListEntry;
typedef struct _ListIterator ListIterator;
typedef void *ListValue;
struct _ListIterator {
	ListEntry **prev_next;
	ListEntry *current;
};
#define LIST_NULL ((void *) 0)
typedef int (*ListCompareFunc)(ListValue value1, ListValue value2);
typedef int (*ListEqualFunc)(ListValue value1, ListValue value2);



//queue
typedef struct _QueueEntry QueueEntry;
struct _QueueEntry {
	QueueValue data;
	QueueEntry *prev;
	QueueEntry *next;
};
struct _Queue {
	QueueEntry *head;
	QueueEntry *tail;
};
typedef struct _Queue Queue;
typedef void *QueueValue;
#define QUEUE_NULL ((void *) 0)

//rb-tree
struct _RBTreeNode {
	RBTreeNodeColor color;
	RBTreeKey key;
	RBTreeValue value;
	RBTreeNode *parent;
	RBTreeNode *children[2];
};

struct _RBTree {
	RBTreeNode *root_node;
	RBTreeCompareFunc compare_func;
	int num_nodes;
};
typedef struct _RBTree RBTree;
typedef void *RBTreeKey;
typedef void *RBTreeValue;
#define RB_TREE_NULL ((void *) 0)
typedef struct _RBTreeNode RBTreeNode;
typedef int (*RBTreeCompareFunc)(RBTreeValue data1, RBTreeValue data2);
typedef enum {
	RB_TREE_NODE_RED,
	RB_TREE_NODE_BLACK,
} RBTreeNodeColor;
typedef enum {
	RB_TREE_NODE_LEFT = 0,
	RB_TREE_NODE_RIGHT = 1
} RBTreeNodeSide;


//set
struct _SetEntry {
	SetValue data;
	SetEntry *next;
};
struct _Set {
	SetEntry **table;
	unsigned int entries;
	unsigned int table_size;
	unsigned int prime_index;
	SetHashFunc hash_func;
	SetEqualFunc equal_func;
	SetFreeFunc free_func;
};
typedef struct _Set Set;
typedef struct _SetIterator SetIterator;
typedef struct _SetEntry SetEntry;
typedef void *SetValue;
struct _SetIterator {
	Set *set;
	SetEntry *next_entry;
	unsigned int next_chain;
};
#define SET_NULL ((void *) 0)
typedef unsigned int (*SetHashFunc)(SetValue value);
typedef int (*SetEqualFunc)(SetValue value1, SetValue value2);
typedef void (*SetFreeFunc)(SetValue value);




//slist
struct _SListEntry {
	SListValue data;
	SListEntry *next;
};
typedef struct _SListEntry SListEntry;
typedef struct _SListIterator SListIterator;
typedef void *SListValue;
struct _SListIterator {
	SListEntry **prev_next;
	SListEntry *current;
};
#define SLIST_NULL ((void *) 0)
typedef int (*SListCompareFunc)(SListValue value1, SListValue value2);
typedef int (*SListEqualFunc)(SListValue value1, SListValue value2);




//sortedarray
struct _SortedArray {
	SortedArrayValue *data;
	unsigned int length;
	unsigned int _alloced;
	SortedArrayEqualFunc equ_func;
	SortedArrayCompareFunc cmp_func;
};
typedef void *SortedArrayValue;
typedef struct _SortedArray SortedArray;
typedef int (*SortedArrayEqualFunc)(SortedArrayValue value1,
                                    SortedArrayValue value2);
typedef int (*SortedArrayCompareFunc)(SortedArrayValue value1,
                                      SortedArrayValue value2);





//trie
typedef struct _TrieNode TrieNode;
struct _TrieNode {
	TrieValue data;
	unsigned int use_count;
	TrieNode *next[256];
};
struct _Trie {
	TrieNode *root_node;
};
typedef struct _Trie Trie;
typedef void *TrieValue;
#define TRIE_NULL ((void *) 0)

