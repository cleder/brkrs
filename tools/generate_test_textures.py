#!/usr/bin/env python3
"""Generate placeholder test textures for PBR material testing"""
import numpy as np
from PIL import Image


def main():
    # Create 256x256 test ORM texture (Red=occlusion, Green=roughness, Blue=metallic)
    orm = np.zeros((256, 256, 3), dtype=np.uint8)
    orm[:, :, 0] = int(0.5 * 255)  # Occlusion (red channel)
    orm[:, :, 1] = int(0.7 * 255)  # Roughness (green channel)
    orm[:, :, 2] = int(0.3 * 255)  # Metallic (blue channel)
    Image.fromarray(orm).save("tests/fixtures/textures/test_orm.png")
    print("✓ Created test_orm.png (ORM: R=0.5, G=0.7, B=0.3)")

    # Create 256x256 emissive texture (red glow pattern - radial gradient)
    emissive = np.zeros((256, 256, 3), dtype=np.uint8)
    y, x = np.ogrid[:256, :256]
    dist = np.sqrt((x - 128)**2 + (y - 128)**2)
    intensity = np.clip(1.0 - dist / 180.0, 0, 1)
    emissive[:, :, 0] = (intensity * 255).astype(np.uint8)  # Red glow
    Image.fromarray(emissive).save("tests/fixtures/textures/test_emissive.png")
    print("✓ Created test_emissive.png (radial red glow pattern)")

    # Create 256x256 depth texture (grayscale depth pattern - circular depression)
    depth = np.zeros((256, 256, 3), dtype=np.uint8)
    depth_value = np.clip(dist / 180.0, 0, 1)
    gray = (depth_value * 255).astype(np.uint8)
    depth[:, :, 0] = gray
    depth[:, :, 1] = gray
    depth[:, :, 2] = gray
    Image.fromarray(depth).save("tests/fixtures/textures/test_depth.png")
    print("✓ Created test_depth.png (grayscale circular depression)")


if __name__ == "__main__":
    main()
