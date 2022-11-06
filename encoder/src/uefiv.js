const { floor, max } = Math;
const HEADER = [85, 69, 86]; // 'UEF' in ascii

const COLOR_MODES = {
    t4: 0x0,
    g8m: 0x1,
    g8c: 0x2,
    g24: 0x3,
}

function c24toMono(pix) {
    const r = (pix >> 24) & 0xFF;
    const g = (pix >> 16) & 0xFF;
    const b = (pix >>  8) & 0xFF;

    return Math.round(0.299 * r + 0.587 * g + 0.114 * b)
}

function c24to8(pix) {
    const r = (pix >> 24) & 0xE0;
    const g = (pix >> 19) & 0x1C;
    const b = (pix >> 13) & 0x03;

    return r | g | b;
}

function c24extract() {
    const r = (pix >> 24) & 0xFF;
    const g = (pix >> 16) & 0xFF;
    const b = (pix >>  8) & 0xFF;

    return [r, g, b];
}

const convert = {
    t4: (data, w, h) => {
        throw "Not implemented";
    },
    g8m: (data, w, h) => {
        const buf = Buffer.alloc(w * h);

        for (let y = 0; y < h; y++)
            for (let x = 0; x < w; x++) {
                buf[y * w + x] = c24toMono(data.getPixelColor(x, y));
            }

        return buf;
    },
    g8c: (data, w, h) => {
        const buf = Buffer.alloc(w * h);

        for (let y = 0; y < h; y++)
            for (let x = 0; x < w; x++) {
                buf[y * w + x] = c24to8(data.getPixelColor(x, y));
            }
        return buf;
    },
    g24: (data, w, h) => {
        const buf = Buffer.alloc(w * h * 3);

        for (let y = 0; y < h; y++)
            for (let x = 0; x < w; x++) {
                const [r, g, b] = c24extract(data.getPixelColor(x, y));
                const pxIndex = (y * w + x) * 3;

                buf[pxIndex] = r;
                buf[pxIndex + 1] = g;
                buf[pxIndex + 2] = b;
            }
    }
}

function inrange(v, min, max) {
    return v >= min && v <= max
}

function genHeader(w, h, framerate, colorMode) {
    // you might actually get away with bigger values
    if (!inrange(w, 16, 4096)) throw "Invalid dimensions";
    if (!inrange(h, 16, 2048)) throw "Invalid dimensions";

    if (!inrange(framerate, 1, 30)) throw "Invalid framerate";
    if (!colorMode in COLOR_MODES) throw "Invalid color mode";

    let header = Buffer.alloc(8);

    HEADER.forEach((v, i) => header[i] = v); // write file type

    header.writeUInt16BE(w, 3); //write width and height
    header.writeUInt16BE(h, 5);

    let mode = COLOR_MODES[colorMode];   //combine mode and framerate into one byte
    let fpsNibble = Math.floor(framerate / 2);
    mode |= (fpsNibble << 4) & 0xF0;

    header[7] = mode;

    return header;
}

module.exports = {
    COLOR_MODES,
    genHeader,
    convert
}