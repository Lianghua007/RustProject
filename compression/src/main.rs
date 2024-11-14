extern crate flate2;    

use flate2::write::GzEncoder;   // 用于Gzip压缩的编码器
use flate2::Compression;    //  指定压缩级别
use std::env::args;         // 用于获取命令行参数
use std::fs::File;          // 用于文件读写操作
use std::io::copy;          //  用于流数据的复制
use std::io::BufReader;     // 用于缓冲区文件的读取，增加读取效率
use std::time::Instant;     // 获取当前时间，计算程序运行时间

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source len : {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target len : {:?}", output.metadata().unwrap().len());

    println!("Elapsed: {:?}", start.elapsed());
}   
