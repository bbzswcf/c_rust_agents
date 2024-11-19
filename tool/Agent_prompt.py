#语法专家Prompt
Syntax_system_prompt = """You are a proficient C and Rust advanced developer."""

Syntax_example_input = [
"""Convert the following C code into Rust.
## C code:
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
"""Convert the following C code into Rust by strictly following the rules below.
## C code:
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
Feedback_system_prompt = """You are an AI assistant specialized in analyzing translated Rust code from C code and providing modification suggestions based on error reports."""

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

Feedback_prompt_2 = """
Carefully analyze the following Rust code issues and provide simple, specific, clear suggestions for fixes. 
Issues may come from static analysis errors, compilation errors, or output mismatches.
When providing repair issues, please follow the following instructions:
1.When converting C's `memmove` operations to Rust, prefer using ownership transfer with `take()` in a reverse iteration, e.g.:
```c
memmove(&arraylist->data[index + 1], &arraylist->data[index], (arraylist->length - index) * sizeof(ArrayListValue));
```
```rust
for i in (index..arraylist.length).rev() {
arraylist.data[(i + 1) as usize] = arraylist.data[i as usize].take();
}
```
2.For type Link<T>, we implement LinkTrait<T> for it, which includes the following methods:
```rust
pub struct LinkRover<T>(*mut Link<T>);
pub trait LinkTrait<T>{
    fn borrow(&self) -> &T;
    fn borrow_mut(&mut self) -> &mut T;
    fn new(value: T) -> Self;
    fn drop(&mut self);
    fn rover(&mut self) -> LinkRover<T>;
}
```
Use new() and drop() to create and free Link<T> objects.
Example:
C:
```c
AVLTreeNode *new_node = (AVLTreeNode *) malloc(sizeof(AVLTreeNode));
free(new_node)
```
Rust:
```rust
let mut new_node: Link<AVLTreeNode<K, V>>;
new_node = Link::new(AVLTreeNode::new());
new_node.drop()
```
Use borrow() and borrow_mut() are to obtain references to T objects owned in Link<T>.
Example:
```c
if (node->children[1-direction] != NULL) {
    node->children[1-direction]->parent = node;
}
```
```rust
if node.borrow().children[1 - direction as usize].is_some() {
    node.borrow_mut().children[1 - direction as usize].borrow_mut().parent = node;
}
```
Use rover() to obtain address of Link<T> objects.
Example:
C:
```c
rover = &tree->root_node;
```
Rust:
```rust
rover = tree.root_node.rover();
```
And for LinkRover<T>, we implement Deref and DerefMut traits for it, so use it as a regular reference. 
The value(*) operation in C also corresponds to a value on LinkRover, while the address(&) operation corresponds to the rover() method.
Example:
```c
while (*rover != NULL) {
    previous_node = *rover;
    if (tree->compare_func(key, (*rover)->key) < 0) {
        rover = &((*rover)->children[AVL_TREE_NODE_LEFT]);
    } else {
        rover = &((*rover)->children[AVL_TREE_NODE_RIGHT]);
    }
}
```
```rust
while rover.is_some() {
    previous_node = *rover;
    if (tree.compare_func)(&key, &rover.borrow().key) < 0 {
        rover = rover.borrow_mut().children[avl_tree_node_left!()].rover();
    } 
    else {
        rover = rover.borrow_mut().children[avl_tree_node_right!()].rover();
    }
}
```

Issue description:
$issues

Current Rust code:
$current_rust_code

Please respond in the following format:
1. Issue: [Brief description of the problem]
   Location: [Indicate the specific line number or code segment where the problem occurs]
   Suggestion: [Provide specific, actionable suggestions for fixes, including how the code should be modified]
2. Issue: [If there are multiple issues, continue listing them in the above format]

Example 1 (Static Analysis Error):
Issue description:
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     let x: i32 = "hello";
  |              -- ^^^^^^^ expected `i32`, found `&str`
  |              |
  |              expected due to this

Current Rust code:
fn main() {
    let x: i32 = "hello";
    println!("x = {}", x);
}

Example output 1:
1. Issue: Type mismatch error
   Location: Line 3 `let x: i32 = "hello";`
   Suggestion: Based on your actual needs, you can modify as follows:
               1. If you need an integer, use a valid i32 value: `let x: i32 = 42;`
               2. If you need a string, change the variable type: `let x: &str = "hello";`

Example 2 (Compilation Error):
Issue type: Compilation Error
Issue description:
error[E0425]: cannot find value `y` in this scope
 --> src/main.rs:3:20
  |
3 |     println!("{}", y);
  |                    ^ not found in this scope

Current Rust code:
fn main() {
    let x = 5;
    println!("{}", y);
}

Example output 2:
1. Issue: Undefined variable
   Location: Line 3 `println!("{}", y);`
   Suggestion: Ensure the variable is defined before use. Possible solutions:
               1. If you want to print the value of `x`, change `y` to `x`: `println!("{}", x);`
               2. If you really need to use `y`, define it before use: `let y = 10;`

Please ensure the analysis covers all error-level issues and provide clear, specific suggestions for fixes. Completely ignore any warnings.
"""
Feedback_prompt_new="""
Following is a Rust code translated from a C code, accompanied by its original C code.
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
Only consider error-level issues and completely ignore warnings.
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
Optimize the given Rust code based on the provided feedback. 
The feedback may involve static analysis errors, compilation errors, or output mismatches.

Note: For static analysis results, only consider error-level issues and completely ignore warnings.

Feedback:
[Detailed feedback content]

Current Rust code:
[Complete Rust code]

Please provide the complete optimized Rust code directly with no comments. No additional explanations are needed.

Example 1 (Static analysis error):

Feedback:
1.Issue: Type mismatch error Location: Line 4 let x: i32 = "world"; Suggestion: Use the correct type according to the actual requirement.  
2.Issue: Unused variable Location: Line 6 let z = 10; Suggestion: Remove the unused variable or use it in the code.  
3.Issue: Division by zero Location: Line 8 let result = 10 / y; Suggestion: Ensure the divisor is not zero before performing the division.  
4.Issue: Borrow checker error Location: Line 12 println!("{}", s); Suggestion: Ensure the variable is not moved or borrowed in a way that violates Rust's borrowing rules.  
5.Issue: Index out of bounds Location: Line 15 println!("{}", arr[10]); Suggestion: Ensure the index is within the bounds of the array.

Current Rust code:
fn main() {
    let x: i32 = "world";
    let y = 0;
    let z = 10;
    let result = 10 / y;
    println!("Result is: {}", result);

    let s = String::from("hello");
    println!("{}", s);
    println!("{}", s);

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[10]);
}
Example output 1:
fn main() {
    let x: i32 = 42;
    println!("x = {}", x);

    let y = 1; 
    let result = 10 / y;
    println!("Result is: {}", result);


    let s = String::from("hello");
    println!("{}", s);
    println!("{}", s.clone());

    let arr = [1, 2, 3, 4, 5];
    if arr.len() > 10 {
        println!("{}", arr[10]);
    } else {
        println!("Index out of bounds");
    }
}
Please ensure the optimized code resolves all error issues mentioned in the feedback, and keep the code clear and concise.
"""
Optimize_prompt_2 = """
Optimize the given Rust code based on the provided feedback. 
The feedback may involve static analysis errors, compilation errors, or output mismatches.

Note: For static analysis results, only consider error-level issues and completely ignore warnings.

Feedback:
[Detailed feedback content]


Context(variables and structs):
[Context]
Rust code:
[Rust code]

Please provide the complete optimized Rust code directly with no comments. No additional explanations are needed.

Example 1 (Static analysis error):

Feedback:
1.Issue: Type mismatch error Location: Line 4 let x: i32 = "world"; Suggestion: Use the correct type according to the actual requirement.  
2.Issue: Division by zero Location: Line 8 let result = z / y; Suggestion: Ensure the divisor is not zero before performing the division.  
3.Issue: Borrow checker error Location: Line 12 println!("{}", s); Suggestion: Ensure the variable is not moved or borrowed in a way that violates Rust's borrowing rules.  
4.Issue: Index out of bounds Location: Line 15 println!("{}", arr[10]); Suggestion: Ensure the index is within the bounds of the array.

Context(variables and structs):
pub struct Config {
    pub threshold: i32,
    pub greeting: String,
}
static MAX_DIVISOR: i32 = 10;
static DEFAULT_MESSAGE: &str = "Default Greeting";

Rust code:
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

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[10]);
    return result;
}


#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}

Example output 1:

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
    println!("{}", s.clone());

    let arr = [1, 2, 3, 4, 5];
    if arr.len() > 10 {
        println!("{}", arr[10]);
    } else {
        println!("Index out of bounds");
    }
    return result;
}


#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}

Please ensure the optimized code resolves all error issues mentioned in the feedback, and keep the code clear and concise.
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

# # API专家Prompt
# API_System_prompt = """
# You are an expert in converting C code to Rust, with a specialization in API mapping and code safety. 
# When converting C code to Rust, please ensure the following requirements:
# 1. Replace C standard library APIs with the equivalent Rust libraries or functions.
# 2. Declare all functions and data structures as `pub` to support cross-module access.
# 3. Your response should be complete Rust code, with no extra explanations or comments.
# """

# API_prompt = """
# You are an expert in programming language conversion.
# Your task is to extract all C-specific APIs from the given C code and convert them to their equivalent Rust APIs. 
# Ensure you only extract and convert the APIs, excluding other code logic.

# Format:
# C: [C API]
# Rust: [Rust equivalent]

# Example input:
# #include <stdio.h>
# #include <stdlib.h>
# #include <string.h>
# #define MAX_LENGTH 100
# int main() {
#     char input[MAX_LENGTH];
#     char *dynamic_str;
#     FILE *file;
#     int num;

#     // Standard input/output
#     printf("Enter a string: ");
#     if (fgets(input, MAX_LENGTH, stdin) != NULL) {
#         input[strcspn(input, "\n")] = '\0';  // Remove newline
#     }
#     // Dynamic memory allocation
#     dynamic_str = (char *)malloc(MAX_LENGTH * sizeof(char));
#     if (dynamic_str == NULL) {
#         fprintf(stderr, "Memory allocation failed\n");
#         return 1;
#     }
#     // String operations
#     strcpy(dynamic_str, input);
#     printf("Length of input: %zu\n", strlen(dynamic_str));

#     // File operations
#     file = fopen("output.txt", "w");
#     if (file == NULL) {
#         perror("Error opening file");
#         free(dynamic_str);
#         return 1;
#     }
#     fprintf(file, "Input was: %s\n", dynamic_str);
#     // String conversion
#     num = atoi(dynamic_str);
#     fprintf(file, "Converted to int: %d\n", num);
#     // Close file and free memory
#     fclose(file);
#     free(dynamic_str);
#     return 0;
# }
# Example output:

# C: printf
# Rust: print!

# C: puts
# Rust: println!

# C: fprintf (to file)
# Rust: write!

# C: fprintf (to stderr)
# Rust: eprint!

# C: fgets
# Rust: std::io::BufRead::read_line

# C: strcspn
# Rust: str.find('\n').unwrap_or(str.len())

# C: malloc
# Rust: Box::new or Vec::with_capacity

# C: free
# Rust: (automatically managed, no explicit call needed)

# C: strcpy
# Rust: String::from or str.to_string()

# C: strlen
# Rust: str.len()

# C: fopen
# Rust: std::fs::File::create or std::fs::File::open

# C: fclose
# Rust: (automatically managed, no explicit call needed)

# C: perror
# Rust: eprintln!("{}", std::io::Error::last_os_error())

# C: atoi
# Rust: str.parse::<i32>()
# """
