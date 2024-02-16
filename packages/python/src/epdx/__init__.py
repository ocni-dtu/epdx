import json

from .epdx import *
from .pydantic import *

__doc__ = epdx.__doc__
if hasattr(epdx, "__all__"):
    __all__ = epdx.__all__


def convert_ilcd(data: str | dict, *, as_type: str = "dict"):
    """
    Converts a json formatted ILCD+EPD data into EPDx

    The EPDx data can either be returned as a string, a dict or a Pydantic class.
    """

    if isinstance(data, dict):
        data = json.dumps(data)

    try:
        _epd = epdx._convert_ilcd(data)
    except Exception as err:
        raise ParsingException(err)

    if as_type == "str":
        return _epd
    elif as_type == "dict":
        return json.loads(_epd)
    elif as_type == "pydantic":
        return EPD(**json.loads(_epd))
    else:
        raise NotImplemented("Currently only 'dict', 'str' and 'pydantic' is implemented as_type.")


class ParsingException(Exception):
    ...
