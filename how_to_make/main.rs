
// File sturuct
use std::fs::File;
use std::fs; //追加
use std::env; //追加
// File stuructにトレイト実装
use std::io::prelude::*; 
// sha1計算用クレート
use sha1::{Digest, Sha1};
// 圧縮用のクレート
use flate2::Compression; //追加
use flate2::write::ZlibEncoder; //追加
use flate2::read::ZlibDecoder;
// Metadata
use std::os::unix::prelude::MetadataExt; // osによって実装トレイトが違うので注意
// Pathbuf sturuct
use std::path::PathBuf;
// バイトを操作するクレート
use byteorder::{ByteOrder, BigEndian};
// clapクレート
use clap::{Command, Arg};
// 時刻を習得するクレート
use chrono::Utc;

fn main() -> std::io::Result<()> {
    let cmd = Command::new("ngit")
        .about("This is Original Git") 
        .version("0.1.0")
        .author("Noshishi. <noshishi@noshishi.com>")
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(Command::new("add")
            .about("Snapshot latest working directory")
            .arg(Arg::new("file")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(false)
            .value_name("file")))
        .subcommand(Command::new("commit")
            .about("Register snapshot(tree object) as commit object in local repository")
            .arg(Arg::new("message")
            .short('m')
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .help("Add message to commit object ")
            .required(true)));

    match cmd.get_matches().subcommand() {
        Some(("add", sub_m)) => {
            let filename: Option<&String>  = sub_m.get_one("file");
            match filename {
                Some(f) => add(f)?,
                None => panic!("Required file path"),
            }
        },
        Some(("commit", sub_m)) => {
            let message: Option<&String> = sub_m.get_one("message");
            match message {
                Some(m) => commit(m)?,
                None => panic!("Required message"),
            }
        },
        _ => {},
    }
    
    Ok(())
}


#[allow(dead_code)]
fn write_blob(filename: &str) -> std::io::Result<()> {
    let mut path = env::current_dir()?;
    path.push(PathBuf::from(filename));
    let mut f = File::open(path)?;

    let mut content = String::new(); // データを格納するバッファーを用意
    f.read_to_string(&mut content)?;

    // objectは `header`+`\0`+`content`
    let blob_content = format!("blob {}\0{}", content.len(), content); // Rustのlen()は、文字列の文字数ではなくバイト数を返す。

    // 格納する文字列
    // println!("blob content: {}", blob_content);

    // hash値を計算
    let blob_hash = Sha1::digest(blob_content.as_bytes());
    // println!("blob hash: {:x}", blob_hash);
    
    // 圧縮用のバッファを用意する
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob_content.as_bytes())?; // 書き込むときはバイトで行う; 
    let compressed = encoder.finish()?;

    // hash値を文字列にする
    let hash = format!("{:x}", blob_hash);
    // hash値の2文字までがdirectory pathで、38文字がfile path
    let (dir, file) = hash.split_at(2);
 
    // 全てのOSのパスを表現するためにPathBufを使用します。
    let mut current_path = env::current_dir()?; // path/to/ngit
    current_path.push(".git/objects"); // path/to/ngit/.git/objects
    current_path.push(dir); // path/to/ngit/.git/objects/b4

    let object_dir = current_path.clone(); // 後でもう一度使うのでここでは所有権は渡さない！

    // .git/obejects/に格納するためにディレクトリを作成する
    fs::create_dir_all(object_dir)?;

    // file pathは、hash値で決まり、、、
    current_path.push(file); // path/to/ngit/.git/objects/b4/aa0076e9b36b2aed8ca8a21ccdd210c905660a
    let object_path = current_path;
    // 中身は圧縮したコンテンツを入れる
    let mut f = File::create(object_path)?;
    f.write_all(&compressed)?;
    f.flush()?;

    Ok(())
}

#[allow(dead_code)]
fn cat_blob(hash: &str) -> std::io::Result<()> {
    // hash値の2文字までがdirectory pathで、38文字がfile path
    let (dir, file) = hash.split_at(2);

    // オブジェクトまでのパスを習得する
    let mut current_path = env::current_dir()?;
    current_path.push(".git/objects");
    current_path.push(dir);
    current_path.push(file);
    let object_path = current_path;

    // オブジェクトを開いてバイナリで読み込む
    let mut compressed = Vec::new(); // 読み込み用のバッファはbyteのベクタで用意
    let mut file = File::open(object_path)?;
    file.read_to_end(&mut compressed)?;

    // 読み込んだ圧縮データを伸長する
    let mut object_content:Vec<u8> = Vec::new(); // 伸長用のバッファはbyteのベクタで用意
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    decoder.read_to_end(&mut object_content)?;

    // ヌルバイトでheaderとcontentを分離する。
    let mut contents = object_content.splitn(2, |&x| x == b'\0');
    println!("header:\n{}", String::from_utf8(contents.next().unwrap().to_vec()).ok().unwrap());
    println!("file contnet:\n{}", String::from_utf8(contents.next().unwrap().to_vec()).ok().unwrap());

    Ok(())
}

#[allow(dead_code)]
fn create_entry(filename: &str) -> std::io::Result<Vec<u8>> {
    // path名からPathBuf構造体を作成
    let mut path = env::current_dir()?;
    path.push(PathBuf::from(filename));
    // トレイトでMetadata構造体が取り出せる
    let metadata = path.metadata()?;

    let ctime = metadata.ctime() as u32;
    let ctime_nsec = metadata.ctime_nsec() as u32;
    let mtime = metadata.mtime() as u32;
    let mtime_nsec = metadata.mtime_nsec() as u32;
    let dev = metadata.dev() as u32;
    let ino = metadata.ino() as u32;
    let mode = metadata.mode() as u32;
    let uid = metadata.uid() as u32;
    let gid= metadata.gid() as u32;
    let filesize = metadata.size() as u32;

    // blobのハッシュ値
    // write_blobのハッシュ計算部分を使用する
    let mut f = File::open(path)?;
    let mut content = String::new(); // データを格納するバッファーを用意
    f.read_to_string(&mut content)?;
    let blob_content = format!("blob {}\0{}", content.len(), content); // Rustのlen()は、文字列の文字数ではなくバイト数を返す。
    let blob_hash = Sha1::digest(blob_content.as_bytes());
    let hash = blob_hash.as_slice();

    // ファイル名のサイズ
    let filename_size = filename.len() as u16;

    // ファイル名
    // let filename = "first.txt";

    // padding
    let padding_size = padding(filename_size as usize);
    let padding = vec![b'\0'; padding_size];

    // コンテンツを見る
    // println!("コンテンツ！");
    // println!("ctime: {}\nctime_nsec: {}\nmtime: {}\nmtime_nsec: {}\ndev: {}\nino: {}\nmode: {}\nuid: {}\ngid: {}\nfilesize: {}\nhash: {}\nfilename_size: {}\nfilename: {}\npadding_size: {}",
    //     ctime, ctime_nsec, mtime, mtime_nsec, dev, ino, mode, uid, gid, filesize, format!("{:x}", blob_hash), filename_size, filename, padding_size
    // );
    // println!("");

    // コンテンツのバイト数を見る
    // println!("コンテンツのバイト数！");
    // println!("ctime: {:?}\nctime_nsec: {}\nmtime: {}\nmtime_nsec: {}\ndev: {}\nino: {}\nmode: {}\nuid: {}\ngid: {}\nfilesize: {}\nhash: {}\nfilename_size: {}\nfilename: {}\npadding_size: {}",
    //     ctime.to_be_bytes().len(), ctime_nsec.to_be_bytes().len(), mtime.to_be_bytes().len(), mtime_nsec.to_be_bytes().len(), dev.to_be_bytes().len(), ino.to_be_bytes().len(), mode.to_be_bytes().len(), uid.to_be_bytes().len(), gid.to_be_bytes().len(), filesize.to_be_bytes().len(), hash.len(), filename_size.to_be_bytes().len(), filename.as_bytes().len(), padding_size
    // );

    // 全てのコンテンツをバイトで繋ぐ
    let entry_meta = [ctime.to_be_bytes(), ctime_nsec.to_be_bytes(),
        mtime.to_be_bytes(), mtime_nsec.to_be_bytes(), dev.to_be_bytes(),
        ino.to_be_bytes(), mode.to_be_bytes(), uid.to_be_bytes(),
        gid.to_be_bytes(), filesize.to_be_bytes()].concat();
    
    let filemeta_vec = [entry_meta, hash.to_vec(), Vec::from(filename_size.to_be_bytes()),
        filename.as_bytes().to_vec(), padding].concat();

    Ok(filemeta_vec)
}

#[allow(dead_code)]
fn padding(size: usize) -> usize {
    // calclate padding size
    let floor = (size - 2) / 8;
    let target = (floor + 1) * 8 + 2;
    let padding = target - size;

    padding
}

#[allow(dead_code)]
fn write_index(filenames: Vec<&str>) -> std::io::Result<()> {

    // コンテンツの中身を入れる変数を束縛
    let mut content:Vec<Vec<u8>> = vec![];

    // header部分をバイトで集める
    let index_header = b"DIRC";
    let index_version = 2 as u32;
    let entry_num = filenames.len() as u32; 
    let header = [*index_header, index_version.to_be_bytes(), entry_num.to_be_bytes()].concat();
    content.push(header);

    // entry部分をバイトで集める
    for filename in filenames {
        let entry = create_entry(filename)?;
        content.push(entry)
    }

    let mut path = env::current_dir()?;
    path.push(PathBuf::from(".git/index"));
    let mut file = File::create(path)?;
    file.write_all(content.concat().as_slice())?;
    file.flush()?;

    Ok(())
}

#[allow(dead_code, unused_variables)]
fn ls_files() -> std::io::Result<()> {
    // indexをバイトで読み込む
    let mut file = File::open(".git/index")?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    // まずはエントリー数を確認
    let entry_num = BigEndian::read_u32(&buf[8..12]) as usize;
    // どの位置からエントリーの情報があるかを特定、はじめは13バイト目から
    let mut start_size = 12 as usize;

    for _ in 0..entry_num {
        // スタート位置から24-27にかけてファイルのmode
        let mode = BigEndian::read_u32(&buf[(start_size+24)..(start_size+28)]) as u32;
        // スタート位置から40-60にかけてファイル（blob）のハッシュ値
        let hash = (&buf[(start_size+40)..(start_size+60)]).to_vec();
        // スタート位置から60-61にかけてファイル名のサイズ
        let filename_size = BigEndian::read_u16(&buf[(start_size+60)..(start_size+62)]) as u16;
        // スタート位置から62-?にかけてファイル名
        let filename = (&buf[(start_size+62)..(start_size+62+filename_size as usize)]).to_vec();

        // paddingを計算して、次のエントリのバイト数を特定する
        let padding_size = padding(filename_size as usize);
        start_size = start_size + 62 + filename_size as usize + padding_size; 

        // noオプション
        println!("{}", String::from_utf8(filename).ok().unwrap());
        // -sオプション
        // println!("{:0>6o} {} 0\t{}", mode,  hex::encode(hash), String::from_utf8(filename).ok().unwrap());
    }

    Ok(())
}

#[allow(dead_code)]
fn update_index(file_path: &str) -> std::io::Result<()> {
    // ファイルのパスは絶対パスで指定
    let mut path = env::current_dir()?;
    path.push(PathBuf::from(".git/index"));

    // ファイルが存在すれば中身を取り出す
    let buf = match File::open(path.clone()) {
        Ok(mut f) => {
            let mut buf: Vec<u8> = vec![];
            f.read_to_end(&mut buf)?;
            buf
        },
        Err(_) => {
            vec![]
        } 
    };

    if buf == vec![] {
        write_index(vec![file_path])?;
    } else {
        // indexの中にあるファイル名を収集
        let mut file_paths: Vec<String> = vec![];

        // まずはエントリー数を確認
        let entry_num = BigEndian::read_u32(&buf[8..12]) as usize;
        // どの位置からエントリーの情報があるかを特定、はじめは13バイト目から
        let mut start_size = 12 as usize;
        for _ in 0..entry_num {
            // スタート位置から60-61にかけてファイル名のサイズ
            let filename_size = BigEndian::read_u16(&buf[(start_size+60)..(start_size+62)]) as u16;
            // スタート位置から62-?にかけてファイル名
            let filename = (&buf[(start_size+62)..(start_size+62+filename_size as usize)]).to_vec();
    
            // paddingを計算して、次のエントリのバイト数を特定する
            let padding_size = padding(filename_size as usize);
            start_size = start_size + 62 + filename_size as usize + padding_size;
            
            let filename = String::from_utf8(filename).ok().unwrap();
            file_paths.push(filename);

        }
        // すでにファイルがあるかどうか、なければ新しく追加する
        if !file_paths.iter().any(|e| e==&file_path) {
            file_paths.push(file_path.to_owned())
        } 
        // ファイル名順にindexを作成するためにsortする
        file_paths.sort();
        // 新しい対象ファイル群でindexを作成する。
        write_index(file_paths.iter().map(|s| &**s).collect())?;
    }

    Ok(())
}

#[allow(dead_code)]
fn write_tree() -> std::io::Result<String> {
    // indexをバイトで読み込む
    let mut path = env::current_dir()?;
    path.push(PathBuf::from(".git/index"));
    let mut file = File::open(path)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    // まずはエントリー数を確認
    let entry_num = BigEndian::read_u32(&buf[8..12]) as usize;
    // どの位置からエントリーの情報があるかを特定、はじめは13バイト目から
    let mut start_size = 12 as usize;

    let mut entries: Vec<Vec<u8>> = vec![];
    for _ in 0..entry_num {
        // スタート位置から24-27にかけてファイルのmode
        let mode = BigEndian::read_u32(&buf[(start_size+24)..(start_size+28)]) as u32;
        // スタート位置から40-60にかけてファイル（blob）のハッシュ値
        let hash = (&buf[(start_size+40)..(start_size+60)]).to_vec();
        // スタート位置から60-61にかけてファイル名のサイズ
        let filename_size = BigEndian::read_u16(&buf[(start_size+60)..(start_size+62)]) as u16;
        // スタート位置から62-?にかけてファイル名
        let filename = (&buf[(start_size+62)..(start_size+62+filename_size as usize)]).to_vec();

        // paddingを計算して、次のエントリのバイト数を特定する
        let padding_size = padding(filename_size as usize);
        start_size = start_size + 62 + filename_size as usize + padding_size; 

        // 各entryの構造は、`mode filename\0hash` 
        // ただし、modeだけ８進法として格納する
        let entry_header  = format!("{:0>6o} {}\0", mode, String::from_utf8(filename).ok().unwrap());
        // hashはバイトのまま打ち込む
        let entry = [entry_header.as_bytes(), &hash].concat();

        entries.push(entry);
    }

    // entryを一つにまとめて、treeのheaderと合わせます
    let content = entries.concat();
    let header = format!("tree {}\0", content.len());
    let tree_content = [header.as_bytes().to_vec(), content].concat();
    
    // 圧縮用のバッファを用意する
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&tree_content)?; // 書き込むときはバイトで行う; 
    let compressed = encoder.finish()?;

    // hash値を計算
    let tree_hash = Sha1::digest(&tree_content);
    // println!("tree hash: {:x}", tree_hash);
    // hash値を文字列にする
    let hash = format!("{:x}", tree_hash);
    // hash値の2文字までがdirectory pathで、38文字がfile path
    let (dir, file) = hash.split_at(2);
 
    // 全てのOSのパスを表現するためにPathBufを使用します。
    let mut current_path = env::current_dir()?; // path/to/ngit
    current_path.push(".git/objects"); // path/to/ngit/.git/objects
    current_path.push(dir); // path/to/ngit/.git/objects/b4

    let object_dir = current_path.clone(); // 後でもう一度使うのでここでは所有権は渡さない！

    // .git/obejects/に格納するためにディレクトリを作成する
    fs::create_dir_all(object_dir)?;

    // file pathは、hash値で決まり、、、
    current_path.push(file); // path/to/ngit/.git/objects/b4/aa0076e9b36b2aed8ca8a21ccdd210c905660a
    let object_path = current_path;
    // 中身は圧縮したコンテンツを入れる
    let mut f = File::create(object_path)?;
    f.write_all(&compressed)?;
    f.flush()?;

    Ok(hash)
}

#[allow(dead_code)]
fn commit_tree(tree_hash: &str, message: &str) -> std::io::Result<Option<String>> {

    // Build commit object
    // 1. tree
    let tree_hash = format!("tree {}", tree_hash);

    // 3. authorとcommitter
    // 本来はconfigファイルから取り出すべきですが、今回は固定値
    let author = format!("author {} <{}> {} +0900", 
        "noshishi",
        "nopeNoshishi@nope.noshishi",
        Utc::now().timestamp()
    );
    let committer = format!("committer {} <{}> {} +0900", 
        "noshishi",
        "nopeNoshishi@nope.noshishi",
        Utc::now().timestamp()
    );

    
    // HEADに入っているコミットが親コミットになるので、読み込む
    // そのための補助関数としてread_head()を下で定義
    let commit_content = match read_head()? {
        // 2. parent
        Some(h) => {
            // 直前(HEAD)の`commit`に含まれる`tree`と同じであればコミットしない
            if tree_hash.as_str() != cat_commit_tree(h.as_str())? {
                let parent = format!("parent {}", h);
                let content = format!("{}\n{}\n{}\n{}\n\n{}\n", 
                    tree_hash, parent, author, committer, message);
                // commitの中身
                format!("commit {}\0{}", content.len(), content).as_bytes().to_vec()
            } else {
                return  Ok(None);
            }
        },
        _ => {
            let content = format!("{}\n{}\n{}\n\n{}\n", 
                tree_hash, author, committer, message);
            // commitの中身
            format!("commit {}\0{}", content.len(), content).as_bytes().to_vec()
        }
    };

    // 圧縮用のバッファを用意する
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&commit_content)?; // 書き込むときはバイトで行う; 
    let compressed = encoder.finish()?;

    // hash値を計算
    let commit_hash = Sha1::digest(&commit_content);
    // println!("commit hash: {:x}", commit_hash);
    // hash値を文字列にする
    let hash = format!("{:x}", commit_hash);
    // hash値の2文字までがdirectory pathで、38文字がfile path
    let (dir, file) = hash.split_at(2);
 
    // 全てのOSのパスを表現するためにPathBufを使用します。
    let mut current_path = env::current_dir()?; // path/to/ngit
    current_path.push(".git/objects"); // path/to/ngit/.git/objects
    current_path.push(dir); // path/to/ngit/.git/objects/b4

    let object_dir = current_path.clone(); // 後でもう一度使うのでここでは所有権は渡さない！

    // .git/obejects/に格納するためにディレクトリを作成する
    fs::create_dir_all(object_dir)?;

    // file pathは、hash値で決まり、、、
    current_path.push(file); // path/to/ngit/.git/objects/b4/aa0076e9b36b2aed8ca8a21ccdd210c905660a
    let object_path = current_path;
    // 中身は圧縮したコンテンツを入れる
    let mut f = File::create(object_path)?;
    f.write_all(&compressed)?;
    f.flush()?; 

    Ok(Some(hash))
}

#[allow(dead_code)]
/// HEADを読む場合に、一番最初のコミットだけハッシュ値をさしていないので、
/// 返り値はOption<String>>型でおいておく。
fn read_head() -> std::io::Result<Option<String>> {
    // HEADの構造は、"refs: <branch-path> or <hash>""
    let mut path = env::current_dir()?;
    path.push(PathBuf::from(".git/HEAD"));
    let mut file = File::open(path)?;
    let mut referece = String::new();
    file.read_to_string(&mut referece)?;

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    // HEADにブランチ名が入っている場合は、ブランチの中身のハッシュ値がHEADのさすハッシュ値になる
    if prefix_path[1].contains("/") {
        // branchのパスは、".git/refs/heads/<branch-name>"
        let mut branch_path = env::current_dir()?;
        branch_path.push(PathBuf::from(".git"));
        branch_path.push(prefix_path[1].replace("\n", ""));
        // println!("{:?}", prefix_path[1]);

        match File::open(branch_path) {
            Ok(mut f) => {
                let mut hash = String::new();
                f.read_to_string(&mut hash)?;
                return Ok(Some(hash.replace("\n", "")))
            },
            // 一番最初はブランチがないのでハッシュ値はなし
            Err(_e) => return Ok(None)
        }
    }

    // 直接ハッシュ値が格納されていた場合
    Ok(Some(prefix_path[1].replace("\n", "").to_owned()))
}

#[allow(dead_code)]
fn update_ref(commit_hash: &str) -> std::io::Result<()> {

    let mut path = env::current_dir()?;
    path.push(PathBuf::from(".git/HEAD"));
    let mut file_head = File::open(path)?;
    let mut referece = String::new();
    file_head.read_to_string(&mut referece)?;

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();  

    // HEADにブランチ名が入っている場合は、ブランチの中身のハッシュ値がHEADのさすハッシュ値になる
    if prefix_path[1].contains("/") {
        // branchのパスは、".git/refs/heads/<branch-name>"
        let mut branch_path = env::current_dir()?;
        branch_path.push(PathBuf::from(".git"));
        branch_path.push(prefix_path[1].replace("\n", ""));


        let mut file_branch = File::create(branch_path)?;
        file_branch.write_all(commit_hash.as_bytes())?;
        file_branch.flush()?;
    }

    // 直接ハッシュ値が格納されていたHEADに直接書き込み
    let head_content = format!("refs: {}", commit_hash);
    file_head.write_all(&head_content.as_bytes())?;
    file_head.flush()?;

    Ok(())
}

#[allow(dead_code)]
/// `commit`ハッシュから`tree`ハッシュを読み出す
fn cat_commit_tree(commit_hash: &str) -> std::io::Result<String> {
    // hash値の2文字までがdirectory pathで、38文字がfile path
    let (dir, file) = commit_hash.split_at(2);

    // オブジェクトまでのパスを習得する
    let mut current_path = env::current_dir()?;
    current_path.push(".git/objects");
    current_path.push(dir);
    current_path.push(file);
    let object_path = current_path;

    // オブジェクトを開いてバイナリで読み込む
    let mut compressed = Vec::new(); // 読み込み用のバッファはbyteのベクタで用意
    let mut file = File::open(object_path)?;
    file.read_to_end(&mut compressed)?;

    // 読み込んだ圧縮データを伸長する
    let mut object_content:Vec<u8> = Vec::new(); // 伸長用のバッファはbyteのベクタで用意
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    decoder.read_to_end(&mut object_content)?;

    // ヌルバイトでheaderとcontentを分離する。
    let mut contents = object_content.splitn(2, |&x| x == b'\0');

    let _header = contents.next().unwrap();

    // Rustっぽい書き方してみた
    let tree = contents.next().and_then(|c| {
        c.split(|&x| x == b'\n')
            .filter(|x| !x.is_empty())
            .map(|x| String::from_utf8_lossy(x).to_string())
            .find_map(|x| x.split_whitespace().nth(1).map(|x| x.to_string()))
    });

    Ok(tree.unwrap())
}

#[allow(dead_code)]
fn add(file_path: &str) -> std::io::Result<()> {
    write_blob(file_path)?;
    update_index(file_path)?;

    Ok(())
}

#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!("Nothing to commit")
    };

    Ok(())
}
