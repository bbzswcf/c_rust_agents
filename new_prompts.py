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
You are an experienced C and Rust developer. 
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

Optimize_prompt = """
You are an experienced C and Rust developer.
Given a flawed Rust code and its fix issues.
Analyze each fix issue, and when you think an issue can be adopted, fix the current Rust code based on the error code and fix code it provides.
After completing the analysis of all issues, provide the repaired Rust code.
Only output the repaired Rust code.Do not output any detailed analysis information.
---
Example 1:
Current Rust Code:
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
fn reverse_array(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = arr[i];
        arr[i] = arr[len - 1 - i];
        arr[len - 1 - i] = temp;
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
---
Example 2:
Current Rust Code:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("The sum of even numbers is: {}", sum);
}

Fix issues:
#Issue 1
Inconsistency: List of inconsistencies
Error code:
let sum: i32=numbers.iter().sum();
Fixed Code:
let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();

Fixed Rust Code:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("The sum of even numbers is: {}", sum);
}
"""

# 修复
Optimize_prompt="""
You are an experienced C and Rust developer.
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

Feedback_prompt="""
You are an experienced C and Rust developer.
"""

align_prompt_0="""
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
"""
align_prompt_1="""
### C
{c_code}
### Rust
{rust_code}
Instructions:You are an experienced C and Rust developer. The above Rust code is translated from C code, please match the above translation sentence by sentence, following the format of the above sample.
Do not change the content of these two pieces of code.
Do not output any other detailed analysis information.
output:
"""

consistency_analyze_and_fix_prompt_0="""
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
Firstly you should list the logical and behavioral inconsistencies in terms of function.
For each function, a inconsistency should contain a detailed explanation of the differences between C and Rust code.
---
In function <function name>:
1. <summary>
In C code, <explanation>
In Rust code, <explanation>
...
k. <summary>
In C code, <explanation>
In Rust code, <explanation>
In function <function name>:
1. <summary>
In C code, <explanation>
In Rust code, <explanation>
...
k. <summary>
In C code, <explanation>
In Rust code, <explanation>
...
---
Then give the corresponding fix issues.
Fix of each error Rust code snippet should be placed separately in an issue.
Each issue should start with the summary of inconsistency to solve.
Then give the error Rust code snippet that caused the inconsistency, followed by a fixed code snippet.
output:
### List of inconsistencies
In function main:
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
"""
consistency_analyze_and_fix_prompt_1="""
### C
{aligend_c_code}
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

compiler_error_prompt_0="""
The above Rust code is translated from C code, followed by the error message of Rust code runtime error.
### Original C Code:
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
### Translated Rust Code:
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
### Error message:
thread 'main' panicked at test.rs:6:18:
index out of bounds: the len is 5 but the index is 5
### Instructions:
Analyze the above error and refer to the original C code to infer the cause of the error.
You need to locate the Rust code snippets that caused the error and provide the corresponding fixed Rust code snippets in a series of issues
Note that there may be multiple errors caused by the same error Rust code snippet.
Do not add comments or code that is not necessary to fix the error. 
Format Instruction:
Fix of each error Rust code snippet should be placed separately in an issue, noted that this error Rust code snippet may involve one or multiple errors.
Each issue should start with the summary of errors it is involved.
Then provide your speculated reasons for these errors, and give the Rust code snippet that caused the errors, followed by a fix code snippet.
Number of issues should not exceed 5.
output:
#Issue 1
Errors: Array out of bounds access.
Reason: When i is 0, arr [len-i] is actually arr [len], which exceeds the valid index range of the array (the valid index range of the array is 0 to len-1).
Error Code:
arr[i] = arr[len - i];
arr[len - i] = temp;
Fix Code:
arr[i] = arr[len - 1 - i];
arr[len - 1 - i] = temp;
"""
compiler_error_prompt_1="""
---
The above Rust code is translated from C code, followed by the error message of Rust code runtime error.
### Original C Code:
{c_code}
### Translated Rust Code
{rust_code}
### Error message:
{error_message}
### Instructions:
Analyze the above error and refer to the original C code to infer the cause of the error.
You need to locate the Rust code snippets that caused the error and provide the corresponding fixed Rust code snippets in a series of issues
Note that there may be multiple errors caused by the same error Rust code snippet.
Do not add comments or code that is not necessary to fix the error. 
Format Instruction:
Fix of each error Rust code snippet should be placed separately in an issue, noted that this error Rust code snippet may involve one or multiple errors.
Each issue should start with the summary of errors it is involved.
Then provide your speculated reasons for these errors, and give the Rust code snippet that caused the errors, followed by a fix code snippet.
Number of issues should not exceed 5.
output:
"""