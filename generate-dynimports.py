#!/usr/bin/python
import sys
import os
from time import sleep
import tomli
from dataclasses import dataclass

CURRENT_EXEC_DIR = os.path.dirname(sys.argv[0])
CARGO_RELEASE_TOML = "Cargo.release.toml"

CRATES_ROOT = f"{CURRENT_EXEC_DIR}/crates"
CONFIG_PATH = f"{CURRENT_EXEC_DIR}/{CARGO_RELEASE_TOML}"

cargo_toml_template = """
[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
dynimports = {{ path = "../dynimports" }}
"""

lib_rs_template = """
pub use dynimports::{name}::*;
"""


class PackageInfo:
    name: str
    crate_path: str
    crate_cargo_toml: str
    crate_src: str
    lib_rs: str

    def __init__(self, name):
        self.name = name
        self.crate_path = f"{CRATES_ROOT}/{name}"
        self.crate_cargo_toml = f"{self.crate_path}/Cargo.toml"
        self.crate_src = f"{self.crate_path}/src"
        self.lib_rs = f"{self.crate_src}/lib.rs"

    def __repr__(self):
        return f"""Info of package: {self.name}
    crate_path={self.crate_path}
    crate_cargo_toml={self.crate_cargo_toml}
    crate_src={self.crate_src}
    lib_rs={self.lib_rs}"""


def gen_content(name):
    cargo_toml = cargo_toml_template.format(name=name)
    lib_rs = lib_rs_template.format(name=name.replace("-", "_"))
    return (cargo_toml, lib_rs)


def gen_package(name):
    info = PackageInfo(name)
    (cargo_toml, lib_rs) = gen_content(name)

    try:
        os.mkdir(info.crate_path)
        print(f"    info: created: {info.crate_path}")

        os.mkdir(info.crate_src)
        print(f"    info: created: {info.crate_src}")

        with open(info.crate_cargo_toml, "w") as f:
            f.write(cargo_toml)
        print(f"    info: created: {info.crate_cargo_toml}")

        with open(info.lib_rs, "w") as f:
            f.write(lib_rs)
        print(f"    info: created: {info.lib_rs}")
    except Exception as e:
        print(f"error: can't create file for package {name}. Error: {e}")


def gen_file_from_list(package_list):
    print(f"info: {len(package_list)} packages")

    for name in package_list:
        print(f"info: creating package: {name}")
        gen_package(name)


def gen_imports(package_list):
    for name in package_list:
        print(f"""{name} = {{ path = "./crates/{name}" }}""")
    pass


def main():
    print(f"info: read dependecies from: {CONFIG_PATH}")

    data = ""
    try:
        with open(CONFIG_PATH, "rb") as manifest_file:
            data = tomli.load(manifest_file)
    except Exception as e:
        print(f"error: {e}")
        exit(1)

    package_list = list(data["dependencies"].keys())

    gen_file_from_list(package_list)
    print("info: done")
    print("info: generating dependencies")

    gen_imports(package_list)


if __name__ == "__main__":
    main()
