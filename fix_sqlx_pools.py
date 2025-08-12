#!/usr/bin/env python3
"""
Script to fix SQLx 0.8 pool dereferencing patterns.
Replaces pool with &*pool (reverting previous change) and then applies correct SQLx 0.8 patterns.
"""

import os
import re
from pathlib import Path

def fix_pool_patterns(file_path):
    """Fix pool patterns for SQLx 0.8 compatibility."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # First, revert pool back to &*pool where it was changed incorrectly
        # Look for patterns like .execute(pool), .fetch_one(pool), etc.
        content = re.sub(r'\.execute\(pool\)', '.execute(&*pool)', content)
        content = re.sub(r'\.fetch_one\(pool\)', '.fetch_one(&*pool)', content)
        content = re.sub(r'\.fetch_all\(pool\)', '.fetch_all(&*pool)', content)
        content = re.sub(r'\.fetch_optional\(pool\)', '.fetch_optional(&*pool)', content)
        
        # Handle function calls that pass pool as parameter
        content = re.sub(r'([a-zA-Z_][a-zA-Z0-9_]*::)?([a-zA-Z_][a-zA-Z0-9_]*)\(pool,', r'\1\2(&*pool,', content)
        content = re.sub(r'([a-zA-Z_][a-zA-Z0-9_]*::)?([a-zA-Z_][a-zA-Z0-9_]*)\(pool\)', r'\1\2(&*pool)', content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        return False
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Main function to process all Rust files."""
    src_dir = Path("C:/Users/jjgor/JSG-StoryWeaver/src-tauri/src")
    
    if not src_dir.exists():
        print(f"Source directory not found: {src_dir}")
        return
    
    rust_files = list(src_dir.rglob("*.rs"))
    print(f"Found {len(rust_files)} Rust files")
    
    fixed_count = 0
    for file_path in rust_files:
        if fix_pool_patterns(file_path):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()