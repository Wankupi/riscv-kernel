import json
import sys

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
\t.global _userapp_{t}
_userapp_{t}:
\t.incbin "{build_dir}/{t}"
""")

with open(rs_file, "w") as f:
    f.write('extern "C" {\n')
    for t in targets:
        f.write(f"\tpub fn _userapp_{t}();\n")
    f.write('}\n')
