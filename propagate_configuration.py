import toml
from dataclasses import dataclass
from typing import Optional
from enum import Enum
from generate_webext_config import generate_webext_config


class TagCategory(Enum):
    GOOD = "GOOD"
    WARNING = "WARNING"
    BAD = "BAD"


@dataclass
class Tag:
    category: TagCategory
    text: str


@dataclass
class ClientsideFeature:
    type: str
    internal: str
    display: str
    default: any
    predicated: Optional[str]


@dataclass
class Language:
    technical_name: str
    database_name: str
    deepl_name: Optional[str]
    llm_name: str
    picker_name: str  # emojis etc, used in the picker
    casual_name: str  # used in the demo's text
    weblist_topname: str
    weblist_subname: Optional[str]
    flag_code: str
    supports_deepl: bool
    is_rtl: bool
    supports_dicts: bool
    prefers_dicts: bool
    additional_tags: list[Tag]
    clientside_features: list[ClientsideFeature]


with open("languages.toml") as f:
    text = f.read()
    data = toml.loads(text)

    languages = []

for key, language_data in data.items():
    additional_tags = [
        Tag(category=TagCategory(tag["category"]), text=tag["text"])
        for tag in language_data.get("additional_tags", [])
    ]

    clientside_features = [
        ClientsideFeature(
            type=feature["type"],
            internal=feature["internal"],
            display=feature["display"],
            default=feature["default"],
            predicated=feature.get("predicated"),
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
    )
    languages.append(language)

print(languages)

generate_webext_config(languages)
