import epdx


def test_parse_ilcd(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = epdx.convert_ilcd(ilcd_file.read_text())

    assert isinstance(epd, dict)
