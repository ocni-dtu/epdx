# Custom EPDx Parser in Python  

EPDx is an exchange format for Environmental Product Declarations (EPDs). It is based on JSON and is designed to be easy
to read and write by humans and machines alike. EPDx is an open standard that can be used by anyone without restriction.

To be an exchange format you need to be able to convert from other formats to EPDx. This is a writeup of how to
create a custom parser for converting a CSV file into EPDx.

To do that we will overwrite the `from_dict` class method of the `epdx.pydantic.EPD` class.
This method is used to convert a dictionary into a class instance of `epdx.pydantic.EPD`.

Basically what we need to do is:

```python
from epdx.pydantic import EPD

class EPDx(EPD):

    @classmethod
    def from_dict(cls, table7_object: dict):
        """Convert a row from the table 7 csv to an EPDx object"""

        return cls(
            ...
        )
```

## Convert CSV to Dict

However, the first thing we need to do is to convert the data from the CSV file into a dictionary.
We can use the `csv` module for that:

```python
def main(path: Path, out_path: Path):
    reader = csv.DictReader(io.StringIO(path.read_text()))

    for row in reader:
        parse_row(row, out_path)

if __name__ == "__main__":
    p = Path("tabel7.csv")
    out = Path(__file__).parent.parent / "table7"
    main(p, out)
```

## CSV to EPDx

Next thing we need to do is to convert the data from the CSV file into an EPDx object and write it back to disk as a
json file.

```python
def parse_row(row: dict, out_path: Path):
    if row.get("Sorterings ID").startswith("#S"):
        return
    epd = EPDx.from_dict(row)

    (out_path / f"{epd.id}.json").write_text(epd.json(ensure_ascii=False, indent=2))
```

## EPDx Class

Next we just need to populate the `from_dict` method and "map" the right fields from the CSV to the fields in the EPDx
class.

```python
class EPDx(EPD):

    @classmethod
    def from_dict(cls, table7_object: dict):
        """Convert a row from the table 7 csv to an EPDx object"""

        declared_factor = float(table7_object.get("Deklareret faktor (FU)"))
        declared_unit = table7_object.get("Deklareret enhed (FU)")
        table7_id = table7_object.get("Sorterings ID")

        return cls(
            id=cls.convert_lcabyg_id(table7_id),
            format_version=importlib.metadata.version("epdx"),
            name=table7_object.get("Navn DK"),
            version="version 2 - 201222",
            declared_unit=cls.convert_unit(declared_unit),
            valid_until=datetime(year=2025, month=12, day=22),
            published_date=datetime(year=2020, month=12, day=22),
            source="BR18 - Tabel 7",
            standard=Standard.EN15804A1,
            subtype=cls.convert_subtype(table7_object.get("Data type")),
            comment=table7_id,
            reference_service_life=None,
            location="DK",
            conversions=[
                {"to": Unit.KG,
                 "value": float(table7_object.get("Masse faktor")) * declared_factor}
            ],
            gwp={
                "a1a3": cls.convert_gwp(
                    table7_object.get("Global Opvarmning, modul A1-A3"),
                    declared_factor
                ),
                "a4": None,
                "a5": None,
                "b1": None,
                "b2": None,
                "b3": None,
                "b4": None,
                "b5": None,
                "b6": None,
                "b7": None,
                "c1": None,
                "c2": None,
                "c3": cls.convert_gwp(table7_object.get("Global Opvarmning, modul C3"), declared_factor),
                "c4": cls.convert_gwp(table7_object.get("Global Opvarmning, modul C4"), declared_factor),
                "d": cls.convert_gwp(table7_object.get("Global Opvarmning, modul D"), declared_factor),
            },
            meta_fields={"data_source": table7_object.get("Url (link)")},
        )
```

## Complete Code

The full file can be seen below and is also available in the [Table7 repository](https://github.com/ocni-dtu/table7).

```python
import csv
import io
import json
import uuid
from datetime import datetime
from pathlib import Path
import importlib.metadata
from epdx.pydantic import EPD, Standard, SubType, Unit


class EPDx(EPD):

    @classmethod
    def from_dict(cls, table7_object: dict):
        """Convert a row from the table 7 csv to an EPDx object"""

        declared_factor = float(table7_object.get("Deklareret faktor (FU)"))
        declared_unit = table7_object.get("Deklareret enhed (FU)")
        table7_id = table7_object.get("Sorterings ID")

        epd = cls(
            id=cls.convert_lcabyg_id(table7_id),
            format_version=importlib.metadata.version("epdx"),
            name=table7_object.get("Navn DK"),
            version="version 2 - 201222",
            declared_unit=cls.convert_unit(declared_unit),
            valid_until=datetime(year=2025, month=12, day=22),
            published_date=datetime(year=2020, month=12, day=22),
            source="BR18 - Tabel 7",
            standard=Standard.EN15804A1,
            subtype=cls.convert_subtype(table7_object.get("Data type")),
            comment=table7_id,
            reference_service_life=None,
            location="DK",
            conversions=[
                {"to": Unit.KG,
                 "value": float(table7_object.get("Masse faktor")) * declared_factor}
            ],
            gwp={
                "a1a3": cls.convert_gwp(
                    table7_object.get("Global Opvarmning, modul A1-A3"),
                    declared_factor
                ),
                "a4": None,
                "a5": None,
                "b1": None,
                "b2": None,
                "b3": None,
                "b4": None,
                "b5": None,
                "b6": None,
                "b7": None,
                "c1": None,
                "c2": None,
                "c3": cls.convert_gwp(table7_object.get("Global Opvarmning, modul C3"), declared_factor),
                "c4": cls.convert_gwp(table7_object.get("Global Opvarmning, modul C4"), declared_factor),
                "d": cls.convert_gwp(table7_object.get("Global Opvarmning, modul D"), declared_factor),
            },
            meta_fields={"data_source": table7_object.get("Url (link)")},
        )
        return epd

    @staticmethod
    def convert_lcabyg_id(bpst_id: str) -> str:
        _map = json.loads(Path("lcabyg_tabel7_map.json").read_text())
        return _map.get(bpst_id, str(uuid.uuid4()))

    @staticmethod
    def convert_unit(unit: str) -> Unit:
        match unit:
            case "STK":
                return Unit.PCS
            case "M":
                return Unit.M
            case "M2":
                return Unit.M2
            case "M3":
                return Unit.M3
            case "KG":
                return Unit.KG
            case "L":
                return Unit.L
            case _:
                return Unit.UNKNOWN

    @staticmethod
    def convert_subtype(subtype: str) -> SubType:
        _map = {
            "Generisk data": SubType.Generic,
            "Branche data": SubType.Industry,
        }
        return _map.get(subtype)

    @staticmethod
    def convert_gwp(gwp: str, declared_factor: float) -> float | None:
        if gwp == "-":
            return None
        else:
            return float(gwp) / declared_factor


def main(path: Path, out_path: Path):
    reader = csv.DictReader(io.StringIO(path.read_text()))

    for row in reader:
        parse_row(row, out_path)


def parse_row(row: dict, out_path: Path):
    if row.get("Sorterings ID").startswith("#S"):
        return
    epd = EPDx.from_dict(row)

    (out_path / f"{epd.id}.json").write_text(epd.json(ensure_ascii=False, indent=2))


if __name__ == "__main__":
    p = Path("tabel7.csv")
    out = Path(__file__).parent.parent / "table7"
    main(p, out)
```