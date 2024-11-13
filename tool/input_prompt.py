type_convert_input_prompt="""
You are a proficient C and Rust advanced developer.
Here are some translation experiences for your reference.
1. For void* in C, use Option<T> in Rust.
Example:
C: 
typedef void *ArrayListValue;
Rust: 
pub type ArrayListValue<T> = Option<T>;
2. For array pointers in C, use Vec in Rust.
Example:
C: 
typedef void *ArrayListValue;
typedef struct _ArrayList ArrayList;
struct _ArrayList {{
	ArrayListValue *data;
	unsigned int length;
	unsigned int _alloced;
}};
Rust: 
struct ArrayList<T> {{
    pub data: Vec<Option<T>>,
    pub length: u32,
    pub _alloced: u32,
}}
3. For recursive structures in C, that is, structures that contain pointers to the structure, use Option<Rc<RefCell<T>>> in Rust/
Example:
C: 
typedef void *QueueValue;
typedef struct _QueueEntry QueueEntry;
struct _QueueEntry {{
	QueueValue data;
	QueueEntry *prev;
	QueueEntry *next;
}};
Rust:
pub sturct QueueEntry<T> {{
    data: Option<T>,
    prev: Option<Rc<RefCell<QueueEntry<T>>>>,
    next: Option<Rc<RefCell<QueueEntry<T>>>>,
}}
Translate the following C definitions of types or structs to Rust.
C definitions:
{c_code}
Remember to output only the converted Rust code without any explanations.
Declare all items(strctures, enums, functions, constants, etc.) using pub(public) to allow importing.
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



