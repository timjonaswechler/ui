#!/usr/bin/env python3
"""
SVG Texture Atlas Generator
Converts SVG icons to white and creates a texture atlas in a 20x15 grid
"""

import os
import glob
from PIL import Image, ImageDraw
import cairosvg
import io

def svg_to_white_png(svg_path, size=64):
    """Convert SVG to white PNG with transparent background"""
    try:
        # Read SVG content
        with open(svg_path, 'r', encoding='utf-8') as f:
            svg_content = f.read()

        # Replace any fill colors with white, but preserve 'none'
        svg_content = svg_content.replace('fill="black"', 'fill="white"')
        svg_content = svg_content.replace('fill="#000"', 'fill="white"')
        svg_content = svg_content.replace('fill="#000000"', 'fill="white"')
        svg_content = svg_content.replace('stroke="black"', 'stroke="white"')
        svg_content = svg_content.replace('stroke="#000"', 'stroke="white"')
        svg_content = svg_content.replace('stroke="#000000"', 'stroke="white"')

        # Add white fill to paths that don't have explicit fills
        if 'fill=' not in svg_content and '<path' in svg_content:
            svg_content = svg_content.replace('<path', '<path fill="white"')

        # Convert SVG to PNG bytes
        png_bytes = cairosvg.svg2png(
            bytestring=svg_content.encode('utf-8'),
            output_width=size,
            output_height=size
        )

        # Create PIL Image from bytes
        img = Image.open(io.BytesIO(png_bytes)).convert("RGBA")

        # Create white version with preserved alpha
        white_img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
        pixels = img.load()
        white_pixels = white_img.load()

        for y in range(img.height):
            for x in range(img.width):
                r, g, b, a = pixels[x, y]
                if a > 0:  # If pixel is not transparent
                    white_pixels[x, y] = (255, 255, 255, a)  # Make it white but keep alpha

        return white_img

    except Exception as e:
        print(f"Error processing {svg_path}: {e}")
        # Return a blank white square as fallback
        fallback = Image.new("RGBA", (size, size), (255, 255, 255, 255))
        return fallback

def create_texture_atlas(svg_folder, output_folder, icon_size=64, grid_cols=20, grid_rows=15):
    """Create texture atlas from SVG icons"""

    # Create output folder if it doesn't exist
    os.makedirs(output_folder, exist_ok=True)

    # Get all SVG files
    svg_files = glob.glob(os.path.join(svg_folder, "*.svg"))
    svg_files.sort()  # Sort for consistent ordering

    # Limit to grid capacity
    max_icons = grid_cols * grid_rows
    svg_files = svg_files[:max_icons]

    print(f"Processing {len(svg_files)} SVG files...")
    print(f"Creating {grid_cols}x{grid_rows} texture atlas with {icon_size}x{icon_size} icons")

    # Calculate atlas dimensions
    atlas_width = grid_cols * icon_size
    atlas_height = grid_rows * icon_size

    # Create the atlas image
    atlas = Image.new("RGBA", (atlas_width, atlas_height), (0, 0, 0, 0))

    # Process each SVG and place in atlas
    for i, svg_file in enumerate(svg_files):
        print(f"Processing {i+1}/{len(svg_files)}: {os.path.basename(svg_file)}")

        # Convert SVG to white PNG
        icon = svg_to_white_png(svg_file, icon_size)

        # Calculate position in grid
        col = i % grid_cols
        row = i // grid_cols

        # Calculate pixel position
        x = col * icon_size
        y = row * icon_size

        # Paste icon into atlas
        atlas.paste(icon, (x, y), icon)

    # Save the texture atlas
    atlas_path = os.path.join(output_folder, f"texture_atlas_{grid_cols}x{grid_rows}_{icon_size}px.png")
    atlas.save(atlas_path, "PNG")

    print(f"Texture atlas saved to: {atlas_path}")
    print(f"Atlas dimensions: {atlas_width}x{atlas_height} pixels")
    info_file_name = f"icon_mapping_{icon_size}.txt"
    # Create a mapping file for reference
    mapping_path = os.path.join(output_folder, info_file_name)
    with open(mapping_path, 'w') as f:
        f.write(f"Texture Atlas Icon Mapping ({grid_cols}x{grid_rows})\n")
        f.write(f"Icon size: {icon_size}x{icon_size} pixels\n")
        f.write(f"Atlas size: {atlas_width}x{atlas_height} pixels\n")
        f.write("-" * 50 + "\n")

        for i, svg_file in enumerate(svg_files):
            col = i % grid_cols
            row = i // grid_cols
            x = col * icon_size
            y = row * icon_size
            icon_name = os.path.splitext(os.path.basename(svg_file))[0]
            f.write(f"Index {i:3d}: {icon_name:<30} | Grid: ({col:2d},{row:2d}) | Pixel: ({x:3d},{y:3d})\n")

    print(f"Icon mapping saved to: {mapping_path}")

if __name__ == "__main__":
    # Configuration
    SVG_FOLDER = "/Users/tim-jonaswechler/GitHub-Projekte/test/ui/assets/ui/svg"
    OUTPUT_FOLDER = "/Users/tim-jonaswechler/GitHub-Projekte/test/ui/assets/ui"
    ICON_SIZE = 16  # Size of each icon in pixels
    GRID_COLS = 20  # Number of columns
    GRID_ROWS = 16  # Number of rows

    # Create texture atlas
    create_texture_atlas(SVG_FOLDER, OUTPUT_FOLDER, ICON_SIZE, GRID_COLS, GRID_ROWS)
