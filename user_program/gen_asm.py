import json
import sys
import os

if len(sys.argv) < 4:
    print("Usage: gen_asm.py <build_dir> <asm_file> <rs_file>")
    sys.exit(1)

_, build_dir, asm_file, rs_file = sys.argv

manifest = json.load(sys.stdin)

targets: list[str] = []

for target in manifest["targets"]:
    if "bin" not in target["kind"]:
        continue
    targets.append(target["name"])

targets.sort()


with open(asm_file, "w") as f:
    for t in targets:
        f.write(f"""
\t.section .text._userapp_{t}
\t.align 2
\t.global _userapp_{t}
_userapp_{t}:
\t.incbin "{build_dir}/{t}"
\t.global _userapp_{t}_end
_userapp_{t}_end:
""")

with open(rs_file, "w") as f:
    f.write('extern "C" {\n')
    for t in targets:
        f.write(f"\tpub fn _userapp_{t}();\n")
        f.write(f"\tpub fn _userapp_{t}_end();\n")
    f.write('}\n')
    f.write('\n')
    for t in targets:
        obj = f"{build_dir}/{t}"
        sz = os.path.getsize(obj)
        f.write(f"pub const _userapp_{t}_size: usize = {sz};\n")
    f.write('\n')

    f.write(f"pub const userapps_count: usize = {len(targets)};\n")
    f.write(f"pub const userapps_name: [&str; userapps_count] = [{
        ', '.join(map(lambda x: f"\"{x}\"", targets))
    }];\n")

    f.write(f"pub const usreapps_addr: [*const u8; userapps_count] = [{
        ', '.join(map(lambda x: f"_userapp_{x} as *const u8", targets))
    }];\n")

    f.write(f"pub const userapps_size: [usize; userapps_count] = [{
        ', '.join(map(lambda x: f"_userapp_{x}_size", targets))
    }];\n")
