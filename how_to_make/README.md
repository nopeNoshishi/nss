# Hello Dev community! 
I'm noshishi, a apprentice engineer in Tokyo.
This article is about understanding Git from the inside by creating a simple program that `add` and `commit`.

**But it's a very long story, so I'll post the development section separately!**

# Foreword
The starting point is 'If I could understand git, I could make it?!!' 

I took this opportunity to try out a new programming language, so I decided to try Rust this time. The repository I actually created is [My original git nss](https://github.com/nopeNoshishi/nss). The quality of the code isn't quite there yet and many parts are still incomplete, but you can do a straight line of local development!

If you give me a **star**, I'll be happy to fly, and of course I'll be waiting for your contributions! Feel free to touch this repository any way you like!

**Please forgive us for not being able to explain some of the details in this article alone. Also, we use `Rust` for development, but `Python` for the stage where we uncover Git's internals!**

# TOC
<!-- TOC -->

- [Git Inside?](#git-inside)
	- [Where is repository](#where-is-repository)
	- [Object](#object)
	- [Index](#index)
- [Analyze Object](#analyze-object)
	- [blob](#blob)
	- [tree](#tree)
	- [commit](#tree)
	- [Summary](#summary1)
- [Analyze Index](#analyze-index)
	- [Specification](#specification)
	- [Index](#index)
	- [Summary](#summary2)
- [Background of Command](#background-of-command)
	- [add](#add)
	- [commit](#commit)
- [Digression](#digression)
	- [Deciphering Tree](#deciphering-tree)
	- [HEAD and Branch](#head-and-branch)  
	- [Plumbing commands](#plumbing-commands)  
- [Finally](#finally)
- [What you need](#what-you-need)


<!-- TOC -->


# Git Inside
First, we will unpack how Git handles data, based on the official documentation.
The Git command system is very complex.
But, Git data structure is very simple!

## Where is repository
A repository is the directory under the control of Git, and the folder `.git` in the directory created by `init` or `clone` is the actual state of the repository.

Let's put an empty folder called `project` under Git's control.
```bash
$ pwd
/home/noshishi/project
$ ls -a
# nothing yet
$ git init
Initialized empty Git repository in /home/noshishi/project/.git/
$ ls -a
.git
```


This `.git` directory consists of the following.

```text
.git
├── HEAD
├── (index)  // Not created by `init`!
├── config
*
├── objects/
└── refs/
    ├── heads/
    └── tags/
```

**(info)**

The path types of Git repositories are difficult to understand at first glance. We have added `/` to the directory path so that you can refer to it. Also, we have omitted parts that are not explained in this article.

## Object
Git manage versions by **file data** called **objects**.  
Objects are stored in `.git/objects`.


### Types
Objects has four types, `blob`、`tree`、`commit`、`tag`.

The contents of each and the corresponding data will be as follows.
- `blob` ... File data
- `tree` ... Directory data
- `commit` ... Metadata to manage the `tree` of the repository
- `tag` ... Metadata for a specific `commit` * Not explained at this article.

Image with `first.txt` in the `project` repository
<img width="400" alt="Object=png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/4f9a06f6-d69b-97f0-c01f-74277e1590a6.png">


### Structure
The Object is **FILE DATA**, so it has a `file name (path)` and the `data` stored in it, just like a normal file.

**File name (path)**
The file name (path) is **40-character string**. This is a hash (`sha-1`[2](#sha1)) of object **data**.

Actually, the first two are the directory path and the remaining 38 are the file path.


**Data**
Object data is compressed by `zlib` [1](#zlib). The decompressed data consists of two parts: `header` and `content`. The two elements are then separated by `\0` (null byte).


`header` is a combination of the object type and the size of `content`.

`content` contains the corresponding data in an easy-to-handle format, as indicated by the **type**. (Later we will see the details).


How to Create `blob` Object
<p align="center">
<img src="https://github.com/nopeNoshishi/nss/blob/main/picture/howtoblob.png" width="500">
</p>

## Index (staging area)
The actual index used when you `add` is a file `.git/index`.

### Structure
The index stores data of files marked by `add` with meta information. The stored data contains the latest file data at the time of `add`.

It is important to note that **all data recorded in the index is in file data units**.
I will describe meta information in detail later, but the storage format is exactly defined as shown in [index-format](https://github.com/git/git/blob/v2.12.0/Documentation/technical/index-format.txt).

> Hmm.... feel sleepy....

Wait!

Let's actually analyze the object and the index!

# Analyze Object
Before starting the analysis work, create all of the `blob`, `tree`, and `commit`.
Just add the files in `project` and commit.


Createing the following two files...


`first.txt`
```
Hello World!
This is first.txt.
```
`second.py`
```python
def second():
    print("This is second.py")
```

next, `add` and `commit`.
```bash
git add -A
git commit -m 'initial'
```

Then the contents of `.git/objects` are now as follows.
```text
.git/
└── objects/
    ├── 48/
    |   └── c972ae2bb5652ada48573daf6d27c74db5a13f
    ├── af/
    |   └── 22102d62f1c8e6df5217b4cba99907580b51af
    ├── da/
    |   └── f3f26f3fa03da346999c3e02d5268cb9abc5c5
    └── f7/
        └── f18b17881d80bb87f281c2881f9a4663cfcf84
```

***From now on, hash values in the text will omit the number of characters. [3](#hash-number)**


<br>
<br>

The corresponding data and hash values for each are summarized below.

|hash value |Object |correspond data  |
|---|---|---|
|`f7f18b1`|`blob`|first.txt|
|`af22102`|`blob`|second.py|
|`daf3f26`|`tree`|project direcrtory|
|`48c972a`|`commit`|commit version 1|

*The analysis work will be conducted interactively using Python, an interpreted language.

## blob
`blob` is an object corresponding to file data.
The image looks like this.
![Blob.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3f8cbf30-2d00-eb36-d25f-73cbc96ccc34.png)


### Data
First, let's look at `f7f18b1`, which corresponds to `first.txt`.

...Oops, I failed.
```bash
% python
>>> with open('.git/objects/f7/f18b17881d80bb87f281c2881f9a4663cfcf84', 'r') as f:
...     contnet = f.read()
UnicodeDecodeError: 'utf-8' codec can't decode byte 0xca in position 3: invalid continuation byte
```

Since the content is compressed, attempting to read the content as-is as a string [4](#compressed-string) will fail.
Therefore, we read the content as binary.

```bash
>>> with open('.git/objects/f7/f18b17881d80bb87f281c2881f9a4663cfcf84', 'rb') as f: # read binary!
...     contnet = f.read()
>>> content
b'x\x01K\xca\xc9OR06d\xf0H\xcd\xc9\xc9W\x08\xcf/\xcaIQ\xe4\n\xc9\xc8,V\x00\xa2\xb4\xcc\xa2\xe2\x12\xbd\x92\x8a\x12=\x00\xfa-\r\x03'
```

Then I read successfully and the byte string.

Now, decompress the content with `zlib`, as described in the official documentation.

```bash
>>> import zlib
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'blob 31\x00Hello World!\nThis is first.txt.'
>>> decompressed.split(b'\0')
[b'blob 31', b'Hello World!\nThis is first.txt.']
```
We found that a `blob` consists of the following elements

- `header` ... `blob 31`
- `Null byte` ... `\x00`　※hex notation
- `content` ... `Hello World!\nThis is first.txt.`

### File name
We should check whether the hash value of the object is indeed correct.

The file name of the object should be the value obtained by hashing `decompressed` with the hash function `sha1`, so check it.

```bash
>>> import hashlib
>>> blob = b'blob 31\x00Hello World!\nThis is first.txt.'
>>> sha1 = hashlib.sha1(blob).hexdigest()
>>> sha1
'f7f18b17881d80bb87f281c2881f9a4663cfcf84'
```
Great, exact match!

<br>
<br>

### How about another file
Let's also look at `af22102`, which corresponds to the other `second.py`.
```bash
>>> with open('.git/objects/af/22102d62f1c8e6df5217b4cba99907580b51af', 'rb') as f:
...     contnet = f.read()
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'blob 44\x00def second():\n    print("This is second.py")'

>>> blob = b'blob 44\x00def second():\n    print("This is second.py")'
>>> sha1 = hashlib.sha1(test).hexdigest()
>>> sha1
'af22102d62f1c8e6df5217b4cba99907580b51af'
```
It can be summarized as follows
- `header` ... `blob 44`
- `Null byte` ... `\x00`
- `content` ... `def second():\n    print("This is second.py")`

And the `sha1` values (hash values) derived from the data also matched.


**Supplemental**
The `blob` itself does not hold the filename of the corresponding file data.

Instead of `blob`, the object that manages its name is `tree`.


## Tree
`tree` is an object corresponding to directory data.
The image looks like this.
![Tree.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/ad49d01c-b791-d0e3-2a2f-5ae6b43e0d78.png)


We will analyze it in the same way as for `blob`.
```bash
>>> with open('.git/objects/da/f3f26f3fa03da346999c3e02d5268cb9abc5c5', 'rb') as f:
...     content = f.read()
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'tree 74\x00100644 first.txt\x00\xf7\xf1\x8b\x17\x88\x1d\x80\xbb\x87\xf2\x81\xc2\x88\x1f\x9aFc\xcf\xcf\x84100644 second.py\x00\xaf"\x10-b\xf1\xc8\xe6\xdfR\x17\xb4\xcb\xa9\x99\x07X\x0bQ\xaf'
>>> decompressed.split(b'\0')
[b'tree 74',
 b'100644 first.txt',
 b'\xf7\xf1\x8b\x17\x88\x1d\x80\xbb\x87\xf2\x81\xc2\x88\x1f\x9aFc\xcf\xcf\x84100644 second.py',
 b'\xaf"\x10-b\xf1\xc8\xe6\xdfR\x17\xb4\xcb\xa9\x99\x07X\x0bQ\xaf']
```

The `tree` has multiple contents, so we seem a bit complicated.

The `tree` contnet is composed of repeating `mode`[5](#mode), `path` and `hash`, which are meta information about the data in the directory,

If you simply separate them with `\0`, the hash value of the previous data and the meta information of the next file data are attached to each other.

This is because the meta information and the hash value are separated by `\0`.


First, we will check the data stored in the first one.
Looking at the split, like `first.txt` is stored, right?

```bash
>>> temp = decompressed.split(b'\0')
>>> temp[1]
b'100644 first.txt'
>>> temp[2]
b'\xf7\xf1\x8b\x17\x88\x1d\x80\xbb\x87\xf2\x81\xc2\x88\x1f\x9aFc\xcf\xcf\x84100644 second.py'
```

In order to split `temp[2]` well, let's take it out by 20 bytes.
Array access of byte strings can be byte.

```bash
>>> temp[2][0:20]
b'\xf7\xf1\x8b\x17\x88\x1d\x80\xbb\x87\xf2\x81\xc2\x88\x1f\x9aFc\xcf\xcf\x84'
>>> temp[2][0:20].hex()
'f7f18b17881d80bb87f281c2881f9a4663cfcf84'
>>> temp[2][20:]
b'100644 second.py'
```

Repeating the same process revealed the following.
- `header` ... `tree 74`
- `Null byte` ... `\x00`
- `content1` ... `100644 first.txt\x00f7f18b1...`
- `content2` ... `100644 second.py\x00af22102...`

The management of `tree` hashes is described in [(Digression) deciphering Tree bytes](#de)!

**Supplemental**
A `tree` may contain not only a `blob` but also a `tree`.
That is, if there is a directory within a directory.
This is because `tree`, like `blob`, does not keep the directory name of itself and the corresponding data.


## Commit
`commit` contains the `tree` of the repository directory with meta information.
The image looks like this.
![Commit.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/0cee461c-c465-26a3-6014-7baf5f0dd471.png)


Let's analyze!

```bash
>>> with open('.git/objects/48/c972ae2bb5652ada48573daf6d27c74db5a13f', 'rb') as f:
...     content = f.read()
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'commit 188\x00tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5\nauthor nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\ncommitter nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\n\ninitial\n'
>>> decompressed.split(b'\0')
[b'commit 188',
 b'tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5\nauthor nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\ncommitter nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\n\ninitial\n']

# a little bit more
>>> header, content = decompressed.split(b'\0')
>>> header
b'commit 188'
>>> content
b'tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5\nauthor nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\ncommitter nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\n\ninitial\n'
>>> content.split(b'\n')
[b'tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5',
 b'author nopeNoshishi <nope@noshishi.jp> 1674995860 +0900', 
 b'committer nopeNoshishi <nope@noshishi.jp> 1674995860 +0900', 
 b'', 
 b'initial',
 b'']
```
The stored data are as follows.
- `header` ... `commit 188`
- `Null byte` ... `\x00`
- `tree` ... `tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5`
- `author` ... `author nopeNoshishi <nope@noshishi.jp> 167...`
- `committer` ... `committer nopeNoshishi <nope@noshishi.jp> 167...`
- `message` ... `initial`

You can see that it contains the `tree` hash value that you saw in the `tree` chapter earlier, information about the repository owner and the person who made the commit, and the message.


I will go ahead with the commit and analyze it again.
Edit `first.txt` as follows and `add` and `commit` again.

`first.txt(version2)`
```text
Hello World!
This is first.txt.
Version2
```
```bash
git add first.txt
git commit -m 'second'
```

Then the contents of `.git/objects` are now as follows.
```text
.git/
└── objects/
    ├── 3f/
    |   └── f934272  # new tree .. project repo version 2
    ├── 37/
    |   └── 349c9b0  # new commit .. "second"
    ├── 48/
    |   └── c972ae2  # old commit .. "initial"
    ├── af/
    |   └── 22102d6  # old blob .. second.py version 1
    ├── c8/
    |   └── 843b4db  # new blob .. first.txt version 2
    ├── da/
    |   └── f3f26f3  # old tree .. project repo version 1
    └── f7/
        └── f18b178  # new blob .. first.txt version 1
```


See the new commit...

```bash
>>> with open('.git/objects/37/349c9b05c73281008e7b6b7453b595bb034a52', 'rb') as f:
...     content = f.read()
... 
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'commit 235\x00tree 3ff9342727caf81397740327aa406c1cc6d4408e\nparent 48c972ae2bb5652ada48573daf6d27c74db5a13f\nauthor nopeNoshishi <nope@noshishi.jp> 1675174139 +0900\ncommitter nopeNoshishi <nope@noshishi.jp> 1675174139 +0900\n\nsecond\n'
```
The stored data are as follows.
- `header` ... `commit 188`
- `Null byte` ... `\x00`
- `tree` ... `tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5`
- `parent` ... `parent 48c972ae2bb5652ada48573daf6d27c74db5a13f`
- `author` ... `author nopeNoshishi <nope@noshishi.jp> 167...`
- `committer` ... `committer nopeNoshishi <nope@noshishi.jp> 167...`
- `message` ... `second`

The new commit stored the hash value of the previous version of `commit`.



**Supplemental**

The difference against `blob` or `tree` is that `commit` does not store the actual data in the repository. But it has meta data starting from `tree`.


## Key-Value Store
Some of you may have an idea of what I'm talking about.

If you unravel a `commit`, you can get a `tree`, and if you unravel a `tree`, you can get a `blob`.


<img width="400" alt="つながり.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/665545d3-7b06-df09-bc82-909dc4174ec1.png">

The version flow shows the history because `commit` knows the hash value of the previous `commit`.
This image shows the history of the current commit.

<img width="600" alt="つながり.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/94be8594-4225-0089-056e-fe0d204d1ad5.png">

So Git manages file versions from **the starting point, which is the hash value of the object**.

**(Info)**
Officially, Git is called **Address (hash) File System**.
The hash function itself is an `invertible transformation`, so the original data cannot be restored from the hash value, but as long as the hash value depends on the contents of the object to begin with, it may be called a **value-value store**.


## Summary
In a world without version control systems like Git, what do you do when you **want to keep your current files and work on something new with the same files**?
Perhaps one way you might think of doing this is to copy the file and put it in another folder.
In fact, this seemingly weird management method is the closest form of version control that supports Git.

**(Info)**
Git is a storage system that makes clever use of the OS file system.


# Analize Index
The index (staging area) is veiled, but like the object, the design is very simple.
(On the other hand, it is a bit quirky to analyze. The dismantling of the index sucked up dozens of hours...

I'm going to analyze `.git/index`, which has been committed for the second time.

## Specification
In order to analyze, we need to understand the design specification of `index`.

Referring to [Index format](https://git-scm.com/docs/index-format) in the official document, we found the following specifications.

```text
Index Format
Header
    - 4 bytes   Index header                * DIRC
    - 4 bytes   Index version   　　　　     * basic version 2
    - 32 bits   number of entries in index  * Entries are the meta information for each file.

エントリー
    - 32 bits   create file time
    - 32 bits   create file time at nano
    - 32 bits   modify file time
    - 32 bits   modify file time at nano
    - 32 bits   device id
    - 32 bits   inode
    - 32 bits   Permission (mode)
    - 32 bits   user id
    - 32 bits   group id
    - 32 bits   file size
    - 160 bits  `blob` hash value
    - 16 bits   filename size               * Number of bytes in filename string
    - ?  bytes  filename                    * Variable depending on file name
    - 1-8 bytes padding                     * Variable depending on entry

... The same thing continues by number of entries ....
```


### Index
Now that we have the specifications, we will read them again in python.

The `index` is uncompressed, but reads in binary format as well as the object because all meta information is stored in bytes.
```bash
>>> with open('.git/index', 'rb') as f:
...     index = f.read()
>>> index
b'DIRC\x00\x00\x00\x02\x00\x00\x00\x02c\xd9 \xf4\x05\xeb\x80\xb2c\xd9 \xf4\x05\xeb\x80\xb2\x01\x00\x00\x06\x00\xb8\'\x07\x00\x00\x81\xa4\x00\x00\x01\xf5\x00\x00\x00\x14\x00\x00\x00(\xc8\x84;M\xb8\x06\xe5\xd6Z\x12\xefV\xbfK\xeeQ\xe7\x15\'\x93\x00\tfirst.txt\x00c\xd6hv\x17\xa5\x05nc\xd6hv\x17\xa5\x05n\x01\x00\x00\x06\x00\xb8\'\x14\x00\x00\x81\xa4\x00\x00\x01\xf5\x00\x00\x00\x14\x00\x00\x00,\xaf"\x10-b\xf1\xc8\xe6\xdfR\x17\xb4\xcb\xa9\x99\x07X\x0bQ\xaf\x00\tsecond.py\x00TREE\x00\x00\x00\x19\x002 0\n?\xf94\'\'\xca\xf8\x13\x97t\x03\'\xaa@l\x1c\xc6\xd4@\x8e\xf2\xe4\xd7:\x95\xc1?\x18\xd3\xe9\x7f\x8fp\x9c$N\xc9dX\xa4'
```

It looks readable in places.
You can see the original `DIRC`, `first.txt` and `second.py`!

Since 32bits is 4bytes, it can be easily pulled out.
```bash
>>> index[0:4]
b'DIRC' # Index header -> DIRC
>>> index[4:8]
b'\x00\x00\x00\x02' # Index version => 2
>>> index[8:12]
b'\x00\x00\x00\x02' # number of entries => 2
```

The `index` manages metadata per file, so you will have two entries, `first.txt` and `second.py`.

**For the purpose of this article**, I will just take a quick look at the meta information from the next creation time to the group ID, which is not very important except for the mode.
```bash
>>> index[12:16]
b'c\xd9 \xf4' # ctime
>>> index[16:20]
b'\x05\xeb\x80\xb2' # ctime nano
>>> index[21:24]
b'\xd9 \xf4' # mtime
>>> index[24:28]
b'\x05\xeb\x80\xb2'  # mtime nano
>>> index[28:32]
b'\x01\x00\x00\x06' # dev id
>>> index[32:36]
b"\x00\xb8'\x07" # inode
>>> index[36:40]
b'\x00\x00\x81\xa4' # mode
>>> index[41:44]
b'\x00\x01\xf5' # user id
>>> index[44:48]
b'\x00\x00\x00\x14' # gorup id
```

Here are the key points to look at.
First is the file size.

```bash
# file size
>>> index[48:52]
b'\x00\x00\x00('
>>> index[48:52][0]
0
>>> index[48:52][1]
0
>>> index[48:52][2]
0
>>> index[48:52][3]
40
```

The file size of the next file to come is found to be 40bytes.

Next is the hash value.

```bash
# hash
>>> index[52:72]
b"\xc8\x84;M\xb8\x06\xe5\xd6Z\x12\xefV\xbfK\xeeQ\xe7\x15'\x93"
>>> index[52:72].hex()
'c8843b4db806e5d65a12ef56bf4bee51e7152793'
```
We see the hash value matches the one in version 2 `first.txt`!

And the size of the filename.
```bash
# filename size
>>> index[72:74]
b'\x00\t'
>>> index[72:74][0]
0
>>> index[72:74][1]
9
```
This size (in bytes) is very important, without it, you will have to search for the next file name by your feeling.

Now that we know the filename is 9 bytes, we can...
```bash
>>> index[74:83]
b'first.txt'
```
We can extract the file name without missing anything.

Finally, padding depends on the number of bytes used to represent the entry.
The calculation method is to find **X bytes** such that the bytes up to the padding plus the **X bytes** to be padded is a multiple of 8.

Expressed as a formula, X (padding), y (filename size), a (remainder)

$$
(62 + y) / 8 = quotient \dots a \\ 
8 - a = X
$$

In this case, from `creation time` to `file size`, 62 bytes, and the `file name` is 9 bytes.

$$
(62 + 9) / 8 = 8 ... 7 \\ 
8 - 7 = 1
$$

We found the bytes of padding was 1 byte.

```bash
>>> index[83:84]
b'\x00'
>>> index[83:85]
b'\x00c' # There's one that isn't a null bite, and it's from the second bite!
>>> index[83:86]
b'\x00c\xd6'
```
The bytes of padding up to the next entry `creation time` was correctly matched.

## Summary
Actually, when you `add`, `tree` is not created.
You commit, then `tree` will be generated from `index`.

`index` has important role to link added file data to `blob`s and manage which versions of files are committed.

You may have heared git dealed a snapshot, not difference.
In other words, when indexes have not been updated, file data will always remain unless explicitly excluded.
And that means that everything you commit can be restored through the index.

**(Info)**
`index` is an important entity that holds the key to whether or not a file is subject to version control in Git.


# Background of Command
Now that we know how Git handles data, let's take a quick look at how the commands behave.

The command has many options, so more complex behavior can be achieved, but I only describe a basic role.

## add
`add` is responsible for adding, deleting, and updating the target file data to the index.
When added, git creates a `blob` of the **instantaneous(latest)** file data.

The plumbing commands that make this happen are `hash-object` and `update-index`.
※In [Plumbing commands](#plumbing-commands) chapter, I describe the detail.

## commit
Git create a `tree` corresponding to the repository directory based on the index created, and then create a `commit`.
After the `commit` is successfully created, change the hash value of the `commit` that the `HEAD` and `branch` point to.

The plumbing commands that accomplish this are `write-tree`, `commit-tree`, and `update-ref`.

# Digression
## Deciphering Tree
We'll look into the byte in a bit.

What is the maximum value of a number that can be represented by a single (unsigned) byte?
2^8 - 1 = 255. This corresponds to the maximum number of hexadecimal digits that can be represented by two hexadecimal digits.

```bash
>>> temp[2][0]
247　 # = `\xf7`
```

I used the `hex()` function quickly above, but if you look at it one byte at a time...
```bash
>>> hash = ''
>>> for hex in temp[2][0:20]:
...     hash += format(hex, 'x')
>>> hash
'f7f18b17881d80bb87f281c2881f9a4663cfcf84'
```

I can get the hash value of the `blob` corresponding to `first.txt` as a string!


`hash` are 40 characters, but each character is a value calculated in hexadecimal. So the trick is that one byte can represent two characters .

`commit` stores the hash value as a string, but for some reason the `tree` stores the hash value directly as bytes, not as a string.

There was some discussion on stackoverflow as to why.

https://stackoverflow.com/questions/42009133/how-to-inflate-a-git-tree-object


## HEAD and Branch
The `Branch` is responsible for marking specific `commit` objects.
It is stored under `.git/refs/heads/`.
You can easily see the contents with the Linux command `cat`.

Since we were working on the `master` branch earlier, we can look at `.git/refs/heads/master` and see ...
```bash
% cat .git/refs/heads/master
37349c9b05c73281008e7b6b7453b595bb034a52
```
The hash value of the last committed `commit` object was stored.


The `HEAD` indicates which `commit` object you are basing your file edits on.
HEAD can point directly to a `commit` object, but it basically goes through `branch`.
`.git/HEAD` is what it is.

The data is stored as follows.
```bash
% cat .git/HEAD
ref: refs/heads/master
```

It contained the path about where the `master` branch is stored.

If you want to point directly to a commit (detached head), use `checkout` to move `HEAD`.
```bash
% git checkout 37349c9b05c73281008e7b6b7453b595bb034a52
% cat .git/HEAD
ref: 37349c9b05c73281008e7b6b7453b595bb034a52
```


## Plumbing commands
To further manipulate Git at a low level, there is a command for every single action.
(These are god-like commands created by Mr. Linus for ordinary people like me.)

### `cat-file` 
This command allows you to see the contents of an object.
We worked hard earlier to analyze the object, but this single command is the solution.

```bash
# See object type
% git cat-file -t af22102d62f1c8e6df5217b4cba99907580b51af # second.py
blob

# Output object content
% git cat-file -p af22102d62f1c8e6df5217b4cba99907580b51af # second.py
def second():
    print("This is second.py")
```

### `hash-object` 
You can hash file data, etc. or store them in `.git/objects`.

Let's create `third.rs`.
```rust
struct Third {
    message: String   
}
```

```bash
# calculate hash value
% git hash-object
4aa58eed341d5134f73f2e9378b4895e216a5cd5

# Create blob object
% git hash-object -w
4aa58eed341d5134f73f2e9378b4895e216a5cd5
% ls .git/objects/4a
a58eed341d5134f73f2e9378b4895e216a5cd5
```

### `update-index` 
This command adds the target file to the index.
Note, however, that no object is created.

### `ls-files` 
This command provides a concise view of the contents of the index.

```bash
# see the latest index
% git ls-files
first.txt
second.py

# add index third.rs cache
% git update-index --add third.rs 
% git ls-files
first.txt
second.py
third.rs
% git ls-files -s
100644 c8843b4db806e5d65a12ef56bf4bee51e7152793 0       first.txt
100644 af22102d62f1c8e6df5217b4cba99907580b51af 0       second.py
100644 4aa58eed341d5134f73f2e9378b4895e216a5cd5 0       third.rs
```

### `write-tree` 

We create a `tree` based on the contents of the index.
All directories, not just repository directory.

```bash
% git write-tree
109e41a859caa3e3b87e8f59744b0b1845efe275
% ls .git/objects/10 
9e41a859caa3e3b87e8f59744b0b1845efe275
```

### `commit-tree` 

We create the `commit` with the hash of the (repository directory) `tree`.

```bash
# Enter the hash value of the parent `commit` and the 
# hash value of the `tree` you just created
% git commit-tree -p 37349c9b05c73281008e7b6b7453b595bb034a52 -m 'third commit' 109e41a859caa3e3b87e8f59744b0b1845efe275
ddb3c0d94d860ff657e2cdb82f5513f7db2924f1
% ls .git/objects/dd 
b3c0d94d860ff657e2cdb82f5513f7db2924f1　#　object is created
```

### `update-ref`

We can't just `commit-tree` and follow the history, because no one will see the commits you made.
This is because no one can see the commits we have made.
```bash
# Because the git log follows the history sequentially 
# from the commit pointed to by HEAD, the commit you
# just created is not yet referenced.
% git log
commit 37349c9b05c73281008e7b6b7453b595bb034a52 (HEAD -> master)
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Tue Jan 31 23:08:59 2023 +0900

    second

commit 48c972ae2bb5652ada48573daf6d27c74db5a13f
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Sun Jan 29 21:37:40 2023 +0900

    initial

# Change the branch's references.
% git update-ref refs/heads/master ddb3c0d 37349c9 # new-hash old-hash
% git log
commit ddb3c0d94d860ff657e2cdb82f5513f7db2924f1 (HEAD -> master)
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Thu Feb 2 21:17:24 2023 +0900

    third commit

commit 37349c9b05c73281008e7b6b7453b595bb034a52
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Tue Jan 31 23:08:59 2023 +0900

    second

```

In creating Git, it is difficult to suddenly create something as sophisticated as `add` or `commit`.
Therefore, while implementing the plumbing command , we will create `add` and `commit` in the development section to bypass the functionality of this command.


# Finally
Thank you for reading all the way to the end!!!
This is still a rough explanation, but I hope it contributes to your understanding.
If you may ok, please star my repository!


# Reference Site
[Officail Documentation](https://git-scm.com/doc)

# What you need

Listed here are the key elements in making git.

[Binary](https://elixir-lang.org/getting-started/binaries-strings-and-char-lists.html)

[Byte](https://web.stanford.edu/class/cs101/bits-bytes.html)

[Bitwise operation](https://en.wikipedia.org/wiki/Bitwise_operation)

[n-decimal system and character strings](https://users.ece.utexas.edu/~ryerraballi/CPrimer/chap3/chap3.htm)

[String](https://en.wikipedia.org/wiki/String_(computer_science))

[Compression algorithms](https://geekyhumans.com/de/most-popular-data-compression-algorithms/)

[Hash function](https://www.thesslstore.com/blog/what-is-a-hash-function-in-cryptography-a-beginners-guide/)

[File system](https://www.javatpoint.com/linux-file-system#:~:text=What%20is%20the%20Linux%20File,more%20information%20about%20a%20file.)


# Annotation

### zlib

<span id="q1" style="font-size:x-small">1: This is a free software to compress data losslessly. The main compression algorithm called Deflate is very interesting.[Official Site](https://www.zlib.net/) [back to article](#object)</span>

### sha1
<span id="q2" style="font-size:x-small">2: One of the very famous SHA-based hash functions, characterized by the generation of a 60-bit (20-byte) hash value. Incidentally, the probability of a collision of sha1 hash values is said to be astronomical.[The Reality of SHA1](https://pthree.org/2014/03/06/the-reality-of-sha1/#:~:text=It%20should%20take%202%5E160,in%20about%202%5E80%20operations.)  [back to article](#object)</span>

### hash number
<span id="q3" style="font-size:x-small">3: When you specify a hash value directly in a Git command, you may only use 7 characters. As mentioned in [^2](#ano-2), this means that even with a small input hash value, we can find a specific object because there are almost no hash collisions. It is similar to the feeling of pressing tab in `shell` to receive input assistance. [back to article](#analyze-object)</span>

### compressed string
<span id="q4" style="font-size:x-small">4: Compressed data is stored in a form that does not correspond to a character code. Therefore, it cannot be read as a specific character code. [back to article](#blob)</span>

### mode
<span id="q5" style="font-size:x-small">5: The mode (permission) can of course also be expressed in binary. And since there are few combinations, certain combinations can be expressed in computation. [back to article](#tree)</span>
