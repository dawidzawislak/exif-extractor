import os

if __name__ == "__main__":
    for file in os.listdir("./resources/images/"):
        os.system(f"cargo run ./resources/images/\"{file}\" -c -n ./resources/cleaned/\"{file}\" -o ./resources/outputs/\"{file[:-4]}\".txt")
        os.system(f"cargo run ./resources/cleaned/\"{file}\" -p")
