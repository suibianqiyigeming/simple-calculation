use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("请提供加密货币d的价格作为参数，用空格分隔");
    }

    let mut prices: Vec<f64> = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.parse::<f64>() {
            Ok(price) => prices.push(price),
            Err(_) => {
                eprintln!("无法解析参数'{}'为有效d的价格",arg);
                return;
            }
        }
    }

    if prices.len() == 0 {
        eprintln!("没有有效的价格数据");
        return;
    }

    let sum = prices.iter().sum::<f64>();
    let avg = sum / prices.len() as f64;
    let max = prices.iter().cloned().fold(
        f64::NEG_INFINITY,
        |acc,x| if x > acc { x } else { acc },
    );
    let min = prices.iter().cloned().fold(
        f64::INFINITY,
        |acc,x| if x < acc { x } else { acc },
    );
    println!("价格平均值:{:.2}",avg);
    println!("价格最大值:{:.2}",max);
    println!("价格最小值:{:.2}",min);
}
