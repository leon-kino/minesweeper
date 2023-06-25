use std::usize;

use proconio::input;
use rand::Rng;

/// <summary>メインの関数</summary>
fn main() {
    println!("マスのサイズは？");
    input! {
        space:usize,
    }
    println!("ボムの数は？");
    input! {
        bom:u8,
    }
    minesweeper(space, bom, input_point())
}

/// <summary>ユーザーの指定した場所をPoint構造体に変化させる</summary>
/// <return Point>ユーザーの入力した場所が入ったPoint構造体</return>
fn input_point() -> Point {
    println!("どこを開けますか？");
    input! {
        n1: usize,
        n2: usize,
    }
    Point {
        row: n1,
        column: n2,
    }
}

/// <summary>実際の処理</summary>
/// <param space>マスのサイズ</param>
/// <param bom>ボムの配置数</param>
/// <param p>1番最初にユーザーから入力された場所</param>
/// TODO: displayから重複をなくす
fn minesweeper(space: usize, bom: u8, p: Point) {
    // map作成
    let map = create_map(space, bom);
    let mut p = p;

    // ユーザーから指定された場所が空白になるまで作り直す
    if map[p.row][p.column] != '0' {
        return minesweeper(space, bom, p);
    }

    // 表示するマスの場所
    let mut display: Vec<Point> = Vec::new();

    // ゲームが終了するまで繰り返す
    loop {
        let stack = vec![p];
        // ユーザーに指定されたマスを表示する
        display.push(Point {
            row: stack[0].row,
            column: stack[0].column,
        });

        // ユーザーに指定されたマスが '0' の場合、周りのマスも開く
        if map[stack[0].row][stack[0].column] == '0' {
            display = search_map(&map, stack, display);
        }

        // 全てのマスが開いたら終了
        if display.len() == space * space - bom as usize {
            println!("クリア！！");
            show_all(map, space);
            return;
        }

        // 現状の表示
        show_map(&map, space, &display);

        // 次の入力を受け付ける
        p = input_point();

        // 爆弾を踏んだ場合は終了する
        if map[p.row][p.column] == '*' {
            println!("失敗…");
            show_all(map, space);
            return;
        }
    }
}

/// <summary>ユーザーから指定されたサイズのマスと、爆弾を配置する</summary>
/// <param space>マスのサイズ</param>
/// <param bom>爆弾の量</param>
/// <return>マスの情報</return>
fn create_map(space: usize, bom: u8) -> Vec<Vec<char>> {
    let mut map = vec![vec![' '; space]; space];
    let mut rand = rand::thread_rng();
    let mut random: Vec<usize> = vec![];
    for _ in 0..space * space {
        random.push(rand.gen())
    }

    // ボムの位置をランダムで決める（重複をなくす）
    let mut temp = random.clone();
    temp.sort();
    let threshold = temp[bom as usize];
    for index in 0..random.len() {
        if random[index] < threshold {
            map[index / space][index % space] = '*';
        }
    }

    for i in 0..space {
        for j in 0..space {
            if map[i][j] != '*' {
                let mut cnt: u8 = 0;
                for k in (i as isize - 1)..=(i as isize + 1) {
                    for l in (j as isize - 1)..=(j as isize + 1) {
                        if k < 0 || k == space as isize || l < 0 || l == space as isize {
                            continue;
                        }
                        if map[k as usize][l as usize] == '*' {
                            cnt += 1;
                        }
                    }
                }
                map[i][j] = (cnt + 48) as char;
            }
        }
    }
    return map;
}

/// <summary>全てのマスを表示する</summary>
/// <param map>マップの情報</param>
/// <param space>マスのサイズ</param>
fn show_all(map: Vec<Vec<char>>, space: usize) {
    // タイトル
    print!("    ");
    for i in 0..space {
        print!("{:>2}  ", i);
    }
    println!();
    print!("   ");
    for _ in 0..space {
        print!("----");
    }
    println!("-");

    // map
    for i in 0..space {
        print!("{:>2} |", i);
        for j in 0..space {
            let output_char;
            if map[i][j] == '0' {
                output_char = ' ';
            } else {
                output_char = map[i][j];
            }
            print!(" {}", output_char);
            print!(" |");
        }
        println!();
        print!("   ");
        for _ in 0..space {
            print!("----");
        }
        println!("-");
    }
}

/// <summary>選択されたマップから、公開できるマスを探し出す</summary>
/// <param map>マップの情報</param>
/// <param s>残りのタスク（スタック）</param>
/// <param d>表示するマスの場所</param>
/// <return>表示するマスの情報</return>
fn search_map(map: &Vec<Vec<char>>, s: Vec<Point>, d: Vec<Point>) -> Vec<Point> {
    let mut stack = s;
    let mut display = d;
    // iがstackの数よりも多い場合は終了
    if stack.is_empty() {
        return display;
    }
    let space = map.len();
    let p = &stack.pop().unwrap();
    let mut temp: Vec<Point> = vec![];

    for k in (p.row as isize - 1)..=(p.row as isize + 1) {
        for l in (p.column as isize - 1)..=(p.column as isize + 1) {
            if k < 0 || k >= space as isize || l < 0 || l >= space as isize {
                continue;
            }
            if !display
                .iter()
                .any(|x| x.row == k as usize && x.column == l as usize)
            {
                if map[k as usize][l as usize] == '0' {
                    temp.push(Point {
                        row: k as usize,
                        column: l as usize,
                    });
                }
                display.push(Point {
                    row: k as usize,
                    column: l as usize,
                });
            }
        }
    }

    stack.append(&mut temp);
    return search_map(map, stack, display);
}

/// <summary>指定された場所を表示する</summary>
/// <param map>マップの情報</param>
/// <param space>マップのサイズ</param>
/// <param stack>表示するマスの場所</param>
fn show_map(map: &Vec<Vec<char>>, space: usize, stack: &Vec<Point>) {
    // タイトル
    print!("    ");
    for i in 0..space {
        print!("{:>2}  ", i);
    }
    println!();
    print!("   ");
    for _ in 0..space {
        print!("----");
    }
    println!("-");

    // map
    for i in 0..space {
        print!("{:>2} |", i);
        for j in 0..space {
            let p = Point { row: i, column: j };
            let mut output_char = '/';
            if stack.iter().any(|x| x.row == p.row && x.column == p.column) {
                if map[i][j] == '0' {
                    output_char = ' ';
                } else {
                    output_char = map[i][j];
                }
            }
            print!(" {}", output_char);
            print!(" |");
        }
        println!();
        print!("   ");
        for _ in 0..space {
            print!("----");
        }
        println!("-");
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    column: usize,
}
