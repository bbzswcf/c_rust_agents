type_convert_input_prompt="""
Convert the following C type definitions to Rust code.
Here are some relevant Rust code examples:

C code:
```c
{c_code}
```

Reference code:
```rust
{similar_rust_code}
```

Please follow these rules:
1. Use appropriate type conversion patterns
2. Maintain naming and visibility conventions
3. Use idiomatic Rust structures
4. Work with LinkRover<T> and Link<T> types
5. Handle memory safety and ownership

Output only the converted Rust code without explanation.
"""

type_convert_input_prompt_rag = """
Convert the following C code to Rust code.
Here are some relevant Rust code examples:
C code:
```c
{c_code}
```
Reference code:
```rust
{similar_rust_code}
```
Type Conversion Rules:
1. char* -> CStr
2. char array[] = "string" -> cstr!("string")
3. unsigned char* -> CStr
4. typedef void *a -> pub type a<T: GenericValue> = T //Always use the same format generic for void*.
5. Array literals -> Use arr! macro
   Example:
   C: int array[] = {{1, 2, 3}}
   Rust: arr![1, 2, 3]
6. Array/pointer fields -> Use Vector<T> instead of Vec<T>
   Example:
   C: ArrayListValue *data;
   Rust: pub data: Vector<ArrayListValue<T>>,
Available types and utilities in translation_utils:
1. Pointer Types:
   - Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>
2. Memory Operations:
   - c_malloc!, c_free!, c_calloc!, c_realloc!
   - c_memcpy!, c_memmove!, c_memset!
   - c_sizeof!, c_ref!
3. String Operations:
   - CStr type
   - c_strlen!, c_strcmp!, c_strdup!, c_sprintf!
   - c_tolower!, c_toupper!
4. Control Flow:
   - c_for!
Note: Check reference code for exact initialization patterns and type usage.
Output only the converted Rust code.
"""
type_convert_input_prompt_rag_1 = """
Convert the following C code to Rust code.
Here are some relevant Rust code examples:

C code:
```c
{c_code}
```

Reference code:
```rust
{similar_rust_code}
```

Study the reference implementation:
1. Variable and type patterns
2. Function implementations
3. Memory operations
4. Control structures

Available utilities in translation_utils:
- Memory: Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>
- Operations: c_malloc!, c_free!, c_memcpy!, c_ref!, etc.
- Strings: CStr, c_strlen!, c_strcmp!, etc.
- Control: c_for!

Convert the C code based on the reference implementation.
Output only the converted Rust code.
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
LinkRover<T> corresponds to the two-dimensional pointer in C, we implement Deref and DerefMut traits for it, so use it as a regular reference. 
The value(*) operation in C also corresponds to a value on LinkRover, while the address(&) operation corresponds to the rover() method.
Example:
```c
AVLTreeNode **rover;
rover = &tree->root_node;
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
let mut rover: LinkRover<AVLTreeNode<K, V>>;
rover = tree.root_node.rover();
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
Output only the converted Rust code without any explanations.
Declare functions using pub(public) to allow importing.
"""
Syntax_prompt_rag = """
Convert the following C code to Rust.
Below are some relevant Rust implementations.

C code:
```c
$c_code
```

Reference code:
```rust
$similar_rust_code
```
Type Conversion Rules:
1. char* -> CStr
2. char array[] = "string" -> cstr!("string")
3. unsigned char* -> CStr
Available types and utilities in translation_utils:
1. Pointer Types:
   - Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>
2. Memory Operations:
   - c_malloc!, c_free!, c_calloc!, c_realloc!
   - c_memcpy!, c_memmove!, c_memset!
   - c_sizeof!, c_ref!
3. String Operations:
   - CStr type
   - c_strlen!, c_strcmp!, c_strdup!, c_sprintf!
   - c_tolower!, c_toupper!
4. Control Flow:
   - c_for!
Note: Check reference code for exact initialization patterns and type usage.
Output only the converted Rust code.
"""
Syntax_prompt_rag_1 = """
Convert the following C code to Rust.
Below are some relevant Rust implementations.
C code:
```c
$c_code
```
Reference code:
```rust
$similar_rust_code
```
Study the reference implementation:
1. Variable and type patterns
2. Function implementations
3. Memory operations
4. Control structures

Available utilities in translation_utils:
- Memory: Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>
- Operations: c_malloc!, c_free!, c_memcpy!, c_ref!, etc.
- Strings: CStr, c_strlen!, c_strcmp!, etc.
- Control: c_for!

Convert the C code based on the reference implementation.
Output only the converted Rust code.
"""

test_prompt="""
Convert the following C code into Rust by strictly following the rules below.
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
"""

test_prompt_rag="""
Convert the following C code to Rust.
Below are some relevant Rust implementations.

C code:
```c
$c_code
```

Reference code:
```rust
$similar_rust_code
```
Type Conversion Rules:
1. char* -> CStr
2. unsigned char *p
3. char array[] = "string" -> cstr!("string")
4. Add 'mut' keyword for all mutable pointers in C
   Example:
   C: int a = 4;
   Rust: let mut a = 4;
   C: int array[5];
   Rust: let mut array: Array<i32, 5> = Default::default();
Available types and utilities in translation_utils:
1. Pointer Types:
   - Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>
2. Memory Operations:
   - c_malloc!, c_free!, c_calloc!, c_realloc!
   - c_memcpy!, c_memmove!, c_memset!
   - c_sizeof!, c_ref!
3. String Operations:
   - CStr type
   - c_strlen!, c_strcmp!, c_strdup!, c_sprintf!
   - c_tolower!, c_toupper!
4. Control Flow:
   - c_for!
Note: Check reference code for exact initialization patterns and type usage.
Output only the converted Rust code.
"""
test_prompt_rag_1="""
Convert the following C code to Rust code.
Here are some relevant Rust code examples:

C code:
```c
$c_code
```
Reference code:
```rust
$similar_rust_code
```

Study the reference implementation:
1. Variable and type patterns
2. Function implementations
3. Memory operations
4. Control structures

Available utilities in translation_utils:
- Memory: Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>
- Operations: c_malloc!, c_free!, c_memcpy!, c_ref!, etc.
- Strings: CStr, c_strlen!, c_strcmp!, etc.
- Control: c_for!

Convert the C code based on the reference implementation.
Output only the converted Rust code.
"""
feedback_input_prompt="""
Analyze the following error message based on the original C code and the translated Rust code:
Original C Code:
{c_code}

Translated Rust Code:
{rust_code}
Use the following rules to provide specific fix suggestions on the Rust code:
Type Conversion Rules:
1. char* -> CStr
2. unsigned char *p
3. char array[] = "string" -> cstr!("string")
4. typedef void *a -> pub type a<T: GenericValue> = T //Always use the same format generic for void*.
5. Array literals -> Use arr! macro
   Example:
   C: int array[] = {{1, 2, 3}}
   Rust: arr![1, 2, 3]
6. Add 'mut' keyword for all mutable pointers in C
   Example:
   C: int a = 4;
   Rust: let mut a = 4;
   C: int array[5];
   Rust: let mut array: Array<i32, 5> = Default::default();
Error message:
{error_msg}
Available types and utilities in translation_utils:
1. Pointer Types:
   - Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>
2. Memory Operations:
   - c_malloc!, c_free!, c_calloc!, c_realloc!
   - c_memcpy!, c_memmove!, c_memset!
   - c_sizeof!, c_ref!
3. String Operations:
   - CStr type
   - c_strlen!, c_strcmp!, c_strdup!, c_sprintf!
   - c_tolower!, c_toupper!
4. Control Flow:
   - c_for!
Please provide specific fix suggestions on the Rust code, but do not generate improved code.
"""

feedback_input_prompt_1="""
Analyze the following error message based on the original C code and the translated Rust code:
Original C Code:
{c_code}

Translated Rust Code:
{rust_code}

Error message:
{error_msg}

Please provide specific fix suggestions on the Rust code, but do not generate improved code.
When providing repair issues, please follow the following instructions:
1.When converting C's `memmove` operations to Rust, prefer using ownership transfer with `take()` in a reverse iteration, e.g.:
```c
    memmove(&arraylist->data[index + 1], &arraylist->data[index], (arraylist->length - index) * sizeof(ArrayListValue));
```
```rust
for i in (index..arraylist.length).rev() {{
    arraylist.data[(i + 1) as usize] = arraylist.data[i as usize].take();
}}
```
2.For type Link<T>, we implement LinkTrait<T> for it, which includes the following methods:
```rust
pub struct LinkRover<T>(*mut Link<T>);
pub trait LinkTrait<T>{{
    fn borrow(&self) -> &T;
    fn borrow_mut(&mut self) -> &mut T;
    fn new(value: T) -> Self;
    fn drop(&mut self);
    fn rover(&mut self) -> LinkRover<T>;
}}
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
if (node->children[1-direction] != NULL) {{
    node->children[1-direction]->parent = node;
}}
```
```rust
if node.borrow().children[1 - direction as usize].is_some() {{
    node.borrow_mut().children[1 - direction as usize].borrow_mut().parent = node;
}}
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
LinkRover<T> corresponds to the two-dimensional pointer in C, we implement Deref and DerefMut traits for it, so use it as a regular reference. 
The value(*) operation in C also corresponds to a value on LinkRover, while the address(&) operation corresponds to the rover() method.
Example:
```c
AVLTreeNode **rover;
rover = &tree->root_node;
while (*rover != NULL) {{
    previous_node = *rover;
    if (tree->compare_func(key, (*rover)->key) < 0) {{
        rover = &((*rover)->children[AVL_TREE_NODE_LEFT]);
    }} else {{
        rover = &((*rover)->children[AVL_TREE_NODE_RIGHT]);
    }}
}}
```
```rust
let mut rover: LinkRover<AVLTreeNode<K, V>>;
rover = tree.root_node.rover();
while rover.is_some() {{
    previous_node = *rover;
    if (tree.compare_func)(&key, &rover.borrow().key) < 0 {{
        rover = rover.borrow_mut().children[avl_tree_node_left!()].rover();
    }} 
    else {{
        rover = rover.borrow_mut().children[avl_tree_node_right!()].rover();
    }}
}}
```
"""

optimize_input_prompt="""
Optimize the Rust code based on the following specific feedback:
Feedback:
{feedback}

Context(variables and structs):
{rust_items}

Rust code:
{functions}

Please strictly follow the steps mentioned in the prompt to optimize the code.
Ensure all issues mentioned in the feedback are resolved, and do not add comments.
Use the provided context (variables and structs) to ensure consistent usage of types and variables across functions.
Only return the complete optimized Rust code without additional explanations.
"""

fix_prompt="""
You are given the below errors from clippy and Rust code snippets from one or more '. rs ' files related to the errors.
errors:
{error_block}
---
code snippets:
{code_snippets}
Instructions: 
Fix the error on the above code snippets. 
Not every snippet might require a fix or be relevant to the error, but take into account the code in all above snippets as it could help you derive the best possible fix.
Assume that the snippets might not be complete and could be missing lines above or below. 
Do not add comments or code that is not necessary to fix the error. 
Do not use unsafe or unstable features ( through '#![ feature (...) ]').
For your answer, return one or more ChangeLog groups, each containing one or more fixes to the above code snippets. 
Each group must be formatted with the below instructions.
Format instructions: 
Each ChangeLog group must start with a description of its included fixes. 
The group must then list one or more pairs of ( OriginalCode , FixedCode ) code snippets. 
Each OriginalCode snippet must list all consecutive original lines of code that must be replaced ( including a few lines before and after the fixes ), 
followed by the FixedCode snippet with all consecutive fixed lines of code that must replace the original lines of code ( including the same few lines before and after the changes ).
In each pair, the OriginalCode and FixedCode snippets must start at the same source code line number [N]. 
Use line number in code snippets instead of those in error message. 
Each listed code line, in both the OriginalCode and FixedCode snippets, must be prefixed with [ N ] that matches the line index N in the above snippets, and then be prefixed with exactly the same whitespace indentation as the original snippets above.
Make sure that OriginalCode snippet and FixedCode snippet correspond to the same range, that is, if this changelog is adopted, then simply replacing OriginalCode snippet with FixedCode snippet is sufficient.
If the OriginalCode should be deleted, then keep the corresponding FixedCode snippets empty.
Do not repeat the position of each OriginalCode snippet.
If there is help information in the error, you can directly apply it.

---
ChangeLog :1 @ <file>
FixDescription : <summary>.
OriginalCode@4 -6:
[4] <white space> <original code line>
[5] <white space> <original code line>
[6] <white space> <original code line>
FixedCode@4 -6:
[4] <white space> <fixed code line>
[5] <white space> <fixed code line>
[6] <white space> <fixed code line>
OriginalCode@9 -10:
[9] <white space> <original code line>
[10] <white space> <original code line>
FixedCode@9 -9:
[9] <white space> <fixed code line>
...
ChangeLog : K@ <file>
FixDescription : <summary>.
OriginalCode@15 -16:
[15] <white space> <original code line>
[16] <white space> <original code line>
FixedCode@15 -17:
[15] <white space> <fixed code line>
[16] <white space> <fixed code line>
[17] <white space> <fixed code line>
OriginalCode@23 -23:
[23] <white space> <original code line>
FixedCode@23 -23:
[23] <white space> <fixed code line>
---
Answer:
"""