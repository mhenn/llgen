use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_points(file: String) -> i32 {
    let mut points = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(ip) = line {
                let ip: Vec<&str> = ip.split(' ').into_iter().collect();
                let typ : String =  ip.get(9).unwrap().to_string();
                if let Some(time) = ip.get(16){
                    let mut time = time.to_string();
                time.pop();
                if let Ok(time) = time.parse::<f64>(){
                if typ == "Mounted" && time < 20.0{
                        continue;
                }
                if typ == "Delivered" && time < 80.0{
                    continue;
                }
                }
                }
                points += ip.get(4).unwrap().parse::<i32>().unwrap();
            }
        }
    }
    points
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//#[test]
fn ree(){
    let res =  parse_points("./run_results/g10_i10_d2/output/gen_9_ind_9".to_string());
    println!("{}", res);
    assert!(false)
}
