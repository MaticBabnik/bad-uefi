const { Buffer } = require('buffer');
const PNG_HEADER = Buffer.from([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
const BUFFER_SIZE = 2 * 2 ** 20; // 2MiB

/**
 * Takes in a stream and calls a callback with the PNG buffers found.
 * @param {ReadableStream} stream The stream to read from.
 * @param {(buffer?:Buffer,size?:Number)=>void} callback Recieves the PNG buffers. if the buffer is null, the stream has ended.
 * @returns {Promise<void>} A promise that resolves when the stream closes.
 */
function getPngBuffers(stream, callback) {
    return new Promise((resolve) => {
        let currentImage = Buffer.alloc(BUFFER_SIZE);

        let chunkHeader = Buffer.alloc(8); //contains the size+type for the next chunk

        let cur = {
            inPng: false,
            bytesRead: 0, // bytes read for the current image
            nextChunkPosition: 8,
            iend: false // when this is true nextChunkPosition means EOF
        }; //holds current status

        stream.on('data', /*async*/ (buffer) => {
            for (let i = 0; i < buffer.length; i++) {
                // find the next valid PNG header
                if (!cur.inPng) {
                    if (buffer[i] == PNG_HEADER[cur.bytesRead]) {
                        cur.bytesRead++;
                        if (cur.bytesRead == 8) {
                            cur.inPng = true;
                            PNG_HEADER.copy(currentImage, 0);
                        }

                    } else { //invalid header, reset
                        cur.bytesRead = 0;
                        cur = {
                            inPng: false,
                            bytesRead: 0,
                            nextChunkPosition: 8,
                            iend: false
                        }
                    }
                } else { //in png

                    currentImage[cur.bytesRead] = buffer[i]; //copy byte to buffer
                    if (cur.iend && cur.bytesRead === cur.nextChunkPosition - 1) { //we have read the last byte; reset and call callback

                        const cbBuffer = Buffer.alloc(cur.bytesRead + 1);
                        currentImage.copy(cbBuffer, 0, 0, cur.bytesRead + 1);

                        /*await*/ callback(cbBuffer, cur.bytesRead);

                        currentImage.fill(0);
                        cur = {
                            inPng: false,
                            bytesRead: 0,
                            nextChunkPosition: 8,
                            iend: false
                        }
                        i--; //wtf?
                        continue;
                    } else { //reading between the PNG header and IEND

                        if (cur.bytesRead >= cur.nextChunkPosition && cur.bytesRead - 8 < cur.nextChunkPosition) { //inside the chunk header
                            const chunkIndex = cur.bytesRead - cur.nextChunkPosition;
                            chunkHeader[chunkIndex] = buffer[i];

                            if (chunkIndex === 7) { // finished reading the chunk header
                                let size = chunkHeader.readUInt32BE(0);
                                let type = chunkHeader.subarray(4, 8).toString("ascii");

                                cur.nextChunkPosition += size + 12;

                                cur.iend = type === "IEND";
                            }
                        }

                    }
                    cur.bytesRead++;
                }
            }
        })

        stream.on('close', () => {
            callback(null, null);

            delete currentImage;
            delete chunkHeader; //probably not necessary, can't hurt
            resolve();
        })
    });
}

module.exports = {
    getPngBuffers
}