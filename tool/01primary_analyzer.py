
"""
This module provides functionality for analyzing C code files and directories.

It uses the tree-sitter library to parse C code and extract various metrics
such as total lines, function count, function sizes, and comment lines.
"""
import os
import argparse
import matplotlib.pyplot as plt

from tree_sitter_c_config import c_parser, C_LANGUAGE

def analyze_file(file_path):
    """
    Analyze a single C file and extract various metrics.
    """
    with open(file_path, 'r', encoding='utf-8') as file:
        content = file.read()
    
    total_lines = len(content.splitlines())
    
    tree = c_parser.parse(bytes(content, 'utf8'))

    comment_lines = 0
    comment_query = C_LANGUAGE.query("(comment) @comment")
    comment_captures = comment_query.captures(tree.root_node)
    for capture in comment_captures:
        comment_node = capture[0]
        comment_lines += comment_node.end_point[0] - comment_node.start_point[0] + 1
    
    query = C_LANGUAGE.query("(function_definition) @function")
    
    captures = query.captures(tree.root_node)
    functions = [capture[0] for capture in captures if capture[1] == 'function']
    
    function_count = len(functions)
    
    if functions:
        function_lines = [node.end_point[0] - node.start_point[0] + 1 for node in functions]
        min_function_lines = min(function_lines)
        max_function_lines = max(function_lines)
    else:
        min_function_lines = max_function_lines = 0
    
    return total_lines, function_count, min_function_lines, max_function_lines, comment_lines

def analyze_directory(directory):
    """
    Analyze all C files in the given directory and its subdirectories.
    """
    results = []
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.c'):
                file_path = os.path.join(root, file)
                result = analyze_file(file_path)
                results.append((file, *result))
    return results

def plot_results(results):
    """
    Plot the analysis results in three subplots.
    """
    files, total_lines, function_counts, min_lines, max_lines, comment_lines = zip(*results)
    
    files = [os.path.splitext(os.path.basename(file))[0] for file in files]
    
    fig, (ax1, ax2, ax3) = plt.subplots(3, 1, figsize=(10, 10))
    
    # Plot total lines and function count
    bars1 = ax1.bar(files, total_lines, label='Total Lines')
    bars2 = ax1.bar(files, function_counts, label='Function Count')
    ax1.set_title('Total Lines and Function Count')
    ax1.set_xticklabels(files, rotation=45, ha='right', fontsize=8)
    ax1.legend()
    ax1.bar_label(bars1, label_type='edge')
    ax1.bar_label(bars2, label_type='edge')
    
    # Plot maximum and minimum function lines
    bars3 = ax2.bar(files, max_lines, label='Maximum Function Lines')
    bars4 = ax2.bar(files, min_lines, label='Minimum Function Lines')
    ax2.set_title('Maximum and Minimum Function Lines')
    ax2.set_xticklabels(files, rotation=45, ha='right', fontsize=8)
    ax2.legend()
    ax2.bar_label(bars3, label_type='edge')
    ax2.bar_label(bars4, label_type='edge')
    
    # Plot comment lines
    bars5 = ax3.bar(files, comment_lines)
    ax3.set_title('Comment Lines')
    ax3.set_xticklabels(files, rotation=45, ha='right', fontsize=8)
    ax3.bar_label(bars5, label_type='edge')
    
    plt.tight_layout(h_pad=1.5)
    plt.show()

if __name__ == "__main__":
    # Set up command-line argument parser
    parser = argparse.ArgumentParser(description='Analyze C/C++ code in a directory.')
    parser.add_argument('--directory', default='./Input/01-Primary', type=str, help='Directory path to analyze')
    args = parser.parse_args()

    # Analyze the directory and get results
    results = analyze_directory(args.directory)

    # Print analysis results
    print("\nAnalysis Results:")
    for file, total_lines, func_count, min_lines, max_lines, comment_lines in results:
        print(f"File: {file}")
        print(f"  Total lines: {total_lines}")
        print(f"  Function count: {func_count}")
        print(f"  Minimum function lines: {min_lines}")
        print(f"  Maximum function lines: {max_lines}")
        print(f"  Comment lines: {comment_lines}")
        print()
    
    # Plot the results
    plot_results(results)
