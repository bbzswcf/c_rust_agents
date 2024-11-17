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
3. For recursive structures in C, that is, structures that contain pointers to the structure, use Option<Rc<RefCell<T>>> in Rust/
Example:
C: 
```c
typedef void *QueueValue;
typedef struct _QueueEntry QueueEntry;
struct _QueueEntry {{
	QueueValue data;
	QueueEntry *prev;
	QueueEntry *next;
}};
```
Rust:
```rust
pub sturct QueueEntry<T> {{
    data: Option<T>,
    prev: Option<Rc<RefCell<QueueEntry<T>>>>,
    next: Option<Rc<RefCell<QueueEntry<T>>>>,
}}
```c
Remember to output only the converted Rust code without any explanations.
Declare all items(strctures, enums, functions, constants, etc.) using pub(public) to allow importing.
Keep all variable names unchanged, and do not change the case of variable names.
"""

feedback_input_prompt="""
Original C Code:
{c_code}

Translated Rust Code:
{rust_code}

Error message:
{error_msg}

Issues:
"""

optimize_input_prompt="""
Current Rust Code:
{rust_code}

Fix issues:
{issues}

Fixed Rust Code:
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