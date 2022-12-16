// dotenvの読み込みが必要かどうか確認する
fn is_required_set_dotenv() -> bool {
    match std::env::var("IS_LOCAL").ok() {
        Some(s) if s.to_uppercase() == "TRUE" => {
            println!(
                "Env 'IS_LOCAL' is '{}'. Continue  set_dotenv.",
                s.to_uppercase()
            );
            true
        }
        Some(s) => {
            println!(
                "Env 'IS_LOCAL' is '{}'. Break set_dotenv.",
                s.to_uppercase()
            );
            false
        }
        None => {
            println!("Env 'IS_LOCAL' is not found. Continue set_dotenv.");
            true
        }
    }
}

/// dotenvファイルを読み込む
/// 読み込み順の関係上、tracingではなくprintln!()を使っている
pub fn set_dotenv(package: &str) {
    // dotenvの読み込みが必要でなければ関数を抜ける
    if !is_required_set_dotenv() {
        return;
    }

    // ルートで実行されない場合に備え、
    // カレントあるいは親ディレクトリからdotenvを探す
    let current_dir = match std::env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(e) => panic!("Fail to get current directory\n{:#?}", e),
    };
    let parent_dir = current_dir.parent().unwrap();
    let current_dir_name = current_dir.file_name().unwrap().to_str().unwrap();
    let parent_dir_name = parent_dir.file_name().unwrap().to_str().unwrap();

    // 指定されたパッケージ名とディレクトリ名を比較し、
    // 合致したディレクトリの配下にdotenvが存在するとみなす
    let dirpath = match package {
        package if package == current_dir_name => current_dir,
        package if package == parent_dir_name => parent_dir.into(),
        _ => {
            println!(
                "Fail to load dotenv file, because not match Package '{}' and Directory '{}'",
                package,
                current_dir.display()
            );
            return;
        }
    };
    let dotenv_path = dirpath.join("dotenv").join(".env");

    // 読み込み対象のpathを表示する
    println!("Load dotenv from: {:#?}", dotenv_path);
    dotenv::from_path(dotenv_path).ok();
}
