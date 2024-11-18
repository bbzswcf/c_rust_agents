type_convert_input_prompt="""
Translate the following C definitions of types, structs, variable and macros to Rust.
C definitions:
```c
{c_code}
```
Here are some translation experiences for your reference.
1. For void* in C, use Option<T> in Rust.
Example:
C: 
```c
typedef void *ArrayListValue;
```
Rust: 
```rust
pub type ArrayListValue<T> = Option<T>;
```
2. For array pointers in C, use Vec in Rust.
Example:
C: 
```c
typedef void *ArrayListValue;
typedef struct _ArrayList ArrayList;
struct _ArrayList {{
	ArrayListValue *data;
	unsigned int length;
	unsigned int _alloced;
}};
```
Rust: 
```rust
struct ArrayList<T> {{
    pub data: Vec<Option<T>>,
    pub length: u32,
    pub _alloced: u32,
}}
```
3. For recursive structures in C, that is, structures that contain pointers to the structure, use custom Link<T> in Rust.The definition of Link<T> has been provided in Rust as follows.
```rust
pub type Link<T> = Option<NonNull<T>>;
```
Example:
C: 
```c
typedef void *AVLTreeKey;

typedef void *AVLTreeValue;

typedef struct _AVLTreeNode AVLTreeNode;

struct _AVLTreeNode {{
	AVLTreeNode *children[2];
	AVLTreeNode *parent;
	AVLTreeKey key;
	AVLTreeValue value;
	int height;
}};
```
Rust:
```rust
pub type AVLTreeKey<T> = Option<T>;

pub type AVLTreeValue<T> = Option<T>;

pub struct _AVLTreeNode<K, V> {{
    pub children: [Link<AVLTreeNode<K, V>>; 2],
    pub parent: Link<AVLTreeNode<K, V>>,
    pub key: AVLTreeKey<K>,
    pub value: AVLTreeValue<V>,
    pub height: i32,
}}
```
Remember to output only the converted Rust code without any explanations.
Declare all items(strctures, enums, functions, constants, etc.) using pub(public) to allow importing.
"""

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
"""

feedback_input_prompt="""
Analyze the following error message based on the original C code and the translated Rust code:
Original C Code:
{c_code}

Translated Rust Code:
{rust_code}

Error message:
{error_msg}

Please provide specific fix suggestions on the Rust code, but do not generate improved code.
"""

optimize_input_prompt="""
Optimize the Rust code based on the following specific feedback:
Feedback:
{feedback}
Rust code:
Context(variables and structs):
{rust_items}

{functions}

Please strictly follow the steps mentioned in the prompt to optimize the code.
Ensure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.
Use the provided context (variables and structs) to ensure consistent usage of types and variables across functions.
Only return the complete optimized Rust code without additional explanations.
"""
