# EPDx

EPDx is a library for parsing EPD files into a common exchange format.

The library is written in Rust, to allow for cross-language package distribution.
Currently, we support Javascript/Typescript, Python and Rust.

EPDx is part of a larger project of making life cycle assessments more accessible and transparent to building
professionals.

Read more about our [overall goal](https://github.com/ocni-dtu/life-cycle-formats)

# Install Packages

=== "JavaScript"

    ```bash
    npm install epdx
    ```

=== "Python"

    ```bash
    pip install epdx
    ```

# Usage

=== "JavaScript"

    ```javascript
    import fs from 'fs'
    import { convert_ilcd } from 'epdx'
    
    const ilcdString = fs.readFileSync('./ilcd.json', 'utf8')
    const epd = convert_ilcd(ilcdString)
    ```

=== "Python"

    ```python 
    from pathlib import Path
    import json
    import epdx
    
    ilcd_file = Path(__file__) / "ilcd.json"
    epd = epdx.convert_ilcd(ilcd_file.read_text())
    ```