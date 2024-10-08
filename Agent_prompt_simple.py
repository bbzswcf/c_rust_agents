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
Feedback_prompt = """
Carefully analyze the following Rust code issues and provide specific, clear suggestions for fixes. 
Issues may come from static analysis errors, compilation errors, or output mismatches.

Issue type: [Static Analysis Error / Compilation Error / Output Mismatch]
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
Issue type: Static Analysis Error
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

Example 3 (Output Mismatch):
Issue type: Output Mismatch

Issue description:
C output:
The sum of even numbers is: 6
Rust output:
The sum of even numbers is: 15

Current Rust code:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("The sum of even numbers is: {}", sum);
}

Example output 3:
1. Issue: Sum logic error
   Location: Line 3 `let sum: i32 = numbers.iter().sum();`
   Suggestion: Modify the sum logic to only sum even numbers. You can use the filter method to achieve this:
               `let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();`
               This ensures that only even numbers are included in the sum.

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