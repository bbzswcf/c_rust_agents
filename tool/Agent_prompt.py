#语法专家Prompt
Syntax_system_prompt = """You are a proficient C and Rust advanced developer."""

Syntax_example_input = [
"""Convert the following C code into Rust.
```c
int arraylist_enlarge(ArrayList *arraylist)
{
	ArrayListValue *data;
	unsigned int newsize;
	newsize = arraylist->_alloced * 2;

	data = realloc(arraylist->data, sizeof(ArrayListValue) * newsize);

	if (data == NULL) {
		return 0;
	} else {
		arraylist->data = data;
		arraylist->_alloced = newsize;
		return 1;
	}
}
```
""",
"""Convert the following C code into Rust.
```c
AVLTreeNode *avl_tree_insert(AVLTree *tree, AVLTreeKey key, AVLTreeValue value)
{
	AVLTreeNode **rover;
	AVLTreeNode *new_node;
	AVLTreeNode *previous_node;

	rover = &tree->root_node;
	previous_node = NULL;

	while (*rover != NULL) {
		previous_node = *rover;
		if (tree->compare_func(key, (*rover)->key) < 0) {
			rover = &((*rover)->children[AVL_TREE_NODE_LEFT]);
		} else {
			rover = &((*rover)->children[AVL_TREE_NODE_RIGHT]);
		}
	}
	new_node = (AVLTreeNode *) malloc(sizeof(AVLTreeNode));

	if (new_node == NULL) {
		return NULL;
	}

	new_node->children[AVL_TREE_NODE_LEFT] = NULL;
	new_node->children[AVL_TREE_NODE_RIGHT] = NULL;
	new_node->parent = previous_node;
	new_node->key = key;
	new_node->value = value;
	new_node->height = 1;

	*rover = new_node;

	avl_tree_balance_to_root(tree, previous_node);

	++tree->num_nodes;

	return new_node;
}
```
"""
]
Syntax_example_output = [
"""
```rust
pub fn arraylist_enlarge<T>(arraylist: &mut ArrayList<T>) -> i32 {
    let mut data: Vec<ArrayListValue<T>>;
    let mut newsize: u32;

    newsize = arraylist._alloced * 2;
	data = std::mem::take(&mut arraylist.data);
	data.resize_with(newsize as usize, || None);
	arraylist.data = data;
	arraylist._alloced = newsize;
	return 1;
}
```
""",
"""
```rust
pub fn avl_tree_insert<K, V>(tree: &mut AVLTree<K, V>, mut key: AVLTreeKey<K>, value: AVLTreeValue<V>) -> Link<AVLTreeNode<K, V>> {
    let mut rover: LinkRover<AVLTreeNode<K, V>>;
    let mut new_node: Link<AVLTreeNode<K, V>>;
    let mut previous_node: Link<AVLTreeNode<K, V>>;

    rover = tree.root_node.rover();
    previous_node = None;

    while rover.is_some() {
        previous_node = *rover;
        if (tree.compare_func)(&key, &rover.borrow().key) < 0 {
            rover = rover.borrow_mut().children[avl_tree_node_left!()].rover();
        } 
        else {
            rover = rover.borrow_mut().children[avl_tree_node_right!()].rover();
        }
    }

    let mut new_node = Link::new(AVLTreeNode::new());
    new_node.borrow_mut().parent = previous_node;
    new_node.borrow_mut().key = key;
    new_node.borrow_mut().value = value;
    new_node.borrow_mut().height = 1;
    *rover = new_node;
    avl_tree_balance_to_root(tree, previous_node);
    tree.num_nodes += 1;
    return new_node;
}
```
"""
]

Syntax_hash_compare_example_input = [
"""Convert the following C code into Rust.
```c
int float_compare(void *vnum1, void *vnum2)
{
    float *num1;
    float *num2;

    num1 = (float *) vnum1;
    num2 = (float *) vnum2;
    
    if (*num1 < *num2) return -1;
    else if (*num1 > *num2) return 1;
    else return 0;
}
```
""",
"""Convert the following C code into Rust.
```c
int str_length_compare(void *vstr1, void *vstr2) 
{
    char *str1;
    char *str2;
    
    str1 = (char *) vstr1;
    str2 = (char *) vstr2;
    
    size_t len1 = strlen(str1);
    size_t len2 = strlen(str2);
    
    if (len1 < len2) return -1;
    else if (len1 > len2) return 1;
    else return 0;
}
```
"""
]

Syntax_hash_compare_example_output = [
"""
```rust
pub fn float_compare(num1: &Option<f32>, num2: &Option<f32>) -> i32 {
    match (num1, num2) {
        (Some(v1), Some(v2)) => {
            if v1 < v2 { -1 }
            else if v1 > v2 { 1 }
            else { 0 }
        },
        _ => 0
    }
}
```
""",
"""
```rust
pub fn str_length_compare(str1: &Option<String>, str2: &Option<String>) -> i32 {
    match (str1, str2) {
        (Some(s1), Some(s2)) => {
            if s1.len() < s2.len() { -1 }
            else if s1.len() > s2.len() { 1 }
            else { 0 }
        },
        _ => 0
    }
}
```
"""
]

test_example_input = [
"""Convert the following C code into Rust.
```c
void test_arraylist_index_of(void)
{
	int entries[] = { 89, 4, 23, 42, 16, 15, 8, 99, 50, 30 };
	int num_entries;
	ArrayList *arraylist;
	int i;
	int index;
	int val;

	/* Generate an arraylist containing the entries in the array */

	num_entries = sizeof(entries) / sizeof(int);
	arraylist = arraylist_new(0);

	for (i=0; i<num_entries; ++i) {
		arraylist_append(arraylist, &entries[i]);
	}

	/* Check all values get found correctly */

	for (i=0; i<num_entries; ++i) {

		val = entries[i];

		index = arraylist_index_of(arraylist, int_equal, &val);

		assert(index == i);
	}

	/* Check invalid values */

	val = 0;
	assert(arraylist_index_of(arraylist, int_equal, &val) < 0);
	val = 57;
	assert(arraylist_index_of(arraylist, int_equal, &val) < 0);

	arraylist_free(arraylist);
}
```
""",
"""Convert the following C code into Rust.
```c
void test_avl_tree_child(void)
{
	AVLTree *tree;
	AVLTreeNode *root;
	AVLTreeNode *left;
	AVLTreeNode *right;
	int values[] = { 1, 2, 3 };
	int *p;
	int i;

	/* Create a tree containing some values. Validate the
	 * tree is consistent at all stages. */

	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);

	for (i=0; i<3; ++i) {
		avl_tree_insert(tree, &values[i], &values[i]);
	}

	/* Check the tree */

	root = avl_tree_root_node(tree);
	p = avl_tree_node_value(root);
	assert(*p == 2);

	left = avl_tree_node_child(root, AVL_TREE_NODE_LEFT);
	p = avl_tree_node_value(left);
	assert(*p == 1);

	right = avl_tree_node_child(root, AVL_TREE_NODE_RIGHT);
	p = avl_tree_node_value(right);
	assert(*p == 3);

	/* Check invalid values */

	assert(avl_tree_node_child(root, 10000) == NULL);
	assert(avl_tree_node_child(root, 2) == NULL);

	avl_tree_free(tree);
}
```
"""
]

test_example_output = [
"""
```rust
fn test_arraylist_index_of() {
    let entries = vec![89, 4, 23, 42, 16, 15, 8, 99, 50, 30];
    let num_entries = entries.len();
    let mut arraylist = arraylist_new(0).unwrap();

    for i in 0..num_entries {
        arraylist_append(&mut arraylist, Some(entries[i as usize]));
    }

    for i in 0..num_entries {
        let val = entries[i as usize];
        let index = arraylist_index_of(&mut arraylist, int_equal, &mut Some(val));

        assert_eq!(index, i.try_into().unwrap());
    }

    let val = 0;
    assert!(arraylist_index_of(&mut arraylist, int_equal, &mut Some(val)) < 0);
    let val = 57;
    assert!(arraylist_index_of(&mut arraylist, int_equal, &mut Some(val)) < 0);

    arraylist_free(&mut arraylist);
}
```
""",
"""
```rust
fn test_avl_tree_child() {
    let mut tree: AVLTree<i32, i32>;
    let mut root: Link<AVLTreeNode<i32, i32>>;
    let mut left: Link<AVLTreeNode<i32, i32>>;
    let mut right: Link<AVLTreeNode<i32, i32>>;
    let values: [i32; 3] = [1, 2, 3];
    let mut p: Option<i32>;
    let mut i: i32;

    tree = avl_tree_new(int_compare).unwrap();

    for i in 0..3 {
        avl_tree_insert(&mut tree, Some(values[i]), Some(values[i]));
    }

    root = avl_tree_root_node(&mut tree);
    p = avl_tree_node_value(root);
    assert_eq!(p.unwrap(), 2);

    left = avl_tree_node_child(root, avl_tree_node_left!());
    p = avl_tree_node_value(left);
    assert_eq!(p.unwrap(), 1);

    right = avl_tree_node_child(root, avl_tree_node_right!());
    p = avl_tree_node_value(right);
    assert_eq!(p.unwrap(), 3);

    assert!(avl_tree_node_child(root, 10000).is_none());
    assert!(avl_tree_node_child(root, 2).is_none());
    validate_tree(&mut tree);
    avl_tree_free(&mut tree);

    unsafe { assert_eq!(*CREATE_COUNT.lock().unwrap(), 0) };
}
```
"""
]

#反馈专家Prompt
Feedback_prompt_new="""Following is a Rust code translated from a C code, accompanied by its original C code.
Original C Code:
<c_code>
Translated Rust Code:
<rust_code>
This Rust code encountered the following errors during runtime, which may be due to runtime errors or mismatches with the original C code output.
Error message:
<error_message>
Then analyze the above error and refer to the original C code to infer the cause of the error.
You need to locate the Rust code snippets that caused the error and provide the corresponding fixed Rust code snippets in a series of issues
Note that there may be multiple errors caused by the same error Rust code snippet.
Do not add comments or code that is not necessary to fix the error. 

Format Instruction:
Fix of each error Rust code snippet should be placed separately in an issue, noted that this error Rust code snippet may involve one or multiple errors.
Each issue should start with the summary of errors it is involved.
Then provide your speculated reasons for these errors, and give the Rust code snippet that caused the errors, followed by a fix code snippet.
---
#Issue 1
Errors: <summary>
Reason: <reason>
Error Code:
<error code snippet>
Fixed Code:
<fixed code snippet>s
#Issue 2
Errors: <summary>
Reason: <reason>
Error Code:
<error code snippet>
Fixed Code:
<fixed code snippet>
...
#Issue k (k is no more than 5)
Errors: <summary>
Reason: <reason>
Error Code:
<error code snippet>
Fixed Code:
<fixed code snippet>

---
Example 1:
Original C Code:
#include <stdio.h>
void reverse_array(int arr[], int size) {
    int start = 0;
    int end = size - 1;
    while (start < end) {
        int temp = arr[start];
        arr[start] = arr[end];
        arr[end] = temp;
        start++;
        end--;
    }
}
int main() {
    int arr[] = {1, 2, 3, 4, 5};
    int size = sizeof(arr) / sizeof(arr[0]);
    reverse_array(arr, size);
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    return 0;
}

Translated Rust Code:
fn reverse_array(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = arr[i];
        arr[i] = arr[len - i];
        arr[len - i] = temp;
    }
}
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    reverse_array(&mut arr);
    for &item in &arr {
        print!("{} ", item);
    }
    println!();
}

Error message:
thread 'main' panicked at test.rs:6:18:
index out of bounds: the len is 5 but the index is 5

Issues:
#Issue 1
Errors: Array out of bounds access.
Reason: When i is 0, arr [len-i] is actually arr [len], which exceeds the valid index range of the array (the valid index range of the array is 0 to len-1).
Error Code:
arr[i] = arr[len - i];
arr[len - i] = temp;
Fix Code:
arr[i] = arr[len - 1 - i];
arr[len - 1 - i] = temp;

---
Example2:
Original C code:
void main() {
    int numbers[] = {1, 2, 3, 4, 5};
    int sum = 0;
    for (int i = 0; i < sizeof(numbers) / sizeof(numbers[0]); i++) {
        if (numbers[i] % 2 == 0) {
            sum += numbers[i];
        }
    }
    printf("The sum of even numbers is: %d\n", sum);
}

Translated Rust code:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("The sum of even numbers is: {}", sum);
}

Error message:
Output mismatch:
C output:
The sum of even numbers is: 6
Rust output:
The sum of even numbers is: 15

Issues:
#Issue 1
Errors: Output mismatch
Reason: C code calculates the sum of all even numbers in an array, while Rust code calculates the sum of all elements in the array.
Error Code:
let sum: i32 = numbers.iter().sum();
Fix Code:
let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
"""

"""
3. For custom parameters structures in function signatures except Link<T>, follow the following rules:
If the function is to create a structure object, it should return a Box type, as () -> Box<Struct>.
If the function is to use a structure object, itshould use its mut borrow in parameter list, as (&mut Box<Struct>).
If the function is to free a structure object, it should acquire its ownership in parameter list, as (Box<Struct>).
"""

# 优化专家Prompt
Optimize_prompt = """
Given a flawed Rust function code and its fix issues.
Analyze each fix issue, and when you think an issue can be adopted, fix the current Rust code based on the error code and fix code it provides.
After completing the analysis of all issues, provide the repaired Rust code.
Only output the repaired Rust code, without analysis results.
---
Example 1:
Fix issues:
#Issue 1
Errors: Array out of bounds access.
Reason: When i is 0, arr [len-i] is actually arr [len], which exceeds the valid index range of the array (the valid index range of the array is 0 to len-1).
Error Code:
arr[i] = arr[len - i];
arr[len - i] = temp;
Fix Code:
arr[i] = arr[len - 1 - i];
arr[len - 1 - i] = temp;

Rust Code:
fn reverse_array(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = arr[i];
        arr[i] = arr[len - i];
        arr[len - i] = temp;
    }
}

Fixed Rust Code:
fn reverse_array(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = arr[i];
        arr[i] = arr[len - 1 - i];
        arr[len - 1 - i] = temp;
    }
}
---
Example 2:
Fix issues:
#Issue 1
Errors: Output mismatch
Reason: C code calculates the sum of all even numbers in an array, while Rust code calculates the sum of all elements in the array.
Error Code:
let sum: i32 = numbers.iter().sum();
Fix Code:
let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();

Current Rust Code:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("The sum of even numbers is: {}", sum);
}

Fixed Rust Code:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("The sum of even numbers is: {}", sum);
}
---
Example 3:
Fix issues:
#Issue 1
Errors: Type mismatch.
Reason: The type of x is i32, but it is assigned a string 'world'.
Error Code:
let x: i32 = "world";
Fix Code:
let x: i32 = 42;
#Issue 2
Errors: Division by zero.
Reason: Did not check if the dividend y could be 0.
Error Code:
let result = z / y;
Fix Code:
let result = if y != 0 { z / y } else { 0 };

Current Rust Code:
pub struct Config {
    pub threshold: i32,
    pub greeting: String,
}
static MAX_DIVISOR: i32 = 10;
static DEFAULT_MESSAGE: &str = "Default Greeting";
fn foo(z: i32){
    let config = Config {
        threshold: 5,
        greeting: String::from("Hello from Config"),
    };
    let divisor = if z < config.threshold { MAX_DIVISOR } else { z };
    let y = divisor;
    let x: i32 = "world";
    let result = z / y;
    println!("Result is: {}", result);

    let s = config.greeting.clone();
    println!("{}", s);
    println!("{}", s);

    return result;
}

#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}

Fixed Rust Code:
pub struct Config {
    pub threshold: i32,
    pub greeting: String,
}
static MAX_DIVISOR: i32 = 10;
static DEFAULT_MESSAGE: &str = "Default Greeting";
fn foo(z: i32){
    let config = Config {
        threshold: 5,
        greeting: String::from("Hello from Config"),
    };
    
    let divisor = if z < config.threshold { MAX_DIVISOR } else { z };
    let y = divisor;
    let x: i32 = 42;
    let result = if y != 0 { z / y } else { 0 };
    println!("Result is: {}", result);

    let s = config.greeting.clone();
    println!("{}", s);
    println!("{}", s);

    return result;
}

#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}
"""

Optimize_prompt_2 = """
Given a flawed Rust function code and its fix issues.
Analyze each fix issue, and when you think an issue can be adopted, fix the current Rust code based on the error code and fix code it provides.
After completing the analysis of all issues, provide the repaired Rust code.
Only output the repaired Rust code, without analysis results.
---
Example:
Feedback:
#Issue 1
Errors: Type mismatch.
Reason: The type of x is i32, but it is assigned a string 'world'.
Error Code:
let x: i32 = "world";
Fix Code:
let x: i32 = 42;
#Issue 2
Errors: Division by zero.
Reason: Did not check if the dividend y could be 0.
Error Code:
let result = z / y;
Fix Code:
let result = if y != 0 { z / y } else { 0 };

Context(variables and structs):
pub struct Config {
    pub threshold: i32,
    pub greeting: String,
}
static MAX_DIVISOR: i32 = 10;
static DEFAULT_MESSAGE: &str = "Default Greeting";

Rust Code:
fn foo(z: i32){
    let config = Config {
        threshold: 5,
        greeting: String::from("Hello from Config"),
    };
    let divisor = if z < config.threshold { MAX_DIVISOR } else { z };
    let y = divisor;
    let x: i32 = "world";
    let result = z / y;
    println!("Result is: {}", result);

    let s = config.greeting.clone();
    println!("{}", s);
    println!("{}", s);

    return result;
}

#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}

Fixed Rust Code:
fn foo(z: i32){
    let config = Config {
        threshold: 5,
        greeting: String::from("Hello from Config"),
    };
    
    let divisor = if z < config.threshold { MAX_DIVISOR } else { z };
    let y = divisor;
    let x: i32 = 42;
    let result = if y != 0 { z / y } else { 0 };
    println!("Result is: {}", result);

    let s = config.greeting.clone();
    println!("{}", s);
    println!("{}", s);

    return result;
}

#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}
"""

Optimize_prompt_2_new = """
Given a flawed Rust code and its fix issues.
Analyze each fix issue, and when you think an issue can be adopted, fix the current Rust code based on the error code and fix code it provides.
After completing the analysis of all issues, provide the repaired Rust code.
Only output the repaired Rust code, without analysis results.
---
Example 1:
Current Rust Code:
###function 1###
fn reverse_array(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = arr[i];
        arr[i] = arr[len - i];
        arr[len - i] = temp;
    }
}

###function 2###
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    reverse_array(&mut arr);
    for &item in &arr {
        print!("{} ", item);
    }
    println!();
}

Fix issues:
#Issue 1
Errors: Array out of bounds access.
Reason: When i is 0, arr [len-i] is actually arr [len], which exceeds the valid index range of the array (the valid index range of the array is 0 to len-1).
Error Code:
arr[i] = arr[len - i];
arr[len - i] = temp;
Fix Code:
arr[i] = arr[len - 1 - i];
arr[len - 1 - i] = temp;

Fixed Rust Code:
###function 1###
fn reverse_array(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = arr[i];
        arr[i] = arr[len - 1 - i];
        arr[len - 1 - i] = temp;
    }
}

###function 2###
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    reverse_array(&mut arr);
    for &item in &arr {
        print!("{} ", item);
    }
    println!();
}
---
Example 2:
Current Rust Code:
###function 1###
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("The sum of even numbers is: {}", sum);
}

Fix issues:
#Issue 1
Errors: Output mismatch
Reason: C code calculates the sum of all even numbers in an array, while Rust code calculates the sum of all elements in the array.
Error Code:
let sum: i32 = numbers.iter().sum();
Fix Code:
let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();

Fixed Rust Code:
###function 1###
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("The sum of even numbers is: {}", sum);
}
---
Example 3:
Current Rust Code:
pub struct Config {
    pub threshold: i32,
    pub greeting: String,
}
static MAX_DIVISOR: i32 = 10;
static DEFAULT_MESSAGE: &str = "Default Greeting";
###function 1###
fn foo(z: i32){
    let config = Config {
        threshold: 5,
        greeting: String::from("Hello from Config"),
    };
    let divisor = if z < config.threshold { MAX_DIVISOR } else { z };
    let y = divisor;
    let x: i32 = "world";
    let result = z / y;
    println!("Result is: {}", result);

    let s = config.greeting.clone();
    println!("{}", s);
    println!("{}", s);

    return result;
}

###function 2###
#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}

Fix issues:
#Issue 1
Errors: Type mismatch.
Reason: The type of x is i32, but it is assigned a string 'world'.
Error Code:
let x: i32 = "world";
Fix Code:
let x: i32 = 42;
#Issue 2
Errors: Division by zero.
Reason: Did not check if the dividend y could be 0.
Error Code:
let result = z / y;
Fix Code:
let result = if y != 0 { z / y } else { 0 };


Fixed Rust Code:
pub struct Config {
    pub threshold: i32,
    pub greeting: String,
}
static MAX_DIVISOR: i32 = 10;
static DEFAULT_MESSAGE: &str = "Default Greeting";
###function 1###
fn foo(z: i32){
    let config = Config {
        threshold: 5,
        greeting: String::from("Hello from Config"),
    };
    
    let divisor = if z < config.threshold { MAX_DIVISOR } else { z };
    let y = divisor;
    let x: i32 = 42;
    let result = if y != 0 { z / y } else { 0 };
    println!("Result is: {}", result);

    let s = config.greeting.clone();
    println!("{}", s);
    println!("{}", s);

    return result;
}

###function 2###
#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}
"""
