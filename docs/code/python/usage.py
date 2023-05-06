from pathlib import Path
import json
import epdx.pydantic

ilcd_file = Path(__file__).parent / "ilcd.json"
epd_string = epdx.convert_ilcd(ilcd_file.read_text())
epd_dict = json.loads(epd_string)

print("EPD as dict")
print(json.dumps(epd_dict, indent=2))

epd_pydantic = epdx.pydantic.EPD(**epd_dict)

print("\nEPD as Pydantic model")
print(epd_pydantic)

