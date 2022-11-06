# **U**n**EF**f**I**cient **V**ideo coding (.uefiv)

A very simple video format...

## Header
| 0 - 23  (3B)  	| 24 - 39 (2B)  | 40 - 55 (2B)  | 56 - 63 (1B)	|
| :---------------	| :----------:	| :-----------: |-------------:	|
| `UEV` (ascii) 	| width (u16) 	| height (u16)  | display mode 	|

### Display mode
Display mode is made up of 2 nibbles, the first(most significant) nibble selects the framerate, the second(least significant) selects the color mode.

### Framerate
Calculated as: `(framerate / 2) << 4`.
Note: `0` will be clamped to 1fps;

### Color mode
| color mode    | name  | description               | Bits per pixel    |
| :----         | :---- | :------------------------ | :---------------- |
| 0x0           | t4    | 16 colors (text mode)     | 8 (im lazy)       |
| 0x1           | g8m   | 256 colors (monochrome)   | 8                 |
| 0x2           | g8c   | 256 colors (`RRRGGGBB`)   | 8                 |
| 0x3           | g24   | 24 bit color              | 24                |


## Body
Contains the frame data. The size of a frame is `width * height * (color mode size)`.
A frame contains rows of pixel (from the top to bottom), where the leftmost pixel is the first pixel in the row.
```
(0,0)

 |
 V

 1   2   3   4   5  
 6   7   8   9   10
 11  12  13  14  15  <- (4,2)
```