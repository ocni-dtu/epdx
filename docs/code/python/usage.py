from pathlib import Path
import json
import epdx.pydantic

ilcd_file = Path(__file__).parent.parent / "data" / "ilcd.json"

print("EPD as dict")
epd_dict = epdx.convert_ilcd(ilcd_file.read_text())
print(json.dumps(epd_dict, indent=2))

print("\nEPD as Pydantic model")
epd_pydantic = epdx.convert_ilcd(ilcd_file.read_text(), as_type="pydantic")
print(epd_pydantic)

print("\nEPD as string")
epd_str = epdx.convert_ilcd(ilcd_file.read_text(), as_type="str")
print(epd_str)
