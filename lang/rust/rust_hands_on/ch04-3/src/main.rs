use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::ErrorKind;
use std::io::Write;
use std::fs::OpenOptions;
use std::fs;

fn _main1() {
    println!("===== テキストファイルの利用 =====");

    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        count += 1;
        let txt = line.unwrap();
        println!("{}: {}", count, txt);
    }
}

fn _main2() {
    println!("===== Fileの取得とエラー処理 =====");

    let file = match File::open("data.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("ファイルが見つかりませんでした"),
            ErrorKind::PermissionDenied => panic!("ファイルへのアクセス権限がありません"),
            _ => panic!("ファイルのオープンに失敗しました: {:?}", error),
        },
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn _main3() {
    println!("===== テキストを書き出す =====");

    let data = [
        "Hello world!",
        "これはサンプルのデータです。",
        "テストテスト!"
    ];

    let str_data = data.join("\n");
    let mut file = File::create("backup.txt").unwrap();
    file.write_all(str_data.as_bytes()).unwrap();
}

fn _main4() {
    println!("===== ファイルにデータを追記する =====");

    let str_data = "This is sample!\n";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("append.txt").unwrap();

    file.write_all(str_data.as_bytes()).unwrap();
}

fn _main5() {
    println!("===== ファイル一覧の取得 =====");

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let entry = path.unwrap();
        println!("{:?}", entry.path().to_str());
    }
}

fn _main6() {
    println!("===== エントリーの種類を調べる =====");

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let entry = path.unwrap();
        let ftype = entry.file_type().unwrap();

        if ftype.is_file() {
            println!("{:?} file", entry.path())
        } else if ftype.is_dir() {
            println!("{:?} dir", entry.path())
        } else if ftype.is_symlink() {
            println!("{:?} link", entry.path())
        } else {
            println!("{:?}", entry.path())
        }
    }
}

fn main() {
    println!("===== ファイル/フォルダの操作 =====");

    _ = fs::create_dir("./backup");
    let entries = fs::read_dir("./").unwrap();

    for path in entries {
        let entry = path.unwrap();
        if entry.file_type().unwrap().is_file() {
            let file_name = entry.file_name();
            let from_name = format!("./{}", file_name.to_string_lossy());
            let to_name = format!("./backup/_{}", file_name.to_string_lossy());

            _ = fs::copy(&from_name, &to_name);
            println!("backup: {:?} -> {:?}", from_name, to_name);
        } else {
            println!("not copied.({:?})", entry.file_name());
        }
    }
}



