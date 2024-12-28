import json


def generate_webext_config(languages):
    out = []
    for language in languages:
        obj = {}
        obj["technical"] = language.technical_name
        obj["formatted"] = language.picker_name
        obj["rtl"] = language.is_rtl
        obj["dict"] = language.supports_dicts
        obj["prefer_dict"] = language.prefers_dicts
        obj["features"] = [x.to_dict() for x in language.clientside_features]

        out.append(obj)

    with open("../nuenki-extension/webext/src/languages_config_inject.js", "w") as f:
        f.write("export const languages = " + json.dumps(out, indent=4))
