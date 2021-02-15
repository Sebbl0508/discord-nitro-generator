use clap::{load_yaml, App};
use num_format::{Locale, ToFormattedString};
use std::io::BufRead;
use reqwest::Client;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();

    if let Some(matches) = m.subcommand_matches("gen") {
        if let Some(i) = matches.value_of("AMOUNT") {
            let amount = i.parse::<u64>();

            match amount {
                Ok(i) => {
                    generate_codes(i);
                }
                Err(_e) => {
                    println!("The input amount is not a number!");
                }
            }
        }
    } else if let Some(_matches) = m.subcommand_matches("check") {
        #[allow(unused_imports)]
        use std::io::{stdin, stdout, Write};
        let url = String::new();

        // println!("Please input the webhook link!");
        // print!("> ");
        // let _ = stdout().flush();

        // Get webhook URL from user
        // stdin().read_line(&mut url).expect("Incorrect string!");

        // Remove trailing newline
        // url.truncate(url.len() - 1);

        // Fetch range of lines to check
        // TODO: Implement RANGE check
        // if let Some(_i) = matches.value_of("RANGE") {
        //     let beginrange = 1;
        //     let endrange = 10;
        //     println!("Checking lines {}-{}", beginrange, endrange);
        // } else {
        //     println!("No range specified, checking whole file");
        // }

        match check_codes(url) {
            Ok(i) => {
                println!("Code checking succeeded: {:#?}", i);
            },
            Err(e) => {
                println!("Error occured during code checking: '{}'", e);
            }
        }
    } else {
        println!("No subcommands, try -h/--help");
    }
}

// Check codes for validity against discord
fn check_codes(_url: String) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{prelude::*, BufReader};
    let file = std::fs::File::open("./codes.txt").expect("The file 'codes.txt' does not exist!");
    let buf = BufReader::new(file);

    // Proxy stuff
    let proxy_file = std::fs::File::open("./proxys.txt").expect("The 'proxys.txt' file does not exist");
    let proxy_buf = BufReader::new(proxy_file);
    let proxies: Vec<String> = proxy_buf
        .lines()
        .map(|l| l.expect("[ERROR] Could not parse line"))
        .collect();
    let mut curr_proxy = 0;
    // let mut proxy = reqwest::Proxy::https(&proxies[curr_proxy].clone());

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("[ERROR] Could not parse line"))
        .collect();
    let mut client: reqwest::blocking::Client;

    // match proxy {
    //     Ok(i) => {
    //         client = reqwest::blocking::Client::builder()
    //             .proxy(i)
    //             .build()?;
    //     },
    //     Err(_e) => {
    //         client = reqwest::blocking::Client::builder()
    //             .build()?;
    //         println!("[ERROR] no proxies");
    //     }
    // }
    client = reqwest::blocking::Client::builder()
        .build()?;



    for (idx, item) in lines.iter().enumerate() {
        let res = client.get(format!("https://discordapp.com/api/v6/entitlements/gift-codes/{}?with_application=false&with_subscription_plan=true", item).as_str())
            .send();

        // let res = reqwest::blocking::get(format!("https://discordapp.com/api/v6/entitlements/gift-codes/{}?with_application=false&with_subscription_plan=true", item).as_str())?
        //     .text()?;
        let res_json: serde_json::Value;

        match res {
            Ok(i) => {
                match i.text() {
                    Ok(ii) => {
                        res_json = serde_json::from_str(ii.as_str())?;
                        if res_json["code"] != 10038 && res_json["global"] != false {
                            println!("VALID CODE: {}", item);
                        } else {
                            if res_json["message"] == "You are being rate limited." {
                                println!("{}. Rate limit, sleeping for {}", idx + 1, res_json["retry_after"]);

                                let sleep_dur = res_json["retry_after"].as_i64().unwrap_or(50);

                                std::thread::sleep(std::time::Duration::from_millis(sleep_dur as u64));
                                // curr_proxy += 1;
                                // println!("New proxy: {}", proxies[curr_proxy].clone());
                                // proxy = reqwest::Proxy::https(&proxies[curr_proxy].clone());
                                // match proxy {
                                //     Ok(i) => {
                                //         client = reqwest::blocking::Client::builder()
                                //             .proxy(i)
                                //             .build()?;
                                //     },
                                //     Err(e) => {
                                //         println!("{}", e);
                                //     }
                                // }
                            } else {
                                println!("{}. invalid code", idx + 1);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            },
            Err(e) => {
                println!("[ERROR] {}", e);
                // curr_proxy += 1;
                // println!("New proxy: {}", proxies[curr_proxy].clone());
                // proxy = reqwest::Proxy::https(&proxies[curr_proxy].clone());
                // match proxy {
                //     Ok(i) => {
                //         client = reqwest::blocking::Client::builder()
                //             .proxy(i)
                //             .build()?;
                //     },
                //     Err(e) => {
                //         println!("{}", e);
                //     }
                // }
            }
        }

//        let res_json: serde_json::Value = serde_json::from_str(res.as_str())?;
//         if res_json["code"] != 10038 && res_json["global"] != false {
//             println!("VALID CODE: {}", item);
//         } else {
//             if res_json["message"] == "You are being rate limited." {
//                 println!("{}. Rate limit, retry after {}", i + 1, res_json["retry_after"]);
//             } else {
//                 curr_proxy += 1;
//                 proxy = reqwest::Proxy::http(&proxies[curr_proxy].clone());
//                 match proxy {
//                     Ok(i) => {
//                         client = reqwest::blocking::Client::builder()
//                             .proxy(i)
//                             .build()?;
//                     },
//                     Err(e) => {
//                         println!("{}", e);
//                     }
//                 }
//                 println!("{}. invalid code", i + 1);
//             }
//         }
    }

    Ok(())
}

fn generate_codes(amount: u64) -> String {
    println!(
        "Generating {} codes",
        amount.to_formatted_string(&Locale::de)
    );

    let mut file = std::fs::File::create("codes.txt")
        .expect("Creating 'codes.txt' failed, maybe the file is already there?");

    let start = std::time::Instant::now();
    let mut big_string = String::new();

    use rand::prelude::*;
    use std::io::Write;

    for i in 0..amount {
        let randchar: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        // Append random code together with url & newline to BIGSTRING
        //big_string.push_str(prefix.as_str());
        big_string.push_str(randchar.as_str());
        big_string.push_str("\n");

        if i % 100_000 == 0 {
            file.write_all(big_string.as_bytes())
                .expect("Could'nt write to file");
            big_string = String::from("");
        }
    }
    file.write_all(big_string.as_bytes())
        .expect("Couldn't write to file");

    let generating_time = start.elapsed();

    println!(
        "Generated {} codes | Elapsed time: generating & saving: {:#?}",
        amount.to_formatted_string(&Locale::de),
        generating_time,
    );

    return String::from("");
}
