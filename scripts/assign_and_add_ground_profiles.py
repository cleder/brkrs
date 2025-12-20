import os
import random
import sys
import glob
import re

LEVELS_DIR = os.path.join('assets', 'levels')
TEXTURE_DIR = os.path.join('assets', 'textures', 'background')
MANIFEST = os.path.join('assets', 'textures', 'manifest.ron')

def texture_to_profile(texture_filename):
    name, _ = os.path.splitext(texture_filename)
    return f'ground/{name}'

# --- PART 1: Assign random unique ground_profile to each level ---
level_files = sorted(glob.glob(os.path.join(LEVELS_DIR, '*.ron')))
texture_files = sorted([os.path.basename(f) for f in glob.glob(os.path.join(TEXTURE_DIR, '*')) if os.path.isfile(f)])

if len(texture_files) < len(level_files):
    print(f"Not enough unique textures ({len(texture_files)}) for all levels ({len(level_files)}). Aborting.")
    sys.exit(1)

random.shuffle(texture_files)
assigned = texture_files[:len(level_files)]

def update_presentation_block(content, level_number, ground_profile):
    presentation_re = re.compile(r'(presentation\s*:\s*Some\s*\(\()(.*?)(\)\)),', re.DOTALL)
    if presentation_re.search(content):
        # Replace the whole block
        new_block = f'presentation: Some((\n    level_number: {level_number},\n    ground_profile: Some("{ground_profile}"),\n    background_profile: None,\n    sidewall_profile: None,\n    tint: None,\n    notes: Some("Assigned by script"),\n  )),'
        return presentation_re.sub(new_block, content)
    # Otherwise, insert before the final closing parenthesis
    lines = content.rstrip().splitlines()
    for i in range(len(lines)-1, -1, -1):
        if lines[i].strip() == ')':
            insert_at = i
            break
    else:
        print("Could not find closing parenthesis in level file!")
        return content
    indent = '  ' if lines[0].startswith(' ') else ''
    pres_lines = [f'{indent}presentation: Some((',
                 f'{indent}  level_number: {level_number},',
                 f'{indent}  ground_profile: Some("{ground_profile}"),',
                 f'{indent}  background_profile: None,',
                 f'{indent}  sidewall_profile: None,',
                 f'{indent}  tint: None,',
                 f'{indent}  notes: Some("Assigned by script"),',
                 f'{indent})),']
    new_lines = lines[:insert_at] + pres_lines + lines[insert_at:]
    return '\n'.join(new_lines) + '\n'

assigned_profiles = []
for level_path, texture in zip(level_files, assigned):
    with open(level_path, 'r', encoding='utf-8') as f:
        content = f.read()
    m = re.search(r'number\s*:\s*(\d+)', content)
    if not m:
        print(f"Could not find level number in {level_path}, skipping.")
        continue
    level_number = int(m.group(1))
    ground_profile = texture_to_profile(texture)
    new_content = update_presentation_block(content, level_number, ground_profile)
    with open(level_path, 'w', encoding='utf-8') as f:
        f.write(new_content)
    assigned_profiles.append(ground_profile)
    print(f"Assigned {ground_profile} to {os.path.basename(level_path)}")

# --- PART 2: Add missing ground profiles to manifest.ron safely ---
# 1. Collect all ground_profile IDs used in levels (including just assigned)
ground_profiles = set(assigned_profiles)
profile_re = re.compile(r'ground_profile:\s*Some\("([^)\"]+)"\)')
for level_path in level_files:
    with open(level_path, 'r', encoding='utf-8') as f:
        content = f.read()
    for m in profile_re.finditer(content):
        ground_profiles.add(m.group(1))

# 2. Parse manifest.ron and find existing ground/ IDs
with open(MANIFEST, 'r', encoding='utf-8') as f:
    manifest = f.read()
existing_profiles = set(re.findall(r'id:\s*"(ground/[^\"]+)"', manifest))

# 3. For each missing ground_profile, add a profile block
missing = sorted([p for p in ground_profiles if p not in existing_profiles])
if not missing:
    print("No missing ground profiles to add.")
    sys.exit(0)

# 4. Find the profiles array and its closing bracket
profiles_start = manifest.find('profiles: [')
if profiles_start == -1:
    print("Could not find 'profiles: [' in manifest.ron!")
    sys.exit(1)
open_brackets = 0
insert_idx = -1
for i, line in enumerate(manifest.splitlines()):
    if 'profiles:' in line:
        open_brackets = line.count('[') - line.count(']')
    elif open_brackets > 0:
        open_brackets += line.count('[') - line.count(']')
        if open_brackets == 0:
            insert_idx = i
            break
if insert_idx == -1:
    print("Could not find end of profiles array in manifest.ron!")
    sys.exit(1)

lines = manifest.splitlines()
# Check if last profile before ] has a comma, add if missing
for j in range(insert_idx-1, 0, -1):
    if lines[j].strip().startswith('('):
        # Find the last line of the last profile block
        if not lines[j-1].strip().endswith(','):
            lines[j-1] = lines[j-1] + ','
        break

# 5. Build new profile blocks
profile_blocks = []
for prof in missing:
    tex_name = prof.split('/', 1)[1] + '.png'
    tex_path = f'background/{tex_name}'
    block = [
        '        (',
        f'            id: "{prof}",',
        f'            albedo_path: "{tex_path}",',
        '            normal_path: None,',
        '            roughness: 0.9,',
        '            metallic: 0.0,',
        '            uv_scale: (4.0, 3.0),',
        '            uv_offset: (0.0, 0.0),',
        '            fallback_chain: ["ground/default"],',
        '        ),'
    ]
    profile_blocks.extend(block)

# 6. Insert new blocks before the closing ] of profiles
new_lines = lines[:insert_idx] + profile_blocks + lines[insert_idx:]
with open(MANIFEST, 'w', encoding='utf-8') as f:
    f.write('\n'.join(new_lines) + '\n')
print(f"Added {len(missing)} ground profiles to manifest.ron.")
