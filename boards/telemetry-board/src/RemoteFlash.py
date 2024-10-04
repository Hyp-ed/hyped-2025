#Location of RemoteFlash.py will change however it remains here for testing

import os
import sys 
from random import randint

def inf(s:str) -> str: "\x1b[34;49m[I] " + s + "\x1b"
def que(s:str) -> str: "\x1b[35;49m[Q] " + s + "\x1b"
def err(s:str) -> str: "\x1b[31;49m[E] " + s + "\x1b" 

def main(*args) -> None:
    cargo_path:str = ""
    if args : cargo_path = args[0]
    else :    cargo_path = os.abspath(input(que("Please enter the path to the folder that contains the target Cargo.toml: ")))
    
    try:
        assert os.path.isdir(cargo_path)
        os.chdir(cargo_path)
    except Exception :
        print(err("Unable to open target dir"))
        exit(1)
    
    
    target_toml = os.path.join(cargo_path, "cargo.toml")
    
    if not os.path.isfile(target_toml):
        print(err(f"Couldnt find {target_toml}"))
        exit(2)
        
    # use of randint to try avoid naming conflicts
    build_path =  os.path.join(cargo_path, "remote_flashing_tmp_dir_" + str(randint(100_000, 999_999)))
    # python please add do while :(
    while os.path.isdir(build_path): build_path =  os.path.join(cargo_path, "remote_flashing_tmp_dir_" + str(randint(100_000, 999_999)))
    
    try :
        os.mkdir(build_path)
    except Exception: 
        print(err("Couldn't create temp build dir "))
    print(inf(f"Created tmp build dir {build_path}"))
    print(inf("Running cargo build..."), end=" ")
    
    # todo 
    # 1. cargo build into tmp path
    # 2. parse the toml to find the bin name, (dont try too hard)
    #   i. if it cannot be found promp for its name or crash
    #   ii. if name promp fails ask user to find the bin
    # 3. generate an md5 checksum 
    # 4. save b64 and md5 of the flash to JSON for debuging purpouses
    # 5. THE HARD BIT FLASH THE BOARD   

    
    
if __name__ == "__main__":
    main(sys.argv[1:], sys.kw)