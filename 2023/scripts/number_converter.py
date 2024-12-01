import sys
from pathlib import Path

def parse_file(path: Path):
    with open(path, "r") as file:
        content = file.readlines()
    return content

def replace_values(content: list[str]):
    hash_map = { 
                "one": '1', 
                "two": '2', 
                "three": '3', 
                "four": '4', 
                "five": '5', 
                "six": '6', 
                "seven": '7', 
                "eight": '8', 
                "nine": '9',
            }    
    num_arr = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

    cleaned_list = list(map(lambda x: x.strip(), content))


    # print("forward: ")
    # for string in cleaned_list:
    #     string_buffer = ""
    #     for char in string:
    #         string_buffer += char
    #         if len(string_buffer) > 3:
    #             comparison = [num in string_buffer  for num in num_arr]
    #             if True in comparison:
    #                 print(comparison)
    #                 break
    #
    # print("\nbackward: ")
    # for string in cleaned_list:
    #     string_buffer = ""
    #     for char in string[::-1]:
    #         string_buffer += char
    #         if len(string_buffer) > 3:
    #             comparison = [num in string_buffer  for num in num_arr]
    #             if True in comparison:
    #                 print(comparison)
    #                 break

                
            


def main():
    path = Path(sys.argv[1])
    content = parse_file(path)
    result = replace_values(content)

if __name__ == "__main__":
    main()
