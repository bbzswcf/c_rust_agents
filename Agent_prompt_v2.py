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

Feedback_prompt="""Following is a Rust code translated from a C code, accompanied by its original C code.
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

Optimize_prompt = """
Given a flawed Rust code and its fix issues.
Analyze each fix issue, and when you think an issue can be adopted, fix the current Rust code based on the error code and fix code it provides.
After completing the analysis of all issues, provide the repaired Rust code.

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
Errors: Output mismatch
Reason: C code calculates the sum of all even numbers in an array, while Rust code calculates the sum of all elements in the array.
Error Code:
let sum: i32 = numbers.iter().sum();
Fix Code:
let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();

Fixed Rust Code:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("The sum of even numbers is: {}", sum);
}
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
In each pair, the OriginalCode and FixedCode snippets must start at the same source code line number N. 
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
api_input_prompt="""
        Extract and convert only the C-specific APIs to their Rust equivalents:
        {c_code}
"""
convert_input_prompt = """
    Convert the following C code to Rust using the provided API mappings:
    C code:
    {c_code}
    API mappings:
    {api_conversion}
    Remember to output only the converted Rust code without any explanations.
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
