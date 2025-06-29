#!/usr/bin/env python3
import json
import re

def snake_to_pascal(snake_str):
    """Convert snake_case to PascalCase"""
    # Handle special cases with numbers and hyphens
    components = re.split(r'[-_]', snake_str)
    # Convert each component to title case, handling numbers properly
    pascal_components = []
    for component in components:
        if component.isdigit():
            pascal_components.append(component)
        else:
            pascal_components.append(component.capitalize())
    return ''.join(pascal_components)

def generate_rust_icons():
    """Generate Rust icon definitions from info.json"""
    
    # Read the JSON file
    with open('info.json', 'r') as f:
        icons = json.load(f)
    
    # Generate icon definitions
    icon_definitions = []
    
    for icon_name, icon_data in icons.items():
        # Convert kebab-case to PascalCase for Rust
        rust_name = snake_to_pascal(icon_name)
        unicode_str = icon_data['encodedCode'].replace('\\', '\\u{').replace('e', '') + '}'
        
        icon_definitions.append(f'            {rust_name} : &str = "{unicode_str}",')
    
    # Sort alphabetically
    icon_definitions.sort()
    
    # Generate the complete Rust file content
    rust_content = '''#![allow(non_snake_case)]
use super::macros::IconSize;
use crate::define_font_icon_category;
use bevy::prelude::*;

define_font_icon_category! {
    Interface {
        path: "ui/icons/interface/lucide.ttf",
        icons: [
''' + '\n'.join(icon_definitions) + '''
        ]
    }
}
'''
    
    return rust_content

if __name__ == "__main__":
    content = generate_rust_icons()
    with open('interface_generated.rs', 'w') as f:
        f.write(content)
    print("Generated interface_generated.rs")