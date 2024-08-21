//Copyright © 2024 FuyuGitsune All Rights Reserved.
//https://opensource.org/licenses/mit-license.php
//This software is released under the MIT License, see LICENSE.

use std::io::stdin;
use owo_colors::OwoColorize;

fn main() {
    println!("------某音ゲーの成績を某音ゲー風にするやつ version0.00------");
    println!(" ");
    calc_main();
    loop{
    println!("続けますか？Y/N");
    let ans = input_str();
    if ans == "Y" || ans == "y"{
        calc_main();
    } else if ans == "N" || ans == "n"{
        break;
    }
}
}

fn input_int()->f64{
    loop{
    let mut a = String::new();
    stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    if a < 0.0{
        continue;
    }
    return a
    }
}

fn input_str()->String{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn calc_main(){
    println!("貴方がとったPERFECT[パーフェクト]判定の数を教えてください。");
    let perfect = input_int();
    println!("貴方がとったGREAT[グレート]判定の数を教えてください。");
    let great = input_int();
    println!("貴方がとったGOOD[グッド]判定の数を教えてください。");
    let good = input_int();
    println!("貴方がとったBAD[バッド]判定の数を教えてください。");
    let bad = input_int();
    println!("貴方がとったMISS[ミス]判定の数を教えてください。");
    let miss = input_int();
    println!("最後に、最大コンボ数を教えてください。");
    let combo = input_int();
    println!(" ");
    println!(" ");
    println!(" ");
    println!(" ");

    let all_notes = perfect + great + good + bad + miss;
    if all_notes == 0.0{
        println!("貴方はプレイしていないことになってしまいます！！");
        return;
    }
    let phig_score = ((perfect * 1.0 + great * 0.65) / all_notes * 900000.0 + combo / all_notes * 100000.0).round() as u32;
    let acc = (((perfect * 1.0 + great * 0.65) / all_notes) * 10000.0).round() / 100.0;
    let mut class_alpha = " F ";

    let proscore_acc = (((perfect * 1.0 + great * 0.7 + good * 0.5) / all_notes) * 10000.0).round() / 100.0;
    let girscore_acc = (((perfect * 1.0 + great * 0.73 + good * 0.45) / all_notes) * 10000.0).round() / 100.0;
    let pafescore_acc = ((perfect / all_notes) * 10000.0).round() / 100.0;

    if phig_score >= 700000{
        class_alpha = " C ";
        if phig_score >= 820000{
            class_alpha = " B ";
            if phig_score >= 880000{
                class_alpha = " A ";
                if phig_score >= 920000{
                    class_alpha = " S ";
                    if phig_score >= 960000{
                        class_alpha = " V ";
                    }
                }
            }
        }
    }

    //ここに判定変換。

    println!(" -------Your score-------");
    println!("  perfect    :  {}", perfect as u32);
    println!("  great      :  {}", great as u32);
    println!("  good       :  {}", good as u32);
    println!("  bad        :  {}", bad as u32);
    println!("  miss       :  {}", miss as u32);
    println!(" ");
    println!("  max_combo  :  {}", combo as u32);
    println!(" ");
    if phig_score >= 820000{
        if phig_score >= 900000{
            if phig_score >= 950000{
                if phig_score >= 980000{
                    if phig_score == 1000000{
                        println!("  score      : {}{}{}", " ".black().on_white(), phig_score.black().on_white(), " ".black().on_white());
                    }else{
                        println!("  score      : {}{}{}", " ".white().on_yellow(), phig_score.white().on_yellow(), " ".white().on_yellow());
                    }
                }else{
                    println!("  score      : {}{}{}", " ".white().on_red(), phig_score.white().on_red(), " ".white().on_red());
                }
            }else{
                println!("  score      : {}{}{}", " ".white().on_blue(), phig_score.white().on_blue(), " ".white().on_blue());
            }
        }else{
            println!("  score      : {}{}{}", " ".white().on_green(), phig_score.white().on_green(), " ".white().on_green());
        }
    }else{
        println!("  score      :  {} ", phig_score);
    }
    println!("  acc @grpa  :  {}%", girscore_acc);
    println!("  acc @prsk  :  {}%", proscore_acc);
    println!("  acc @phig  :  {}%", acc);
    println!("  acc @pafe  :  {}%", pafescore_acc);
    if all_notes == perfect + great{
        if all_notes == perfect{
            println!("  evaluation : {}", " φ ".yellow().on_white())
        }else{
            println!("  evaluation : {}", " V ".blue().on_white())
        }
    }else{
        println!("  evaluation : {}", class_alpha);
    }
    println!(" ");
    println!(" ");
    println!(" ");
    println!(" ");
}
