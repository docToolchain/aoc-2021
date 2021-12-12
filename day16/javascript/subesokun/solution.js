const fs = require('fs')

// tag::star1[]
const bitMasks = [
    0x0, 0x1, 0x3, 0x7, 0xF, 0x1F, 0x3F, 0x7F, 0xFF
]

/**
 * @param {Uint8Array[]} input 
 * @param {number} startIndex 
 * @param {number} endIndex 
 * @returns {number}
 */
function getBits(input, startIndex, endIndex) {
    let result = 0
    let currentIndex = startIndex
    while (currentIndex < endIndex) {
        const byteIndex = Math.floor(currentIndex / 8)
        const bitI = currentIndex % 8
        const bitJ = (endIndex >= (byteIndex + 1) * 8) ? 8 : ((endIndex - 1) % 8 + 1)
        result = result << (bitJ - bitI)
        result |= ((input[byteIndex] >> (8 - bitJ)) & bitMasks[bitJ - bitI])
        currentIndex = (byteIndex + 1) * 8
    }
    return result
}

/**
 * @param {number[]} arr 
 * @return {number}
 */
function convert4BitArrayToNumber(arr) {
    if (arr.length % 2 === 1) {
        arr.unshift(0)
    }
    const byteArr = new Uint8Array(arr.length / 2)
    for (let i = 0; i < arr.length; i += 2) {
        byteArr[i / 2] = arr[i] << 4 | arr[i + 1]
    }
    let buffer = Buffer.from(byteArr);
    return buffer.readUIntBE(0, byteArr.length);
}

/**
 * @param {Uint8Array[]} input
 * @param {number} offset 
 * @returns {*}
 */
function processPacket(input, offset) {
    const headerOffset = offset + 3 + 3
    const pVersion = getBits(input, offset, offset + 3)
    const pType = getBits(input, offset + 3, offset + 3 + 3)
    if (pType === 4) {
        let groupIndex = -1
        let isLastGroup = false
        let literal4BitList = []
        while (!isLastGroup) {
            groupIndex++
            const groupStartIndex = headerOffset + groupIndex * 5
            const groupBits = getBits(input, groupStartIndex, groupStartIndex + 5)
            literal4BitList.push(groupBits & bitMasks[4])
            isLastGroup = groupBits >> 4 === 0
        }

        // JavaScript %$/§&$% when working with bit shift operators.
        // Shift works on signed 32-bit values but some values in the input are larger than.
        // As a result, we've suddenly minus values if we shift the value too much left.
        // Workaround: create an array of 4 bit values and convert it to a number...
        //       ¯\_(ツ)_/¯
        const literal = convert4BitArrayToNumber(literal4BitList)

        return {
            offset: headerOffset + groupIndex * 5 + 5,
            packet: {
                version: pVersion,
                type: pType,
                literal
            }
        }
    } else {
        const lengthTypeId = getBits(input, headerOffset, headerOffset + 1)
        if (lengthTypeId === 0) {
            const totalLength = getBits(input, headerOffset + 1, headerOffset + 1 + 15)
            let payloadOffset = headerOffset + 1 + 15
            let currentOffset = payloadOffset
            const packets = []
            while (currentOffset < payloadOffset + totalLength) {
                const { offset, packet } = processPacket(input, currentOffset)
                packets.push(packet)
                currentOffset = offset
            }
            return {
                offset: currentOffset,
                packet: {
                    version: pVersion,
                    type: pType,
                    packets
                }
            }
        } else {
            const numberOfSubPackets = getBits(input, headerOffset + 1, headerOffset + 1 + 11)
            let currentPackage = 1
            let payloadOffset = headerOffset + 1 + 11
            let currentOffset = payloadOffset
            const packets = []
            while (currentPackage <= numberOfSubPackets) {
                const { offset, packet } = processPacket(input, currentOffset)
                packets.push(packet)
                currentOffset = offset
                currentPackage++
            }
            return {
                offset: currentOffset,
                packet: {
                    version: pVersion,
                    type: pType,
                    packets
                }
            }
        }
    }
}

/**
 * @param {*} packets 
 * @returns {number}
 */
function sumVersions(packets) {
    let versionSum = 0
    for (const packet of packets) {
        if (packet.type === 4) {
            versionSum += packet.version
        } else {
            versionSum += packet.version + sumVersions(packet.packets)
        }
    }
    return versionSum
}

/**
 * @param {Uint8Array[]} input
 */
function runSolutionPuzzleOne(input) {
    const { packet } = processPacket(input, 0)
    const versionSum = sumVersions([packet])
    console.log(`Solution to first puzzle: ${versionSum}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {*} packet 
 * @returns {number}
 */
function evaluatePacket(packet) {
    const valueList = packet.packets
        .map(p => p.type === 4 ? p.literal : evaluatePacket(p))
    if (packet.type === 0) {
        return valueList.reduce((sum, v) => sum + v, 0)
    } else if (packet.type === 1) {
        return valueList.reduce((sum, v) => sum * v, 1)
    } else if (packet.type === 2) {
        return Math.min(...valueList)
    } else if (packet.type === 3) {
        return Math.max(...valueList)
    } else if (packet.type === 5) {
        return valueList[0] > valueList[1] ? 1 : 0
    } else if (packet.type === 6) {
        return valueList[0] < valueList[1] ? 1 : 0
    } else if (packet.type === 7) {
        return valueList[0] === valueList[1] ? 1 : 0
    }
}

/**
 * @param {Uint8Array[]} input
 */
function runSolutionPuzzleTwo(input) {
    const { packet } = processPacket(input, 0)
    const result = evaluatePacket(packet)
    console.log(`Solution to second puzzle: ${result}`)
}
// end::star2[]

// tag::input[]
let hexInputRaw = fs.readFileSync('input.txt').toString()
const input = new Uint8Array(hexInputRaw.length / 2)
for (let i = 0; i < hexInputRaw.length; i += 2) {
    input[i / 2] = parseInt(hexInputRaw[i] + hexInputRaw[i + 1], 16)
}
// end::input[]

runSolutionPuzzleOne(input)
runSolutionPuzzleTwo(input)