# rs-text-compression

## Summary

A Rust based english text compression algorithm inspired by another [python algorithm](https://github.com/sloganking/text-compression). This Rust implementation's compression (single core) is around 6000x faster than it's python counterpart, and reduces filesize by around 60% instead of 40%.


~~This repository contains a custom text compression algorithm, optimized for the english language. They key insight to the creation of this algorithm is that there are around 400,000 English words and the average English word is 5.1 characters long (not including the spaces next to it). as a result, we can assign a mapping of all english words to 19 bit combinations. I store these 19 bits inside 3 bytes, which left space for storing the upper/lower case of the each word, and whether they have spaces to the right or left of them.If a word or combination of characters is not in the known dictionary of words, or if encoding it would not decrease file size, it is stored as plain-text / 7 bit ascii.~~

## Encoding

There are four types of encodings

### ASCII
``0XXXXXXX``

A non-compressed plaintext ASCII (<127) character

### 1 byte
``101EEEEE``

### 2 byte
``110BCEEE EEEEEEEE``

### 3 byte
``111BCEEE EEEEEEEE EEEEEEEE``

All byte encodings start with ``AXXXXXXX``. If ``A`` is 0, the byte is plaintext ASCII. If ``A`` is 1, that byte represents a compressed word and the ``B`` bytes ``ABBXXXXX`` are used to determine what type and thus the rest of the encoding.

### Key:

A - Is this a compressed word or an ASCII character? (0 = char, 1 = compressed)

B - 2 bit integer storing the byte length of the encoded word

C - what character came before this word? (0 = ' ', 1 = '\n')

D - Case of the first character in the word (0 = lower, 1 = upper)

E - Integer storing the index of the compressed word

X - A bit that can be either a 0 or a 1

## Notes

- The compression algorithm currently only works with input text files containing ASCII characters with values 127 and lower

- While decompression is straightforward. The compression algorithm must identify compressable words in order to compress them. Words are currently identified by checking if each sequnce of characters between space characters is a word. Some exceptions apply to account for punctuation marks. Improving the identification of words may result in greater compression.

- This algorithm could easilly be optimized for other languages by using a words.txt mapping for their language. So long as that language uses ASCII characters and words.txt does not exceed 524288 (2^19) words.
