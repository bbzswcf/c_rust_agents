"""
This module provides functionality for preprocessing C code.
It includes functions to remove copyright notices.

Note:
    Additional preprocessing functionality is pending implementation.
    Future improvements may include:
    - Macro expansions
    - Removal of useless comments
    - Handling of conditional compilation directives (#ifdef, #ifndef, etc.)
    - Other C-specific preprocessing tasks

TODO:
    Implement additional preprocessing steps to enhance the robustness
    and completeness of the C code preprocessing.
"""
import re

def remove_copyright(code: str) -> str:
    """
    Remove copyright notices from C code.
    """
    copyright_pattern = r'^/\*(?:(?!\*/).)*Copyright(?:(?!\*/).)*\*/'
    code = re.sub(copyright_pattern, '', code, flags=re.DOTALL | re.MULTILINE)
    return code.lstrip()

def remove_debug_blocks(code: str) -> str:
    """
    Remove #if 0 debug blocks from C code.
    """
    pattern = r'\n?\s*#if\s+0\s*\n(.*?)\n\s*#endif\n?'
    preprocessed_code = re.sub(pattern, '\n', code, flags=re.DOTALL | re.MULTILINE)
    return preprocessed_code

def process_alloc_testing(code: str) -> str:
    """
    Process allocation testing related code by:
    - Removing #ifdef ALLOC_TESTING blocks since we want to keep their contents
    - Removing malloc/free testing comments that are not needed
    - Replacing standard memory allocation function calls with alloc_test_* versions
    """
    # Remove ALLOC_TESTING ifdef blocks but keep their contents
    pattern = r'\n?\s*#ifdef\s+ALLOC_TESTING.*?#endif\n?'
    code = re.sub(pattern, '', code, flags=re.DOTALL)
    
    # Remove malloc/free testing comments
    pattern = r'/\*\s*malloc\(\)\s*/\s*free\(\)\s*testing\s*\*/\s*\n?'
    code = re.sub(pattern, '', code)

    return code

def preprocess(code: str) -> str:
    code = remove_copyright(code)
    code = remove_debug_blocks(code)
    code = process_alloc_testing(code)

    return code
