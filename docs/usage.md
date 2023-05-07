# Usage

=== "JavaScript"

    ```javascript
    import fs from 'fs'
    import { convert_ilcd } from 'epdx'
    
    const ilcdString = fs.readFileSync('./ilcd.json', 'utf8')
    const epd = convertIlcd(ilcdString)
    ```

=== "Python"

    ```python 
    {!> code/python/usage.py!}
    ```
