#API专家Prompt
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
Syntax_prompt = """
Convert C code to Rust using provided API mappings. 
Output only the converted Rust code without explanations.

Example input:
C code:
#include <stdio.h>
#include <stdlib.h>
typedef struct {
    int id;
    char name[50];
} Student;
void print_students(Student* students, int count) {
    for (int i = 0; i < count; i++) {
        printf("ID: %d, Name: %s\n", students[i].id, students[i].name);
    }
}
int main() {
    int n;
    printf("Enter number of students: ");
    scanf("%d", &n);
    Student* students = (Student*)malloc(n * sizeof(Student));
    if (students == NULL) {
        printf("Memory allocation failed!\n");
        return 1;
    }
    for (int i = 0; i < n; i++) {
        printf("Enter ID and Name for student %d: ", i + 1);
        scanf("%d %49s", &students[i].id, students[i].name);
    }
    print_students(students, n);
    free(students);
    return 0;
}
API mappings:
C: printf
Rust:print!

C: scanf
Rust: std::io::stdin().read_line(&mut String)

C: malloc
Rust: Vec::with_capacity(size)

C: free
Rust:(automatically managed, no explicit call needed)

C: typedef
Rust: struct

C: void function()
Rust: fn function()

Example output:
use std::io;
struct Student {
    id: i32,
    name: String,
}
fn print_students(students: &[Student]) {
    for student in students {
        println!("ID: {}, Name: {}", student.id, student.name);
    }
}
fn main() {
    let mut n = String::new();
    println!("Enter number of students: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a number");

    let mut students = Vec::with_capacity(n);
    
    for i in 0..n {
        let mut input = String::new();
        println!("Enter ID and Name for student {}: ", i + 1);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let id: i32 = parts[0].parse().expect("Please enter a valid ID");
        let name = parts[1].to_string();
        students.push(Student { id, name });
    }

    print_students(&students);
}

"""
#反馈专家Prompt
# Feedback_prompt =""""""
Feedback_prompt = """
You are given an error from clippy and Rust code snippets from one or more '. rs ' files related to this error.
Instructions: 
Fix the error on the code snippets. 
Not every snippet might require a fix or be relevant to the error, but take into account the code in all above snippets as it could help you derive the best possible fix.
Assume that the snippets might not be complete and could be missing lines above or below. 
Do not add comments or code that is not necessary to fix the error. 
Do not use unsafe or unstable features ( through '#![ feature (...) ]').
For your answer, return one or more ChangeLog groups, each containing one or more fixes to the above code snippets. 
Each group must be formatted with the below instructions.
Format instructions: 
Each ChangeLog group must start with a description of its included fixes. 
The group must then list one or more pairs of (OriginalCode , FixedCode) code snippets. 
Each OriginalCode snippet must list all consecutive original lines of code that must be replaced ( including a few lines before and after the fixes ), followed by the FixedCode snippet with all consecutive fixed lines of code that must replace the original lines of code (including the same few lines before and after the changes).
In each pair, the OriginalCode and FixedCode snippets must start at the same source code line number N. 
Each listed code line, in both the OriginalCode and FixedCode snippets, must be prefixed with [ N ] that matches the line index N in the above snippets, and then be prefixed with exactly the same whitespace indentation as the original snippets above.
Make sure that OriginalCode snippet and FixedCode snippet correspond to the same range, that is, if this changelog is adopted, then simply replacing OriginalCode snippet with FixedCode snippet is sufficient.
If the OriginalCode should be deleted, then keep the corresponding FixedCode snippets empty.
Input Format:
<error_information>
---
<code_snippets>

Output Format:
ChangeLog:1@<file>
FixDescription:<summary>.
OriginalCode@4-6:
[4] <white space> <original code line>
[5] <white space> <original code line>
[6] <white space> <original code line>
FixedCode@4-6:
[4] <white space> <fixed code line>
[5] <white space> <fixed code line>
[6] <white space> <fixed code line>
OriginalCode@9-10:
[9] <white space> <original code line>
[10] <white space> <original code line>
FixedCode@9-9:
[9] <white space> <fixed code line>
...
ChangeLog:K@<file>
FixDescription : <summary>.
OriginalCode@15-16:
[15] <white space> <original code line>
[16] <white space> <original code line>
FixedCode@15-17:
[15] <white space> <fixed code line>
[16] <white space> <fixed code line>
[17] <white space> <fixed code line>
OriginalCode@23-23:
[23] <white space> <original code line>
FixedCode@23-23:
[23] <white space> <fixed code line>
 
Example input:
Input:
error[E0515]: cannot return value referencing temporary value
−−> src/example.rs:21:4
|
21 | self.map.write().unwrap().entry(key).or_insert(Bar::new())
| −−−−−−−−−−−−−−−−−−−^^^^^^^^^^^^^^^^^^^^^^^^^
| |
| returns a value referencing data owned by the current function
| temporary value created here
---
[15] struct Foo { map: RwLock<HashMap<String, Bar>> }
[16]
[17] impl Foo {
[18]    pub fn get(&self, key: String) → &Bar {
[19]        self.map.write().unwrap().entry(key).or_insert(Bar::new())
[20]    }
[21] }

Example output:
ChangeLog:1@src/example.rs
FixDescription: Change the return type of the 'get' method to return an
Arc<Bar> and wrap the Bar in an Arc when inserting it into the HashMap.
OriginalCode@19−23:
[19] impl Foo {
[20]    pub fn get(&self, key: String) → &Bar {
[21]        self.map.write().unwrap().entry(key).or_insert(Bar::new())
[22]    }
[23] }
FixedCode@19−24:
[19] impl Foo {
[20]    pub fn get(&self, key: String) → std::sync::Arc<Bar> {
[21]        self.map.write().unwrap().entry(key).or_insert_with(
[22]        || std::sync::Arc::new(Bar::new())).clone()
[23]    }
[24] }
"""



# Feedback_prompt = """
# Carefully analyze the following Rust code issues and provide specific, clear suggestions for fixes. 
# Issues may come from static analysis errors, compilation errors, or output mismatches.

# Issue type: [Static Analysis Error / Compilation Error / Output Mismatch]
# Issue description:
# [Detailed issue description]

# Current Rust code:
# [Complete Rust code]

# Please respond in the following format:
# 1. Issue: [Brief description of the problem]
#    Location: [Indicate the specific line number or code segment where the problem occurs]
#    Suggestion: [Provide specific, actionable suggestions for fixes, including how the code should be modified]
# 2. Issue: [If there are multiple issues, continue listing them in the above format]

# Example 1 (Static Analysis Error):
# Issue type: Static Analysis Error
# Issue description:
# error[E0308]: mismatched types
#  --> src/main.rs:3:14
#   |
# 3 |     let x: i32 = "hello";
#   |              -- ^^^^^^^ expected `i32`, found `&str`
#   |              |
#   |              expected due to this

# Current Rust code:
# fn main() {
#     let x: i32 = "hello";
#     println!("x = {}", x);
# }

# Example output 1:
# 1. Issue: Type mismatch error
#    Location: Line 3 `let x: i32 = "hello";`
#    Suggestion: Based on your actual needs, you can modify as follows:
#                1. If you need an integer, use a valid i32 value: `let x: i32 = 42;`
#                2. If you need a string, change the variable type: `let x: &str = "hello";`

# Example 2 (Compilation Error):
# Issue type: Compilation Error
# Issue description:
# error[E0425]: cannot find value `y` in this scope
#  --> src/main.rs:3:20
#   |
# 3 |     println!("{}", y);
#   |                    ^ not found in this scope

# Current Rust code:
# fn main() {
#     let x = 5;
#     println!("{}", y);
# }

# Example output 2:
# 1. Issue: Undefined variable
#    Location: Line 3 `println!("{}", y);`
#    Suggestion: Ensure the variable is defined before use. Possible solutions:
#                1. If you want to print the value of `x`, change `y` to `x`: `println!("{}", x);`
#                2. If you really need to use `y`, define it before use: `let y = 10;`

# Example 3 (Output Mismatch):
# Issue type: Output Mismatch

# Issue description:
# C output:
# The sum of even numbers is: 6
# Rust output:
# The sum of even numbers is: 15

# Current Rust code:
# fn main() {
#     let numbers = vec![1, 2, 3, 4, 5];
#     let sum: i32 = numbers.iter().sum();
#     println!("The sum of even numbers is: {}", sum);
# }

# Example output 3:
# 1. Issue: Sum logic error
#    Location: Line 3 `let sum: i32 = numbers.iter().sum();`
#    Suggestion: Modify the sum logic to only sum even numbers. You can use the filter method to achieve this:
#                `let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();`
#                This ensures that only even numbers are included in the sum.

# Please ensure the analysis covers all error-level issues and provide clear, specific suggestions for fixes. Completely ignore any warnings.
# """





# 优化专家Prompt
Optimize_prompt = """
Optimize the given Rust code based on the provided feedback. 
The feedback may involve static analysis errors, compilation errors, or output mismatches.

Note: For static analysis results, only consider error-level issues and completely ignore warnings.

Feedback:
[Detailed feedback content]

Current Rust code:
[Complete Rust code]

Please provide the complete optimized Rust code directly, including all necessary comments. No additional explanations are needed.

Example 1 (Static analysis error):

Feedback:
1.Issue: Type mismatch error Location: Line 4 let x: i32 = "world"; Suggestion: Use the correct type according to the actual requirement.  
2.Issue: Unused variable Location: Line 6 let z = 10; Suggestion: Remove the unused variable or use it in the code.  
3.Issue: Division by zero Location: Line 8 let result = 10 / y; Suggestion: Ensure the divisor is not zero before performing the division.  
4.Issue: Borrow checker error Location: Line 12 println!("{}", s); Suggestion: Ensure the variable is not moved or borrowed in a way that violates Rust's borrowing rules.  
5.Issue: Index out of bounds Location: Line 15 println!("{}", arr[10]); Suggestion: Ensure the index is within the bounds of the array.Current Rust code:
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
    // Modified: Changed string type to i32, assuming we need an integer
    let x: i32 = 42;
    println!("x = {}", x);

    let y = 1; // Modified: Ensure the divisor is not zero
    let result = 10 / y;
    println!("Result is: {}", result);

    // Removed: Unused variable z

    let s = String::from("hello");
    println!("{}", s);
    // Modified: Avoid double borrowing by cloning the string
    println!("{}", s.clone());

    let arr = [1, 2, 3, 4, 5];
    // Modified: Ensure the index is within the bounds of the array
    if arr.len() > 10 {
        println!("{}", arr[10]);
    } else {
        println!("Index out of bounds");
    }
}
Please ensure the optimized code resolves all error issues mentioned in the feedback, and keep the code clear and concise.
Completely ignore any warnings.
"""




planning_agent_prompt_compiler_error="""

"""

system_prompt="You are an experienced C and Rust developer."



align_prompt="""
### C
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
### Rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("The sum of even numbers is: {}", sum);
}
Instructions:
You are an experienced C and Rust developer. The above Rust code is translated from C code, please match the above translation sentence by sentence, following the format of the above sample.
Do not change the content of these two pieces of code.
Do not output any other detailed analysis information.
output:
### C
void main() {   // --- C stmt 1
    int numbers[] = {1, 2, 3, 4, 5};   // --- C stmt 2
    int sum = 0;   // --- C stmt 3
    for (int i = 0; i < sizeof(numbers) / sizeof(numbers[0]); i++) {   // --- C stmt 4
        if (numbers[i] % 2 == 0) {   // --- C stmt 5
            sum += numbers[i];   // --- C stmt 6
        }   // --- C stmt 7
    }   // --- C stmt 8
    printf("The sum of even numbers is: %d\n", sum);   // --- C stmt 9
}   // --- C stmt 10
### Rust
fn main() {   // --- C stmt 1
    let numbers = vec![1, 2, 3, 4, 5];   // --- C stmt 2
    let sum: i32 = numbers.iter().sum();   // --- C stmt 3, C stmt 4, C stmt 5, C stmt 6, C stmt 7, C stmt 8
    println!("The sum of even numbers is: {}", sum);   // --- C stmt 9
}   // --- C stmt 10
-----
### C
{c_code}
### Rust
{rust_code}
Instructions:You are an experienced C and Rust developer. The above Rust code is translated from C code, please match the above translation sentence by sentence, following the format of the above sample.
Do not change the content of these two pieces of code.
Do not output any other detailed analysis information.
output:
"""

consistency_analyze_and_fix_prompt="""
### C
void main() {   // --- C stmt 1
    int numbers[] = {1, 2, 3, 4, 5};   // --- C stmt 2
    int sum = 0;   // --- C stmt 3
    for (int i = 0; i < sizeof(numbers) / sizeof(numbers[0]); i++) {   // --- C stmt 4
        if (numbers[i] % 2 == 0) {   // --- C stmt 5
            sum += numbers[i];   // --- C stmt 6
        }   // --- C stmt 7
    }   // --- C stmt 8
    printf("The sum of even numbers is: %d\n", sum);   // --- C stmt 9
}   // --- C stmt 10
### Rust
fn main() {   // --- C stmt 1
    let numbers = vec![1, 2, 3, 4, 5];   // --- C stmt 2
    let sum: i32 = numbers.iter().sum();   // --- C stmt 3, C stmt 4, C stmt 5, C stmt 6, C stmt 7, C stmt 8
    println!("The sum of even numbers is: {}", sum);   // --- C stmt 9
}   // --- C stmt 10
### Error message
Output mismatch:
C output:
The sum of even numbers is: 6
Rust output:
The sum of even numbers is: 15
### Instructions:
You are an experienced C and Rust developer. 
Above Rust code failed correctness testing due to logical and behavioral inconsistency with C code, following some error messages caused by inconsistency.
Compare and analyze the logic of the above two pieces of code and list the logical and behavioral inconsistencies between the C code and the Rust code.
Then, for each inconsistency you have listed, locate the specific location of the error Rust code and provide a fixed Rust code in a series of issues.
The error code and fix code should not have alignment comments.
Do not add comments or code that is not necessary to fix the error. 
Format instructions: 
Firstly you should list the logical and behavioral inconsistencies.
Then give the corresponding fix issues.
Fix of each error Rust code snippet should be placed separately in an issue.
Each issue should start with the summary of inconsistency to solve.
Then give the error Rust code snippet that caused the inconsistency, followed by a fixed code snippet.
output:
### List of inconsistencies
1. Sum seeking logic inconsistency
In C code, only even numbers are summed: if (numbers [i]% 2==0) {sum+=numbers [i];}
Directly summing all elements in Rust code: let sum: i32=numbers. iter(). sum();
Rust code does not implement the even sum logic in C code.
### Issues
#Issue 1
Inconsistency: List of inconsistencies
Error code:
let sum: i32=numbers.iter().sum();
Fixed Code:
let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
-----
### C
{current_rust_code}
### Rust
{aligend_rust_code}
### Error message
{error_message}
### Instructions:
You are an experienced C and Rust developer. 
Above Rust code failed correctness testing due to logical and behavioral inconsistency with C code, following some error messages caused by inconsistency.
Compare and analyze the logic of the above two pieces of code and list the logical and behavioral inconsistencies between the C code and the Rust code.
Then, for each inconsistency you have listed, locate the specific location of the error Rust code and provide a fixed Rust code in a series of issues.
The error code and fix code should not have alignment comments.
Do not add comments or code that is not necessary to fix the error. 
Format instructions: 
Firstly you should list the logical and behavioral inconsistencies.
Then give the corresponding fix issues.
Fix of each error Rust code snippet should be placed separately in an issue.
Each issue should start with the summary of inconsistency to solve.
Then give the error Rust code snippet that caused the inconsistency, followed by a fixed code snippet.
output:
"""




Optimize_prompt="""
### Current rust code
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("The sum of even numbers is: {}", sum);
}
### Issues
#Issue 1
Inconsistency: List of inconsistencies
Error code:
let sum: i32=numbers.iter().sum();
Fixed Code:
let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
### Instructions:
Given above flawed Rust code and its fix issues.
Analyze each fix issue, and when you think an issue can be adopted, fix the current Rust code based on the error code and fix code it provides.
After completing the analysis of all issues, provide the repaired Rust code.
Only output the repaired Rust code.
Do not output any detailed analysis information.
output:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("The sum of even numbers is: {}", sum);
}
-----
### Current rust code
{current_rust_code}
### Issues
{issues}
### Instructions:
Given above flawed Rust code and its fix issues.
Analyze each fix issue, and when you think an issue can be adopted, fix the current Rust code based on the error code and fix code it provides.
After completing the analysis of all issues, provide the repaired Rust code.
Only output the repaired Rust code.
Do not output any detailed analysis information.
output:
"""



align_shot="""
-----
### C
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
### Rust
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
##### Task:You are an experienced C and Rust developer. The above Rust code is translated from C code, please match the above translation sentence by sentence, following the format of the above sample.
output:
### C
void reverse_array(int arr[], int size) {   // --- C stmt 1
    int start = 0;   // --- C stmt 2
    int end = size - 1;   // --- C stmt 3
    while (start < end) {   // --- C stmt 4
        int temp = arr[start];   // --- C stmt 5
        arr[start] = arr[end];   // --- C stmt 6
        arr[end] = temp;   // --- C stmt 7
        start++;   // --- C stmt 8
        end--;   // --- C stmt 9
    }   // --- C stmt 10
}   // --- C stmt 11
int main() {   // --- C stmt 12
    int arr[] = {1, 2, 3, 4, 5};   // --- C stmt 13
    int size = sizeof(arr) / sizeof(arr[0]);   // --- C stmt 14
    reverse_array(arr, size);   // --- C stmt 15
    for (int i = 0; i < size; i++) {   // --- C stmt 16
        printf("%d ", arr[i]);   // --- C stmt 17
    }   // --- C stmt 18
    printf("\n");   // --- C stmt 19
    return 0;   // --- C stmt 20
}   // --- C stmt 21
### Rust
fn reverse_array(arr: &mut [i32]) { // --- C stmt 1 
    let len = arr.len(); // --- C stmt 2 
    for i in 0..len / 2 { // --- C stmt 3, C stmt 4 
        let temp = arr[i]; // --- C stmt 5 
        arr[i] = arr[len - i]; // --- C stmt 6 
        arr[len - i] = temp; // --- C stmt 7 
    } // --- C stmt 10 
} // --- C stmt 11
fn main() { // --- C stmt 12 
    let mut arr = [1, 2, 3, 4, 5]; // --- C stmt 13 
    reverse_array(&mut arr); // --- C stmt 15 
    for &item in &arr { // --- C stmt 16 
        print!("{} ", item); // --- C stmt 17 
    } // --- C stmt 18 
    println!(); // --- C stmt 19 
} // --- C stmt 21
"""