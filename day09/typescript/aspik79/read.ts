import * as fs from 'fs';

/**
 * Reads all lines from the given file.
 * @param file name of the file.
 * @param path alternative location if file is not found.
 * @returns lines of the file.
 */
export function readLines(file: string, path: string) : string[] {
    return readBlocks(file, path, '\r\n');
}

/**
 * Reads all blocks from the given file.
 * @param file name of the file.
 * @param path alternative location if file is not found.
 * @param separator separator that is to be used for separating blocks.
 * @returns blocks of the file.
 */
 export function readBlocks(file: string, path: string, separator: string) : string[] {
    let input: string;
    
    const path1 = path + '/' + file;
    
    if (fs.existsSync(path1)) {
        input = fs.readFileSync(path1,'utf8');
    }
    else {
        input = fs.readFileSync(file,'utf8');
    }

    return input.split(separator);
}