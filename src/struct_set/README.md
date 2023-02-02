# Object
This struct express version management element. The following four major objects are used to manage version.
- Blob   ... file based object
- Tree   ... directory based obeject
- Commit ... repository based object
- Tags   ... specific repository based object

## What is object
The reality of the object is the file itself using the os file system. Then, an object consists of the following two parts.
- file name    ... Hash value based on a content
- Content      ... A content compressed with zlib

## How to create object
The steps to create an object are as follows.
1. Create content to be stored
2. Calculate hash value
3. Compress content with zlib
4. Save as file

**Official Documentation**
[Git Object](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects)

## Blob
Blob is a smallest object in git. 

It corresponds to a **file** in the filesystem.

<img src="https://github.com/nopeNoshishi/nss/blob/main/picture/blob.png" width="600">

## Tree
Tree is a largest object in git.

It corresponds to a **directory** in the filesystem.

<img src="https://github.com/nopeNoshishi/nss/blob/main/picture/tree.png" width="600">

## Commit
Commit is a tree-targeted object in git.

<img src="https://github.com/nopeNoshishi/nss/blob/main/picture/commit.png" width="600">

# Meta data

## Index
Index is a kind of prelude to formally registering the object in the repository. In reality, the Index is just a file that manages meta-information about objects. But the way it is managed is a bit complicated and needs to be parsed well.

**index structure**

Official doumentation is [Git reference index format](https://git-scm.com/docs/index-format)

header
- 4bytes ... index header     [*To display to encode by utf8*]
- 4bytes ... index version    [*To display to encode by utf8*]
- 32 bits ... entry num       [u8 x 4 -> u32]

entry
- 32 bits ... ctime           [u8 x 4 -> u32]
- 32 bits ... ctime nsec      [u8 x 4 -> u32]
- 32 bits ... mtime           [u8 x 4 -> u32]
- 32 bits ... ctime nsec      [u8 x 4 -> u32]
- 32 bits ... device id       [u8 x 4 -> u32]
- 32 bits ... inode           [u8 x 4 -> u32]
- 32 bits ... mode            [u8 x 4 -> u32]
- 32 bits ... use id          [u8 x 4 -> u32]
- 32 bits ... group id        [u8 x 4 -> u32]
- 32 bits ... file size       [u8 x 4 -> u32]
- 160 bits ... blob hash      [*To display to encode by hex*]
- 16 bits ... filename size   [u8 x 2 -> u16]
- ?? bits* ... filename       [*To display to encode by utf8*]

(*)1-8 nul bytes as necessary to pad the entry to a multiple of eight bytes while keeping the name NUL-terminated. This menas Entry bytes (62 bytes + ? (filename) bytes + padding bytes) to be a multiple of 8.
