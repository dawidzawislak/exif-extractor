import os

if __name__ == "__main__":
    for file in os.listdir("./resources/images/"):
        os.system(f"cargo run ./resources/images/\"{file}\" -c -n ./resources/cleaned/\"{file}\"")
        os.system(f"cargo run ./resources/cleaned/\"{file}\" -p")
