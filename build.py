#!/usr/bin/env python3
from os import path as PATH, listdir
from subprocess import run as run_sp, CalledProcessError

def build(path: str) -> None:
    print(f"Building {path}")
    try: 
        run_sp(["cargo", "build"], cwd=path, check=True)
        return True
    except CalledProcessError as e:
        print(f"Failed to build {path}")
        print(e)
    return False


def main() -> None:
    print("Building main workspace")
    build(".")

    for dir_name in listdir("boards"):
        dir_path = PATH.join("boards", dir_name)
        if PATH.isdir(dir_path):
            if build(dir_path):
                print(f"Successfully built {dir_name}")
            else: 
                print(f"Failed to build {dir_name}")
                break            
        else:
            print(f"Skipping {dir_name} as it is not a directory")


if __name__ == "__main__":
    main()