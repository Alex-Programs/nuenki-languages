import json
from language_types import Tag, TagType


def generate_frontend_config(languages):
    out = []
    for language in languages:
        obj = {}
        obj["technical"] = language.technical_name
        obj["demolist"] = language.picker_name
        obj["title"] = language.weblist_topname
        if language.weblist_subname:
            obj["subtitle"] = language.weblist_subname

        obj["flag_code"] = language.flag_code

        if language.deepl_name == None:
            language.additional_tags.append(
                Tag(type=TagType.WARNING, text="High Latency")
            )
        if language.supports_dicts:
            language.additional_tags.append(Tag(type=TagType.GOOD, text="Dictionary"))

        obj["tags"] = [x.to_dict() for x in language.additional_tags]

        out.append(obj)

    with open("../nuenki-website/frontend/src/languages_external_inject.ts", "w") as f:
        f.write("export const languages: any = " + json.dumps(out, indent=4))
