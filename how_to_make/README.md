# Hello Dev community! 
I'm noshishi, a apprentice engineer in Tokyo.
This article is about understanding Git from the inside by creating a program that `add` and `commit`.


# Foreword
The starting point is 'If I could understand git, I could make it?!!' 
I took this opportunity to try out a new programming language, so I decided to try Rust this time.

The repository I actually created is [here](https://github.com/nopeNoshishi/nss).

The quality of the code isn't quite there yet, but I think you can do a straight line of local development!

If you give me a **star**, I'll be happy to fly, and of course I'll be waiting for your contributions! Feel free to touch this repository any way you like!

**Please forgive us for not being able to explain some of the details in this article alone. Also, we use `Rust` for development, but `Python` for the stage where we uncover Git's internals!**


<!-- TOC -->

- [What is Git?](#what-is-git)
	- [Manage versions and Distribute work](#manage-distribute)
	- [Using Git means](#using-git-means)
	- [Understanding by image](#understading-by-image)
- [Start new work](#start-new-work)
	- [Repositories](#repositories)
	- [Copy the repository and start working](#copy-the-repository)
	- [(Supplemental) Working Directory](#working-directory)
	- [Change and Add files](#change-and-add-file)
	- [Adapt to remote repositories](#adapt-to-remote)
	- [View Differences](#view-differences)
	- [(Aside) One step called staging area](#staging-area)
	- [Summary](#summary1)
- [Branch](#branch)
	- [Create new branch](#create-new-branch)
	- [Work in Branches](#work-in-branches)
	- [(Aside)Git-Flow and GitHub-Flow](#gitflow-githubflow)
	- [Summary](#summary2)
- [Merge](#merge)
	- [Fast Forward](#fast-forward)
	- [No Fast Forward](#no-fast-forward)
	- [Deal with Conflicts](#deal-with-conflicts)
	- [Delete unnecessary branches](#delete-unnecessary-branches)
	- [(aside) What is the branch?](#what-is-the-branch)
	- [Summary](#summary3)
- [Rebase](#rebase)
	- [Move the branch](#move-branch)
	- [Deal with rebase conflicts](#deal-with-rebase-conflicts)
- [Keep local repositories up-to-date](#keep-up-to-date)
	- [Branch and Repository](#branch-and-repository)
	- [Check the latest status](#check-the-latest-status)
	- [Update to the latest status](#update-to-the-status)
	- [Deal with pull conflicts](#deal-with-pull-conflicts)
	- [(Aside) Identity of pull requests](#identity-of-pull-requests)
- [Useful Functions](#useful-functions)
	- [Correct the commit](#correct-the-commit)
	- [Delete the commit](#delete-the-commit)
	- [Evacuate the work](#evacuate-the-work)
	- [Bring the commit](#bring-the-commit)
	- [Mastering HEAD](#mastering-head)
- [End](#end)
	- [Source code management without Git](#source-code-managemaent-without-git)
	- [Where is the remote repository](#where-is-the-remote-repository)
	- [Pointer](#pointer)
	- [To further understand Git](#to-further-understading-git)
- [Reference](#references)

<!-- TOC -->




# Git Inside
First, we will unpack how Git handles data, based on the official documentation.
The Git command system is very complex.
On the other hand, the way Git handles data is very simple!

### Where is repository
A repository is a directory under the control of Git, and the folder `.git` in the directory created by `init` or `clone` is the actual state of the repository.

Let's put an empty folder called `project` under Git's control.
~~~bash
$ pwd
/home/noshishi/project
$ ls -a
# nothing yet
$ git init
Initialized empty Git repository in /home/noshishi/project/.git/
$ ls -a
.git
~~~


This `.git` directory consists of the following.

~~~text
.git
├── HEAD
├── (index)  // Not created by `init`!
├── config
*
├── objects/
└── refs/
    ├── heads/
    └── tags/
~~~

**(info)**

The path types of Git repositories are difficult to understand at first glance. We have added `/` to the directory path so that you can refer to it. Also, we have omitted parts that are not explained in this article.

### Object
Git manage versions by **file data** called **objects**.  
Objects are stored in `.git/objects`.


**Types**
Objects has four types, `blob`、`tree`、`commit`、`tag`.

The contents of each and the corresponding data will be as follows.
- `blob` ... File data
- `tree` ... Directory data
- `commit` ... Metadata to manage the `tree` of the repository
- `tag` ... Metadata for a specific `commit` * Not explained at this article.

Image with `first.txt` in the `project` repository
<img width="400" alt="Object=png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/4f9a06f6-d69b-97f0-c01f-74277e1590a6.png">



**Structure**
The Object is **FILE DATA**, so it has a `file name (path)` and the `data` stored in it, just like a normal file.

**File name (path)**
ファイル名は、オブジェクトに格納する**データ**を`sha-1`[^2]というハッシュ関数に通すことで得られる、**40文字の文字列**を使って決められます。
具体的には、この40文字のうち、前の2文字をディレクトリのパスにして、残り38文字をファイルのパスにしています。

**Data**
データは、`zlib`[^1]によって圧縮されています。
伸張したデータは、`header`と`content`の２つで構成されています。
そして、この２つ要素を`\0`(NULLバイト)で区切っています。

`header`は、オブジェクトに応じた文字列と次に続く`content`のサイズを組み合わせです。
`content`は、**種類**にある通り、対応データが扱いやすく整えられて入っています（詳しくは解体の章で説明します）。

（例）`blob`ができる流れのイメージ。
![Example.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/f0d6d6bc-ba53-a41e-c0aa-9bcda54a1c49.png)


### インデックス（ステージングエリア）
`add`したときに使用されるインデックスの実態は、`.git/index`というファイルです。

**構造**
インデックスでは、`add`によってされたファイルをメタ情報と共に格納しています。
格納されているデータは、`add`されたタイミングの最新のファイルデータのメタ情報を格納しています。

重要なのは、**インデックスに記録されるデータは全てファイルデータ単位**です。
メタ情報は後ほど詳しく記述しますが、格納形式は[index-format](https://github.com/git/git/blob/v2.12.0/Documentation/technical/index-format.txt)の通りきっちり定められています。


と言ってもイメージがつかないと思うので、実際にオブジェクトとインデックスを解体してみましょう！

# オブジェクトを解体してみる
解体作業に入る前に、`blob`、`tree`、`commit`の全てを作成します。
と言っても、`project`の中にファイルを追加して、コミットするだけです。

以下、二つのファイルを作成して、、、
~~~first.txt
Hello World!
This is first.txt.
~~~

~~~second.py
def second():
    print("This is second.py")
~~~

`add`して`commit`します。
~~~bash
git add -A
git commit -m 'initial'
~~~

そうすると、`.git/objects`の中身は以下の通りとなりました。
~~~text
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
~~~

**※これ以後、本文中のハッシュ値は文字数を省略します。[^3]**

<br>
<br>

それぞれの対応するデータとハッシュ値をまとめると以下の通りです。
|ハッシュ値 |オブジェクト |対応データ  |
|---|---|---|
|`f7f18b1`|`blob`|first.txt|
|`af22102`|`blob`|second.py|
|`daf3f26`|`tree`|project direcrtory|
|`48c972a`|`commit`|コミット|

*解体作業は、インタプリタ言語であるPythonを使用し、対話的に進めていきます。

### blob
`blob`は、ファイルデータに対応したオブジェクトです。
イメージはこんな感じです。
![Blob.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3f8cbf30-2d00-eb36-d25f-73cbc96ccc34.png)


**データ**
まず、`first.txt`に対応する`f7f18b1`を見てみるとしましょう。
と思ったら、失敗してしまいました。
~~~bash
% python
>>> with open('.git/objects/f7/f18b17881d80bb87f281c2881f9a4663cfcf84', 'r') as f:
...     contnet = f.read()
UnicodeDecodeError: 'utf-8' codec can't decode byte 0xca in position 3: invalid continuation byte
~~~

コンテンツが圧縮されているので、コンテンツをそのまま文字列[^4]として読み込もうとすると失敗します
そのため、バイナリのまま読み込みます。

~~~bash
>>> with open('.git/objects/f7/f18b17881d80bb87f281c2881f9a4663cfcf84', 'rb') as f: # binaryで読み込む！
...     contnet = f.read()
>>> content
b'x\x01K\xca\xc9OR06d\xf0H\xcd\xc9\xc9W\x08\xcf/\xcaIQ\xe4\n\xc9\xc8,V\x00\xa2\xb4\xcc\xa2\xe2\x12\xbd\x92\x8a\x12=\x00\xfa-\r\x03'
~~~

そうすると無事読み込めて、バイト文字列を変数に格納できました。
それでは、公式ドキュメントにあるように、`zlib`で解凍します。

~~~bash
>>> import zlib
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'blob 31\x00Hello World!\nThis is first.txt.'
>>> decompressed.split(b'\0')
[b'blob 31', b'Hello World!\nThis is first.txt.']
~~~
公式ドキュメント通り、`blob`は、以下の要素で構成されていることがわかりました。
`header` ... `blob 31`
`Null byte` ... `\x00`　※`\x`は16進法表記
`content` ... `Hello World!\nThis is first.txt.`

**ファイル名**
次に確認すべきは、オブジェクトのハッシュ値が本当に正しいかどうかです。
オブジェクトのファイル名は、`decompressed`を`sha1`というハッシュ関数で求まった値であるはずなので、確認してみます。

~~~bash
>>> import hashlib
>>> blob = b'blob 31\x00Hello World!\nThis is first.txt.'
>>> sha1 = hashlib.sha1(blob).hexdigest()  #表示形式はhex(16進法)
>>> sha1
'f7f18b17881d80bb87f281c2881f9a4663cfcf84'
~~~
ばっちり一致しましたね！！

<br>
<br>

**もう一つのファイルはどうか**
もう一つの`second.py`に対応する`af22102`も見ておきましょう。
~~~bash
>>> with open('.git/objects/af/22102d62f1c8e6df5217b4cba99907580b51af', 'rb') as f: # binaryで読み込む！
...     contnet = f.read()
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'blob 44\x00def second():\n    print("This is second.py")'

>>> blob = b'blob 44\x00def second():\n    print("This is second.py")'
>>> sha1 = hashlib.sha1(test).hexdigest()
>>> sha1
'af22102d62f1c8e6df5217b4cba99907580b51af'
~~~
つまり、以下の通りまとめることができます。
`header` ... `blob 44`
`Null byte` ... `\x00`　※`\x`は16進法表記
`content` ... `def second():\n    print("This is second.py")`

そして、データから導かれた`sha1`の値（ハッシュ値）も見事一致しました。


:::note info
**（補足）**
`blob`自体には、対応するファイルデータのファイル名を保持していません。
`blob`の代わりにその名前を管理するオブジェクトが、`tree`になります。
:::


### Tree
`tree`は、ディレクトリデータに対応したオブジェクトです。
イメージはこんな感じです。
![Tree.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/ad49d01c-b791-d0e3-2a2f-5ae6b43e0d78.png)


`blob`同様に解凍していきます。
~~~bash
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
~~~

`tree`は、複数のコンテンツを持っているので、少し複雑です。
`tree`は、ディレクトリ内にあるデータのメタ情報である`mode`[^5]、`path`とそのデータをオブジェクトにした場合の`hash`を繰り返す形で、構成されています。

ただ、単純に`\0`で切り分けると、前データのハッシュ値と次のファイルデータのメタ情報がくっついています。
これは、メタ情報とハッシュ値を`\0`で区切っているためです。



まず、一つ目に格納されたデータを確認していきます。
分割した感じを見ると`first.txt`が格納されていそうですよね。
~~~bash
>>> temp = decompressed.split(b'\0')
>>> temp[1]
b'100644 first.txt'
>>> temp[2]
b'\xf7\xf1\x8b\x17\x88\x1d\x80\xbb\x87\xf2\x81\xc2\x88\x1f\x9aFc\xcf\xcf\x84100644 second.py'
~~~

`temp[2]`をうまく分割するために、20バイトで取り出してみます。
バイト文字列の配列アクセスは、バイト単位で行うことができます。

~~~bash
>>> temp[2][0:20]
b'\xf7\xf1\x8b\x17\x88\x1d\x80\xbb\x87\xf2\x81\xc2\x88\x1f\x9aFc\xcf\xcf\x84'
>>> temp[2][0:20].hex()
'f7f18b17881d80bb87f281c2881f9a4663cfcf84'
>>> temp[2][20:]
b'100644 second.py'
~~~

同じことを繰り返すと以下のことがわかりました。
`header` ... `tree 74`
`Null byte` ... `\x00`　※`\x`は16進法表記
`content1` ... `100644 first.txt\x00f7f18b1...`
`content2` ... `100644 second.py\x00af22102...`

`tree`のハッシュの管理については、[(余談)Treeのバイトを読み解く](#余談Treeのバイトを読み解く)で書いています！

:::note info
**（補足）**
`tree`には、`blob`だけではなく、`tree`も格納されることもあります。
つまりディレクトリ内に、ディレクトリがある場合です。
なぜなら`tree`も`blob`と同様に自身と対応するデータのディレクトリ名を保持していないためです。
:::


### Commit
`tree`は、ディレクトリデータに対応したオブジェクトです。
イメージはこんな感じです。
![Commit.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/0cee461c-c465-26a3-6014-7baf5f0dd471.png)


解凍していきます！

~~~bash
>>> with open('.git/objects/48/c972ae2bb5652ada48573daf6d27c74db5a13f', 'rb') as f:
...     content = f.read()
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'commit 188\x00tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5\nauthor nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\ncommitter nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\n\ninitial\n'
>>> decompressed.split(b'\0')
[b'commit 188',
 b'tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5\nauthor nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\ncommitter nopeNoshishi <nope@noshishi.jp> 1674995860 +0900\n\ninitial\n']

# もう少し分解してみる
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
~~~
格納されているデータは、以下の通りです。
`header` ... `commit 188`
`Null byte` ... `\x00`
`tree` ... `tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5`
`author` ... `author nopeNoshishi <nope@noshishi.jp> 167...`
`committer` ... `committer nopeNoshishi <nope@noshishi.jp> 167...`
`message` ... `initial`

先ほど`tree`の章で確認した`tree`のハッシュ値、リポジトリの所有者やコミットを行った者の情報、そしてメッセージが格納されていることがわかります。

もう少し踏み込んでみてみます。
`first.txt`を以下の通り編集して、再度`add`して`commit`します。
~~~first.txt(version2)
Hello World!
This is first.txt.
Version2
~~~
~~~bash
git add first.txt
git commit -m 'second'
~~~

そうすると、`.git/objects`の中身は以下の通りとなりました。
~~~text
.git/
└── objects/
    ├── 3f/
    |   └── f934272  # new tree .. projectリポジトリのバージョン２
    ├── 37/
    |   └── 349c9b0  # new commit .. "second"
    ├── 48/
    |   └── c972ae2  # old commit .. "initial"
    ├── af/
    |   └── 22102d6  # old blob .. second.pyのバージョン1
    ├── c8/
    |   └── 843b4db  # new blob .. first.txtのバージョン２
    ├── da/
    |   └── f3f26f3  # old tree .. projectリポジトリのバージョン1
    └── f7/
        └── f18b178  # new blob .. first.txtのバージョン1
~~~

新しいコミットを見てみると、、、

~~~bash
>>> with open('.git/objects/37/349c9b05c73281008e7b6b7453b595bb034a52', 'rb') as f:
...     content = f.read()
... 
>>> decompressed = zlib.decompress(content)
>>> decompressed
b'commit 235\x00tree 3ff9342727caf81397740327aa406c1cc6d4408e\nparent 48c972ae2bb5652ada48573daf6d27c74db5a13f\nauthor nopeNoshishi <nope@noshishi.jp> 1675174139 +0900\ncommitter nopeNoshishi <nope@noshishi.jp> 1675174139 +0900\n\nsecond\n'
~~~
格納されているデータは、以下の通りです。
`header` ... `commit 188`
`Null byte` ... `\x00`
`tree` ... `tree daf3f26f3fa03da346999c3e02d5268cb9abc5c5`
`parent` ... `parent 48c972ae2bb5652ada48573daf6d27c74db5a13f`
`author` ... `author nopeNoshishi <nope@noshishi.jp> 167...`
`committer` ... `committer nopeNoshishi <nope@noshishi.jp> 167...`
`message` ... `second`

以前のバージョンの`commit`のハッシュ値を格納していました。



:::note info
**（補足）**
`blob`と`tree`との構造の違いは、実際にリポジトリにあるデータそのものを格納しているのではなく、リポジトリであるディレクトリの`tree`を起点に、メタ的なデータを格納している点です。
:::

### キーバリューストア
ここまでくるとなんとなく察しがつく方もいらしゃると思います。
`commit`を紐解けば`tree`が、`tree`を紐解けば`blob`が読み解けることになります。

<img width="400" alt="つながり.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/665545d3-7b06-df09-bc82-909dc4174ec1.png">



バージョンの流れは、`commit`が前の`commit`のハッシュ値を知っているので、履歴がわかる。
今回のコミットした履歴を表すとこんなイメージです。

<img width="600" alt="つながり.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/94be8594-4225-0089-056e-fe0d204d1ad5.png">

つまり、Gitは**オブジェクトのハッシュ値を起点**として、ファイルのバージョンを管理しているということになります。


:::note info
ちなみに公式では、Gitのことを**アドレス（ハッシュ）ファイルシステム**と呼称しています。
ハッシュ関数自体が`不可逆変換`のため、ハッシュ値から元のデータに復元できませんが、ハッシュ値がそもそもオブジェクトの中身に依存して決まる以上、**バリューバリューストア**とも言えるかもしれませんが（笑）
:::

### まとめ
Gitのようなバージョン管理システムがない世界において、**今のファイルを残したまま、同じファイルで新しい作業を進めたい**となったとき、みなさんはどうするでしょうか？
おそらく、一つの方法として、ファイルをコピーして別のフォルダにしまっておくということを考えた方もいるかもしれません。
実は、この一見してヘンテコな管理方法をこそがGitを支えるバージョン管理に近い形になります。

:::note info
Gitは、OSのファイルシステムを巧みに活用した、ストレージシステムだと考えることができます。
:::


# インデックスを解体してみる
ベールに包まれたインデックス（ステージングエリア）ですが、これもオブジェクト同様に非常にシンプルな設計になっています。
（一方で、解析には少しばかり癖があります。インデックスの解体に、数十時間を吸われました、、、、）

２回目のコミットを終えた、`.git/index`を解体していきます。

### 仕様
解体するにあたって`index`の設計仕様を把握します。

公式ドキュメント内の[Index format](https://git-scm.com/docs/index-format)を参照にすると以下の仕様であることがわかりました。
~~~text
インデックスのフォーマット
ヘッダー
    - 4 bytes   インデックスヘッダー      *DIRCという文字列
    - 4 bytes   インデックスバージョン   　　　　*基本的にVersion２が多いと思います
    - 32 bits   インデックスのエントリー数  *エントリーは各ファイルのメタ情報のこと

エントリー
    - 32 bits   作成時間
    - 32 bits   作成時間のnano単位
    - 32 bits   変更時間
    - 32 bits   変更時間のnano単位
    - 32 bits   デバイスID
    - 32 bits   inode番号
    - 32 bits   パーミッション(mode)
    - 32 bits   ユーザーID
    - 32 bits   グループID
    - 32 bits   ファイルサイズ
    - 160 bits  `blob`のハッシュ値
    - 16 bits   ファイル名のサイズ　　　　　　　　　　　　*ファイル名の文字列のバイト数
    - ?  bytes  ファイル名            *ファイル名によって可変
    - 1-8 bytes パディング           　*エントリーによって可変

... エントリの数だけ同じことが続く
~~~


### index
仕様がわかったので、またpythonで読み解いていきます。

`index`は圧縮されてないものの、全てのメタ情報をバイトで保存しているためオブジェクト同様にバイナリ形式で読み込みます。
~~~bash
>>> with open('.git/index', 'rb') as f:
...     index = f.read()
>>> index
b'DIRC\x00\x00\x00\x02\x00\x00\x00\x02c\xd9 \xf4\x05\xeb\x80\xb2c\xd9 \xf4\x05\xeb\x80\xb2\x01\x00\x00\x06\x00\xb8\'\x07\x00\x00\x81\xa4\x00\x00\x01\xf5\x00\x00\x00\x14\x00\x00\x00(\xc8\x84;M\xb8\x06\xe5\xd6Z\x12\xefV\xbfK\xeeQ\xe7\x15\'\x93\x00\tfirst.txt\x00c\xd6hv\x17\xa5\x05nc\xd6hv\x17\xa5\x05n\x01\x00\x00\x06\x00\xb8\'\x14\x00\x00\x81\xa4\x00\x00\x01\xf5\x00\x00\x00\x14\x00\x00\x00,\xaf"\x10-b\xf1\xc8\xe6\xdfR\x17\xb4\xcb\xa9\x99\x07X\x0bQ\xaf\x00\tsecond.py\x00TREE\x00\x00\x00\x19\x002 0\n?\xf94\'\'\xca\xf8\x13\x97t\x03\'\xaa@l\x1c\xc6\xd4@\x8e\xf2\xe4\xd7:\x95\xc1?\x18\xd3\xe9\x7f\x8fp\x9c$N\xc9dX\xa4'
~~~

ところどころ読めそうなところがあります。
元の`DIRC`や`first.txt`、`second.py`が見えていますね！

仕様に沿って解体していきます。
32bitsは4bytesなので、簡単に引き出すことができます。
~~~bash
>>> index[0:4]
b'DIRC' # インデックスヘッダー -> DIRC
>>> index[4:8]
b'\x00\x00\x00\x02' # インデックスバージョン => 2
>>> index[8:12]
b'\x00\x00\x00\x02' # エントリーの数 => 2
~~~

`index`ではファイル単位でメタデータを管理しているので、`first.txt`、`second.py`の二つがエントリーとして入っていることになります。

**この記事の説明において**、次の作成時間からグループIDまで、mode以外あんまり重要ではないメタ情報なので、さっくりみるだけにします。
~~~bash
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
~~~

ここからがみておきたいポイントです。
まずはファイルサイズです。

~~~bash
# ファイルサイズ
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
~~~

次にくるファイルのファイルサイズが40bytesであることがわかりました。

次はハッシュ値です。
~~~bash
# hash
>>> index[52:72]
b"\xc8\x84;M\xb8\x06\xe5\xd6Z\x12\xefV\xbfK\xeeQ\xe7\x15'\x93"
>>> index[52:72].hex()
'c8843b4db806e5d65a12ef56bf4bee51e7152793'
~~~
ハッシュ値がバージョン２の`first.txt`のものと一致していますね！

そして、ファイル名のサイズ。
~~~bash
# ファイル名のサイズ
>>> index[72:74]
b'\x00\t'
>>> index[72:74][0]
0
>>> index[72:74][1]
9
~~~
このサイズ（バイト）が非常に重要で、これがないと次のファイル名を手探りで探すことになってしまいます。

ファイル名は９バイトとわかったので、、、
~~~bash
>>> index[74:83]
b'first.txt'
~~~
しっかりもれなくファイル名を抜き出すことができました。

最後にパディングですが、これはエントリーを表現するために使用されたバイト数に依存した形で決まります。
計算方法は、パディングまでのバイトとパディングする**Xバイト**を足したものが、8の倍数となるような**Xバイト**を求めます。

計算式で表すと、X(パディング)、y(ファイル名サイズ)、a(余り)
```math
(62 + y) / 8 = 商 ... a \\ 
8 - a = X
```

今回の場合は、
`作成時刻`から`ファイルサイズ`まで、62バイト
`ファイル名が`9バイト
```math
(62 + 9) / 8 = 8 ... 7 \\ 
8 - 7 = 1
```

パディングのバイト数が１バイトであることがわかりました。

~~~bash
>>> index[83:84]
b'\x00'
>>> index[83:85]
b'\x00c' # ヌルバイトではないものが２バイト目からある！
>>> index[83:86]
b'\x00c\xd6'
~~~
ちゃんと次のエントリ`作成時刻`の部分までのパディングのバイト数が一致していました。

### まとめ
実は、`add`した段階ではまだ`tree`オブジェクトが作成されていません。
`commit`を行ったときに、`index`をもとに`tree`オブジェクトが作成されます。

インデックスは、追加されたファイルデータを`blob`と紐付け、どのバージョンのファイルをコミットさせるかを管理する重要な要素だったということです。

よくGitは差分ではなくスナップショットであると解説されています。
インデックスが更新されていないファイルデータは、明示的に除外しない限り常に残り続けます。
そして、コミットしたものは全てインデックスを通して復元できるということです。

:::note info
ファイルをGitのバージョン管理対象にするか否かを握る重要な存在が`index`
:::

# コマンドの裏で起こっていること
Gitのデータの扱い方がわかったところで、次はコマンドがどのように振る舞うのかを簡単に見ていきます。


:::note warn
コマンドには多くのオプションがあるため、もっと複雑な動作が実現できますが、あくまでもベースのコマンドとして記述します。
:::

### add
対象とするファイルデータをインデックスに追加・削除・更新する役割を担います。
追加された場合は、その追加された **瞬間(最新)** のファイルデータの`blob`を作成します。

このコマンドを実現する配管コマンドは、
`hash-object`、`update-index`です。
※配管コマンドについては、[配管コマンド](#配管コマンド)の章で紹介します。

### commit
作成されたインデックスをもとにリポジトリディレクトリに対応する`tree`を作成し、その後`commit`を作成します。
無事`commit`が作成されたら、`HEAD`や`branch`がポイントする`commit`のハッシュ値を変更します。

このコマンドを実現する配管コマンドは、
`write-tree`、`commit-tree`、`update-ref`です。

# 余談
### Treeのバイトを読み解く
少しバイトについて調べてみます。

（符号なし）１バイトで表せれる数字の最大値は幾つでしょうか。
2^8 - 1 = 255です。これは、16進法の数を二つで表せる最大の数と一致します。
~~~bash
>>> temp[2][0]
247　 #\xf7と一致する数字
~~~

上ではさくっと`hex()`関数を使っちゃいましたが、１バイトづつ見ていくと、、、

~~~bash
>>> hash = ''
>>> for hex in temp[2][0:20]:
...     hash += format(hex, 'x')
>>> hash
'f7f18b17881d80bb87f281c2881f9a4663cfcf84'
~~~

`first.txt`に対応する`blob`のハッシュ値が文字列として獲得できました！


`hash`は文字列としては40文字ですが、1文字づつは１６進法で計算された値なので、2文字を1バイトで表せれるというのがカラクリでした。
`commit`は文字列として格納しているのに、`tree`では、なぜかハッシュ値が文字列としてではなく、バイトとして直接格納されています。

stackoverflowでもなんでやねんの議論がありました。

https://stackoverflow.com/questions/42009133/how-to-inflate-a-git-tree-object


### HEADとBranch
`Branch`は、特定の`commit`オブジェクトにマーキングする役割があります。
`.git/refs/heads/`以下に格納されています。
中身を見るのはLinuxコマンドの`cat`で簡単に見れます。

先ほどは`master`ブランチで作業していたので、`.git/refs/heads/master`を見てみると、、
~~~bash
% cat .git/refs/heads/master
37349c9b05c73281008e7b6b7453b595bb034a52
~~~
直前にコミットした`commit`オブジェクトのハッシュ値が格納されていました。


`HEAD`は、自分がどの`commit`オブジェクトをベースにファイルの編集を行なっているかを示しています。
HEADは、直接`commit`オブジェクトを指すこともできますが、基本的に`branch`を経由します。
`.git/HEAD`がその正体です。

今の段階であると以下のようにデータが格納されています。
~~~bash
% cat .git/HEAD
ref: refs/heads/master
~~~

`master`ブランチの格納場所についてのパスが入っていました。

直接コミットを指したい場合(detached head)は、`checkout`で`HEAD`を動かします。
~~~bash
% git checkout 37349c9b05c73281008e7b6b7453b595bb034a52
% cat .git/HEAD
ref: 37349c9b05c73281008e7b6b7453b595bb034a52
~~~


### 配管コマンド
Gitをさらにローレベルで操作するために、一つの動作ごとにコマンドが存在します。
（リーナス氏が、私のような凡人のために作ってくれた神のようなコマンドです。）

`cat-file` 
オブジェクトの中身を見ることができるコマンドです。
先ほど頑張ってオブジェクトを解体しましたが、このコマンド一つで解決です。
~~~bash
# オブジェクトタイプを見る
% git cat-file -t af22102d62f1c8e6df5217b4cba99907580b51af # second.py
blob

# オブジェクトを標準出力で見る
% git cat-file -p af22102d62f1c8e6df5217b4cba99907580b51af # second.py
def second():
    print("This is second.py")
~~~

`hash-object` 
ファイルデータ等をハッシュ化したり、そのまま`.git/objects`に格納することができます。

`third.rs`を作成してみます。
~~~rust
struct Third {
    message: String   
}
~~~

~~~bash
# ハッシュ値を求める
% git hash-object
4aa58eed341d5134f73f2e9378b4895e216a5cd5

# オブジェクトを作成する
% git hash-object -w
4aa58eed341d5134f73f2e9378b4895e216a5cd5
% ls .git/objects/4a
a58eed341d5134f73f2e9378b4895e216a5cd5
~~~

`update-index` 
インデックスに、対象ファイルをインデックスに追加します。
ただし、オブジェクトは作成されないので、要注意です。

`ls-files` 
インデックスの中身を簡潔に見ることができるコマンドです。

~~~bash
# 今の段階で見てみる
% git ls-files
first.txt
second.py

# 追加してみてみる
% git update-index --add third.rs 
% git ls-files
first.txt
second.py
third.rs
% git ls-files -s
100644 c8843b4db806e5d65a12ef56bf4bee51e7152793 0       first.txt
100644 af22102d62f1c8e6df5217b4cba99907580b51af 0       second.py
100644 4aa58eed341d5134f73f2e9378b4895e216a5cd5 0       third.rs
~~~

`write-tree` 
インデックスの内容をもとに`tree`を作成します。
リポジトリディレクトリだけではなく、すべてのディレクトリが対象です。
~~~bash
% git write-tree
109e41a859caa3e3b87e8f59744b0b1845efe275
% ls .git/objects/10 
9e41a859caa3e3b87e8f59744b0b1845efe275
~~~

`commit-tree` 
作成されたリポジトリディレクトリの`tree`のハッシュ値を引数に受けて、`commit`を作成します。
~~~bash
# 親となる`commit`のハッシュ値と先ほど作った`tree`のハッシュ値を入力する
% git commit-tree -p 37349c9b05c73281008e7b6b7453b595bb034a52 -m 'third commit' 109e41a859caa3e3b87e8f59744b0b1845efe275
ddb3c0d94d860ff657e2cdb82f5513f7db2924f1
% ls .git/objects/dd 
b3c0d94d860ff657e2cdb82f5513f7db2924f1　#　オウジェクトが作成されている。
~~~

`update-ref`
`commit-tree` しただけでは履歴を追うことができません。
なぜなら、せっかく作ったコミットを誰も参照していないからです。
~~~bash
# git logはHEADが指しているコミットから順に歴史を追うので
# 先ほど作成したコミットはまだ参照されていない。
% git log
commit 37349c9b05c73281008e7b6b7453b595bb034a52 (HEAD -> master)
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Tue Jan 31 23:08:59 2023 +0900

    second

commit 48c972ae2bb5652ada48573daf6d27c74db5a13f
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Sun Jan 29 21:37:40 2023 +0900

    initial

# このコマンドでブランチの参照先を変えてあげる
% git update-ref refs/heads/master ddb3c0d 37349c9  # 新　旧
% git log
commit ddb3c0d94d860ff657e2cdb82f5513f7db2924f1 (HEAD -> master)
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Thu Feb 2 21:17:24 2023 +0900

    third commit

commit 37349c9b05c73281008e7b6b7453b595bb034a52
Author: nopeNoshishi <nope@noshishi.jp>
Date:   Tue Jan 31 23:08:59 2023 +0900

    second

~~~


Gitを作成する上で、`add`や`commit`のような高機能なものをいきなり作るのは難しいです。
そのため、配管コマンドをうまく実装しながら、このコマンドの機能をバイパスに開発編では`add`や`commit`を作成します。

# 最後に
最後まで読んでくださりありがとうございました！
まだまだ荒い解説ですが、皆さんの理解に少しでも貢献できれば幸いです。


次の開発編もみていただけると幸いです。

# 参考サイト
[公式ドキュメント](https://git-scm.com/doc)
[Gitのステージング領域の正体を知る](https://engineering.mercari.com/blog/entry/2017-04-06-171430/)

# Gitを作る上で理解しておくといいこと
バイナリ

https://elixir-lang.jp/getting-started/binaries-strings-and-char-lists.html

バイト　

https://xtech.nikkei.com/atcl/nxt/column/18/00754/051600003/


ビット演算 

https://qiita.com/Ingward/items/43acda931c8a62c70d2f


n進法と文字列

https://detail.chiebukuro.yahoo.co.jp/qa/question_detail/q11214444166


文字列

https://ja.wikipedia.org/wiki/%E6%96%87%E5%AD%97%E5%88%97


文字列解析、圧縮アルゴリズム

https://www.iwanami.co.jp/book/b257894.html

ハッシュ関数

https://www.jipdec.or.jp/project/research/why-e-signature/hash-function.html#:~:text=%E3%83%8F%E3%83%83%E3%82%B7%E3%83%A5%E9%96%A2%E6%95%B0%EF%BC%88Hash%20Function%EF%BC%89%E3%81%A8,%E9%96%A2%E6%95%B0%EF%BC%88%E3%82%A2%E3%83%AB%E3%82%B4%E3%83%AA%E3%82%BA%E3%83%A0%EF%BC%89%22%E3%81%A7%E3%81%99%E3%80%82


ファイルシステム

https://lpi.or.jp/lpic_all/linux/intro/intro10.shtml#:~:text=OS%E3%81%AE%E5%9F%BA%E6%9C%AC%E6%A9%9F%E8%83%BD%E3%81%AE,%E6%A7%8B%E9%80%A0%E3%81%AB%E3%81%AA%E3%81%A3%E3%81%A6%E3%81%84%E3%81%BE%E3%81%99%E3%80%82

[^1]: データを可逆圧縮するフリーソフトウェアです。メインのDeflateと呼ばれる圧縮アルゴリズムがとても面白いのでぜひみてさい！[公式サイト](https://www.zlib.net/)

[^2]: とても有名なSHA系のハッシュ関数の一つです。60ビット（20バイト）のハッシュ値を生成するのが特徴です。ちなみに、sha1のハッシュ値の衝突する可能性は天文学的な確率になるそうです。[Gitのhashが衝突するのはどれくらいの確率か](http://blog.mwsoft.jp/article/173786778.html)
[^3]: Gitのコマンドでハッシュ値を直接指定する場合あ、よく7文字くらいでのハッシュ値で指定することがあると思います。[2]で述べたように、入力が少ないハッシュ値でも、ほとんどハッシュ衝突しないからこそ特定のオブジェクトを見つけることができるということです。`shell`でtabを押して入力を補助を受ける感じと似ています。
[^4]: 圧縮されたデータは文字コードと対応しない形でデータが保存されています。そのため、特定の文字コードとして読み込めません。[UTF-8（ユーティーエフエイト）とは？](https://ferret-plus.com/7006)
[^5]:mode(パーミッション)ももちろんバイナリで表現できます。そして、組み合わせが少ないので、特定の組み合わせを計算で表現できるようになっています。[アクセス権（パーミッション）の記号表記と数値表記](https://kazmax.zpp.jp/linux_beginner/permission_numerical.html)
