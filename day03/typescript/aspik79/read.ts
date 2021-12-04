import * as fs from 'fs';

export function readLines(file: string, path: string) : string[] {
    let input: string;

    const path1 = path + '/' + file;
    
    if (fs.existsSync(path1)) {
        input = fs.readFileSync(path1,'utf8');
    }
    else {
        input = fs.readFileSync(file,'utf8');
    }

    return input.split('\r\n')
}
