import json


def generate_demo_config(languages):
    out = ""
    for language in languages:
        if language.supports_hybrid_translator:
            hybrid_text = "True"
        else:
            hybrid_text = "False"
        text = (
            f'("{language.technical_name}", "{language.casual_name}", {hybrid_text}),'
        )
        out += text + "\n"

    with open("demo_input.txt", "w") as f:
        f.write(out)

    out = ""
    for language in languages:
        text = f'"{language.casual_name}",'
        out += text + "\n"

    with open("seo_input.txt", "w") as f:
        f.write(out)
