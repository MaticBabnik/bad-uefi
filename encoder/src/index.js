const fs = require('fs');
const { readFile } = require('fs/promises')
const Jimp = require('jimp');
const { spawn } = require('child_process');
const { Command } = require('commander');

const { genHeader, convert } = require('./uefiv');
const { getPngBuffers } = require('./pngstream');

function delay(x) {
    return new Promise(resolve => setTimeout(resolve, x));
}

async function encode(input, output, opt) {
    fs.accessSync(input, fs.constants.R_OK);
    const streamOut = fs.createWriteStream(output, { encoding: 'binary', flags: 'w' }); //create output stream, overwrite if exists

    streamOut.write(genHeader(opt.width, opt.height, opt.framerate, opt.colorMode)); //write header

    const ffmpegArgs = ['-i', input, '-vf', `scale=${opt.width}:${opt.height},fps=${opt.framerate}`, '-c:v', 'png', '-f', 'image2pipe', '-']
    console.log("ffmpeg " + ffmpegArgs.join(' '));
    const frameExtractor = spawn('ffmpeg', ffmpegArgs);

    let frameCount = 0;

    await getPngBuffers(frameExtractor.stdout, async (buffer, size) => {
        if (!buffer) { //empty buffer means end of stream
            return;
        }

        frameCount++;
        if (frameCount % 64 === 0)
            console.log(`Converting frame ${frameCount}`);

        const currentFrame = await Jimp.read(buffer);
        streamOut.write(convert[opt.colorMode](currentFrame, opt.width, opt.height));
    });

    await delay(100); // wait for async stuff to finish

    streamOut.end();
    console.log(`Processed ${frameCount} frames, kthxbye`);
}

let beatConverter = {
    'Breve': 4,
    'Double': 2,
    'Whole': 1,
    'Half': 1 / 2,
    'Quarter': 1 / 4,
    'Eighth': 1 / 8,
    'Sixteenth': 1 / 16,
}

async function audio(input, output, opt) {
    fs.accessSync(input, fs.constants.R_OK);
    const streamOut = fs.createWriteStream(output, { encoding: 'binary', flags: 'w' });

    if (opt.delay != 0) {
        const d = Buffer.alloc(4);
        d.writeUInt16BE(opt.delay - 0, 0);
        d.writeUInt16BE(0, 2);

        streamOut.write(d);
    }

    let beatLength = 60_000 / (opt.bpm-0); //ms 

    let inData = await readFile(input);

    let entries = inData.toString().split('\n').map(line => line.split(',')).map(([arg, timeUnit]) => [parseInt(arg), beatConverter[timeUnit]]);

    const bufferSize = entries.length * 4;
    let sb = Buffer.alloc(bufferSize);

    for (let offset = 0, i = 0; offset < bufferSize; offset += 4, i++) {
        let [arg, beatRatio] = entries[i];
        let time = Math.round(beatLength * beatRatio);

        if (isNaN(arg)) arg = 0;

        sb.writeUInt16BE(time, offset);
        sb.writeUInt16BE(arg, offset + 2);
    }

    streamOut.write(sb);
    streamOut.end();
}

const program = new Command();
program.command('encode')
    .description('Convert a video to UEFIV')
    .argument('<input>', 'The input video file')
    .argument('<output>', 'The output UEFIV file')
    .option('-w, --width <width>', 'The width of the output video', 40)
    .option('-h, --height <height>', 'The height of the output video', 30)
    .option('-f, --framerate <framerate>', 'The framerate of the output video', 10)
    .option('-c, --color-mode <colorMode>', 'The color mode of the output video', 'g8c')
    .action(encode);


program.command('audio')
    .description("Stupid audio format that I can't be bothered to document")
    .argument('<input>', 'The input txt file')
    .argument('<output>', 'The output UEFIA file')
    .option('-d, --delay <delay>', 'The delay before the first note (in ms)', 0)
    .option('-b, --bpm <bpm>', 'Beats per minute', 120)
    .action(audio);

program.parse();