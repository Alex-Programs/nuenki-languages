from dataclasses import dataclass, asdict, field
from typing import Optional
from enum import Enum


class TagType(Enum):
    GOOD = "GOOD"
    WARNING = "WARNING"
    BAD = "BAD"


@dataclass
class Tag:
    type: TagType
    text: str

    def to_dict(self):
        return {
            "type": self.type.value,  # Convert Enum to its value
            "text": self.text,
        }


@dataclass
class ClientsideFeature:
    type: str
    internal: str
    display: str
    default: any
    predicated: Optional[str]

    def to_dict(self):
        return asdict(self)


@dataclass
class Language:
    technical_name: str
    database_name: str
    deepl_name: Optional[str]
    llm_name: str
    picker_name: str
    casual_name: str
    weblist_topname: str
    weblist_subname: Optional[str]
    flag_code: str
    supports_deepl: bool
    is_rtl: bool
    supports_dicts: bool
    prefers_dicts: bool
    wiktionary_code: str
    wiktionary_name: str
    additional_tags: list[Tag] = field(default_factory=list)
    clientside_features: list[ClientsideFeature] = field(default_factory=list)

    def to_dict(self):
        return {
            **asdict(self),
            "additional_tags": [tag.to_dict() for tag in self.additional_tags],
            "clientside_features": [
                feature.to_dict() for feature in self.clientside_features
            ],
        }
