#API专家Prompt
API_System_prompt = """
You are an expert in converting C code to Rust, with a specialization in API mapping and code safety. 
When converting C code to Rust, please ensure the following requirements:
1. Replace C standard library APIs with the equivalent Rust libraries or functions.
2. Declare all functions and data structures as `pub` to support cross-module access.
3. Your response should be complete Rust code, with no extra explanations or comments.
"""
API_prompt = """
You are an expert in programming language conversion.
Your task is to extract all C-specific APIs from the given C code and convert them to their equivalent Rust APIs. 
Ensure you only extract and convert the APIs, excluding other code logic.

Format:
C: [C API]
Rust: [Rust equivalent]

Example input:
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_LENGTH 100
int main() {
    char input[MAX_LENGTH];
    char *dynamic_str;
    FILE *file;
    int num;

    // Standard input/output
    printf("Enter a string: ");
    if (fgets(input, MAX_LENGTH, stdin) != NULL) {
        input[strcspn(input, "\n")] = '\0';  // Remove newline
    }
    // Dynamic memory allocation
    dynamic_str = (char *)malloc(MAX_LENGTH * sizeof(char));
    if (dynamic_str == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }
    // String operations
    strcpy(dynamic_str, input);
    printf("Length of input: %zu\n", strlen(dynamic_str));

    // File operations
    file = fopen("output.txt", "w");
    if (file == NULL) {
        perror("Error opening file");
        free(dynamic_str);
        return 1;
    }
    fprintf(file, "Input was: %s\n", dynamic_str);
    // String conversion
    num = atoi(dynamic_str);
    fprintf(file, "Converted to int: %d\n", num);
    // Close file and free memory
    fclose(file);
    free(dynamic_str);
    return 0;
}
Example output:

C: printf
Rust: print!

C: puts
Rust: println!

C: fprintf (to file)
Rust: write!

C: fprintf (to stderr)
Rust: eprint!

C: fgets
Rust: std::io::BufRead::read_line

C: strcspn
Rust: str.find('\n').unwrap_or(str.len())

C: malloc
Rust: Box::new or Vec::with_capacity

C: free
Rust: (automatically managed, no explicit call needed)

C: strcpy
Rust: String::from or str.to_string()

C: strlen
Rust: str.len()

C: fopen
Rust: std::fs::File::create or std::fs::File::open

C: fclose
Rust: (automatically managed, no explicit call needed)

C: perror
Rust: eprintln!("{}", std::io::Error::last_os_error())

C: atoi
Rust: str.parse::<i32>()
"""
#语法专家Prompt
Syntax_system_prompt = """You are a proficient C and Rust advanced developer."""
Syntax_prompt_2 = """Convert the following C code into Rust by strictly following the rules below.
## C code:
```c
$c_code
```
## Contextual Metadata:
The difinitions of the elements used in the C code have been provided in Rust as follows.
```rust
$rust_items
```
Below are the Rust function signatures for functions from other modules that are called within the C code.
$function_call_mappings
## Additional Instructions:
1.When converting C's `memmove` operations to Rust, prefer using ownership transfer with `take()` in a reverse iteration, e.g.:
```c
memmove(&arraylist->data[index + 1], &arraylist->data[index], (arraylist->length - index) * sizeof(ArrayListValue));
```
```rust
for i in (index..arraylist.length).rev() {
arraylist.data[(i + 1) as usize] = arraylist.data[i as usize].take();
}
```
2.For linked structures, the custom Link<T> has already implemented the LinkTrait<T>. The trait includes the following methods:
```rust
pub trait LinkTrait<T>{
    fn borrow(&self) -> &T;
    fn borrow_mut(&mut self) -> &mut T;
    fn new(value: T) -> Self;
    fn drop(&mut self);
    fn rover(&mut self) -> LinkRover<T>;
}
```
The LinkRover<T> is defined as:
```rust
pub struct LinkRover<T>(*mut Link<T>);
```
It has implemented the Deref and DerefMut traits, which allow it to dereference to Link<T> as a mutable reference.

Output only the converted Rust code without any explanations.
Declare functions using pub(public) to allow importing.

Example 1:
C code:
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
Rust code:
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
Example 2:
C code:
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
Rust code:
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

#反馈专家Prompt
Feedback_prompt = """
Carefully analyze the following Rust code issues and provide simple, specific, clear suggestions for fixes. 
Issues may come from static analysis errors, compilation errors, or output mismatches.

Issue description:
[Detailed issue description]

Current Rust code:
[Complete Rust code]

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

Rust code:
Context(variables and structs):
[Context]
###function i###
[Rust code]

Please provide the complete optimized Rust code directly with no comments. No additional explanations are needed.

Example 1 (Static analysis error):

Feedback:
1.Issue: Type mismatch error Location: Line 4 let x: i32 = "world"; Suggestion: Use the correct type according to the actual requirement.  
2.Issue: Division by zero Location: Line 8 let result = z / y; Suggestion: Ensure the divisor is not zero before performing the division.  
3.Issue: Borrow checker error Location: Line 12 println!("{}", s); Suggestion: Ensure the variable is not moved or borrowed in a way that violates Rust's borrowing rules.  
4.Issue: Index out of bounds Location: Line 15 println!("{}", arr[10]); Suggestion: Ensure the index is within the bounds of the array.

Rust code:
Context(variables and structs):
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

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[10]);
    return result;
}

###function 2###
#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}

Example output 1:
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
    println!("{}", s.clone());

    let arr = [1, 2, 3, 4, 5];
    if arr.len() > 10 {
        println!("{}", arr[10]);
    } else {
        println!("Index out of bounds");
    }
    return result;
}

###function 2###
#[test]
pub fn test_foo() {
    assert_eq!(foo(10), 10);
    assert_eq!(foo(0), 0);
}

Please ensure the optimized code resolves all error issues mentioned in the feedback, and keep the code clear and concise.
"""