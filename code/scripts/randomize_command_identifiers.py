import argparse
import random
import json
import re

def randomize_config(conf_data):

    print()


    random_num_list = random.sample(
        range(1, 65535),
        len(conf_data['implant']['tasks']['commands'])
    )

    try:
        for index, command in enumerate(conf_data['implant']['tasks']['commands']):
            command['code'] = random_num_list[index]
    except Exception as e:
        print(f"[!] Failed to randomize the configuration")
        print(f"[!] Stacktrace: {e}")

    return conf_data

def main():

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input", help="Input configuration file", required=True)
    parser.add_argument("-o", "--output", help="Save the output to this file", required=False)
    parser.add_argument("-r", "--replace", help="Replace the input file (Replace In-Place)", required=False, action="store_true", default=False)

    args = parser.parse_args()
    input_file = args.input
    output_file = args.output
    replace_input_file = args.replace

    input_conf_data = None

    try:
        with open(input_file) as f:    
            input_conf_data = json.load(f)
    except Exception as e:
        print(f"[!] Error reading the configuration from the input file")
        print(f"[!] Stacktrace: {e}")
        return 1

    if input_conf_data is None:
        return 1

    if replace_input_file:
        output_file = input_file

    output_conf_data = randomize_config(input_conf_data)

    if output_file == None:
        print(output_conf_data)
        return 0
    else:
        try:
            with open(output_file, "wb") as f:
                output_conf_data_str = json.dumps(output_conf_data, indent=4)

                output_conf_data = re.sub(
                    r'^((\s*)".*?":)\s*([\[{])',
                    r'\1\n\2\3',
                    output_conf_data_str,
                    flags = re.MULTILINE
                )

                f.write(output_conf_data.encode())

            return 0

        except Exception as e:

            print(f"[!] Error saving the output to file")
            print(f"[!] Stacktrace: {e}")

            return 1


if __name__ == '__main__':

    main()
