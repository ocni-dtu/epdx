import {convertIlcd} from "epdx";
import fs from 'fs'

const ilcdData = fs.readFileSync(`${__dirname}/../data/ilcd.json`)
const epd = convertIlcd(ilcdData)

// Output the EPDx formatted epd to the console
console.log('EPDx', epd)