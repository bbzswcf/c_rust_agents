# 语法专家Prompt
Syntax_prompt = """
Only output the converted Rust code without any explanations.
Ensure the following:
1. All variables and functions have correct type annotations.
2. The types of arguments passed in function calls match the parameter types in function declarations.
3. Use explicit type conversions as much as possible to avoid type mismatches.
4. Pay attention to precise matching of integer types, such as i32, u32, i64, u64, etc.
5. For pointer types, use appropriate Rust equivalents like &, &mut, Box<T>, etc.
6. Use Result and Option types to handle possible errors and null values.
7. Handle ownership and borrowing to adhere to Rust's ownership model.
8. Convert C structs, enums, and unions to Rust equivalents accurately.

Example input:
C code:
#include <stdio.h>
#include <stdlib.h>
typedef struct {
    int id;
    char name[50];
} Student;
union Data {
    int i;
    float f;
} data;
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
Example output:
use std::io;
struct Student {
    id: i32,
    name: String,
}
union Data {
    i: i32,
    f: f32,
}
fn print_students(students: &[Student]) {
    for student in students {
        println!("ID: {}, Name: {}", student.id, student.name);
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n = String::new();
    println!("Enter number of students: ");
    io::stdin().read_line(&mut n)?;
    let n: usize = n.trim().parse()?;

    let mut students = Vec::with_capacity(n);

    for i in 0..n {
        let mut input = String::new();
        println!("Enter ID and Name for student {}: ", i + 1);
        io::stdin().read_line(&mut input)?;
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let id: i32 = parts[0].parse()?;
        let name = parts[1].to_string();
        students.push(Student { id, name });
    }

    print_students(&students);
    Ok(())
}
"""
# 反馈专家prompt
Feedback_prompt = """
Carefully analyze the following Rust code issues and provide specific, clear suggestions for fixes.
Issues may come from static analysis errors, compilation errors, output mismatches, or performance and security concerns.

Note:
1. For static analysis results, focus only on error-level issues and completely ignore warnings.
2. Do not repeat the same or very similar issues. If you find multiple similar problems, merge them into a more comprehensive feedback.
3. Try to provide diverse feedback covering different types of issues (if they exist).
4. Before providing feedback, check and ensure there is no duplicate content.

Issue type: [Static Analysis Error / Compilation Error / Output Mismatch / Performance Issue / Security Concern]
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
Expected output:
The sum of even numbers is: 6
Actual output:
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

# 优化专家prompt
Optimize_prompt = """
Optimize the given Rust code based on the provided feedback. 
The feedback may involve static analysis errors, compilation errors, output mismatches, performance issues, or security concerns.
Note: For static analysis results, only consider error-level issues and completely ignore warnings.

Feedback:
[Detailed feedback content]

Current Rust code:
[Complete Rust code]

Please optimize the code following these steps:
1. Carefully read each piece of feedback.
2. For each issue, locate the corresponding position in the code.
3. Modify the code according to the feedback suggestions, avoiding new errors or warnings.
4. Add comments for each modification using single-line comments (`//`), briefly explaining the reason for the change.
5. Ensure all mentioned error issues are resolved.
6. If there are multiple possible implementations for a suggestion, choose the most appropriate one and explain the choice in a comment.
7. Completely ignore any warnings; do not process warnings.

Please provide the complete optimized Rust code directly, including all necessary comments. No additional explanations are needed.

Example 1 :

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
Please ensure the optimized code resolves all error issues mentioned in the feedback, and keep the code clear and concise. Completely ignore any warnings.
"""