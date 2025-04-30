import toml
import json
from gen_webext import generate_webext_config
from gen_frontend import generate_frontend_config
from language_types import (
    Language,
    TagType,
    Tag,
    ClientsideFeature,
    ClientsideFeatureSelectOption,
)
from gen_demo_input import generate_demo_config

# Load TOML and parse into objects
with open("languages.toml") as f:
    text = f.read()
    data = toml.loads(text)

languages = []

data = sorted(data.items(), key=lambda x: x[1]["picker_name"])

for key, language_data in data:
    additional_tags = [
        Tag(type=TagType(tag["category"]), text=tag["text"])
        for tag in language_data.get("additional_tags", [])
    ]

    clientside_features = [
        ClientsideFeature(
            type=feature["type"],
            internal=feature["internal"],
            display=feature["display"],
            default=feature["default"],
            predicated=feature.get("predicated"),
            options=[
                ClientsideFeatureSelectOption(op["internal"], op["display"])
                for op in feature.get("options", [])
            ],
        )
        for feature in language_data.get("clientside_features", [])
    ]

    language = Language(
        technical_name=key,
        database_name=language_data["database_name"],
        deepl_name=language_data.get("deepl_name"),
        llm_name=language_data["llm_name"],
        picker_name=language_data["picker_name"],
        casual_name=language_data["casual_name"],
        weblist_topname=language_data["weblist_topname"],
        weblist_subname=language_data.get("weblist_subname"),
        flag_code=language_data["flag_code"],
        supports_deepl=language_data["supports_deepl"],
        is_rtl=language_data["is_rtl"],
        supports_dicts=language_data["supports_dicts"],
        prefers_dicts=language_data["prefers_dicts"],
        additional_tags=additional_tags,
        clientside_features=clientside_features,
        wiktionary_code=language_data["wiktionary_code"],
        wiktionary_name=language_data["wiktionary_name"],
        native_name=language_data["native_name"],
        is_groq_acceptable=language_data["is_groq_acceptable"],
    )
    languages.append(language)

# Convert to JSON-serializable dictionaries
languages_json_ready = [language.to_dict() for language in languages]

# Serialize to JSON
print(json.dumps(languages_json_ready, indent=2))

# Pass serialized data to `generate_webext_config`
generate_webext_config(languages)
generate_frontend_config(languages)
generate_demo_config(languages)
