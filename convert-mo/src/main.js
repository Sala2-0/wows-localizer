import * as GettextParser from 'gettext-parser';
import * as fs from 'fs';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const args = process.argv.slice(2);

const moPath = args.at(0);

if (!moPath || !fs.existsSync(moPath) || path.extname(moPath) !== ".mo") {
    console.error("Error: No .mo file path provided.");
    process.exit(1);
}

const moContent = fs.readFileSync(moPath);
const mo = GettextParser.mo.parse(moContent);

let outDir = path.join(__dirname, "out");
fs.mkdirSync(outDir);
fs.writeFileSync(path.join(outDir, "converted.po"), GettextParser.po.compile(mo));

console.log("Converted to .po file.");
process.exit(0);
