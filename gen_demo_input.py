import json


def generate_demo_config(languages):
    out = ""
    for language in languages:
        text = f'("{language.technical_name}", "{language.casual_name}"),'
        out += text + "\n"

    with open("demo_input.txt", "w") as f:
        f.write(out)
