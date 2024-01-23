import pytest

import epdx


def test_parse_ilcd(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = epdx.convert_ilcd(ilcd_file.read_text())

    assert isinstance(epd, dict)


def test_parse_empty():
    with pytest.raises(epdx.ParsingException):
        epdx.convert_ilcd("{}")


def test_parse_ilcd_str(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = epdx.convert_ilcd(ilcd_file.read_text(), as_type='str')

    assert isinstance(epd, str)


def test_parse_ilcd_pydantic(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = epdx.convert_ilcd(ilcd_file.read_text(), as_type='pydantic')

    assert isinstance(epd, epdx.EPD)
