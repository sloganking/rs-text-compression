# rs-text-compression

## Summary

A Rust implementation of the text compression algorithm found [here](https://github.com/sloganking/text-compression). This Rust implementation's compression (single core) is around 6000x faster than it's python counterpart.


This repository contains a custom text compression algorithm, optimized for the english language. They key insight to the creation of this algorithm is that there are around 400,000 English words and the average English word is 5.1 characters long (not including the spaces next to it). as a result, we can assign a mapping of all english words to 19 bit combinations. I store these 19 bits inside 3 bytes, which left space for storing the upper/lower case of the each word, and whether they have spaces to the right or left of them. If a word or combination of characters is not in the known dictionary of words, or if encoding it would not decrease file size, it is stored as plain-text / 7 bit ascii.

This algorithm typically reduces file sized of english text by 40-50%

## Encoding

When decoding:

- Checking from the begining of the file, byte by byte. If the highest bit in a byte is a 0, that byte represents a plaintext character and can simply be transfered to the end of the decompressed txt file.

- If the highest bit in a byte is a 1, that byte and the two bytes after it (3 in total) are used to encode a compressed word. The lowest value 19 bits (bits 0-18) ``0000-0111 1111-1111 1111-1111`` store an integer that is mapped to a corresponding word via what words are in words.txt. 

- Bits 19-20, highest byte ``0001-1000``, store whether there are spaces beside the compressed word. if bit 19 ``0000-1000`` is a 1, add a space (represented here by an underscore) to the right of the decompressed word ``Word_``. If bit 20 ``0001-0000`` is a 1, add a space to the left of the decompressed word ``_Word``

- Bits 21-22 ``0110-0000`` encode the case of the word. ``x00x-xxxx`` means the word is entirly undercase. ``x01x-xxxx`` means the word is entirly upper case. ``x10x-xxxx`` means the first character of the word is upper case. ``x11x-xxxx`` is not yet defined.

## Notes

- The compression algorithm currently only works with input text files containing ASCII characters with values 127 and lower

- While decompression is straightforward. The compression algorithm must identify compressable words in order to compress them. Words are currently identified by checking if each sequnce of characters between space characters is a word. Some exceptions apply to account for punctuation marks. Improving the identification of words may result in greater compression.

- This algorithm could easilly be optimized for other languages by using a words.txt mapping for their language. So long as that language uses ASCII characters and words.txt does not exceed 524288 (2^19) words.
