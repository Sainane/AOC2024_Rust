use std::fs;

#[derive(Clone, Copy, Debug)]
struct File {
    id : i64,
    nb_blocks : i64
}

pub fn day_09() {
    println!("Day 09");
    let content_ = fs::read_to_string("src/input09").expect("Error reading file");
    let mut data: Vec<i64> = Vec::new();
    let mut data2 : Vec<File> = Vec::new();
    let mut id_ = 0;

    let chars: Vec<char> = content_.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let file_block: i64 = chars[i].to_digit(10).unwrap() as i64;
        data2.push(File{id: id_, nb_blocks: file_block});
        for _ in 0..file_block {
            data.push(id_);
        }
        let empty_block : i64;
        if i + 1  < chars.len(){
            empty_block = chars[i+1].to_digit(10).unwrap() as i64;
            for _ in 0..empty_block {
                data.push(-1);
            }

            data2.push(File{id: -1, nb_blocks: empty_block})
        }


        id_ += 1;
        i += 2;
    }


    println!("\t- Part 1 : {}", part01(&data));
    println!("\t- Part 2 : {}", part02(&data2));
}

fn part01(data_: &Vec<i64>) -> i64 {
    let mut data = data_.clone();
    let data_len = data.len();
    for i in 0..data_len/2 {
        let id = data[data_len - i - 1];
        if id != -1 {
            if let Some(index) = data.iter().position(|&x| x == -1) {
                if index >= data_len -i -1 {
                    break
                }
                data[index] = id;
                data[data_len - i - 1] = -1;
            } else {
                break;
            }
        }
    }

    checksum(&data)
}

fn checksum(data: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for (i, &value) in data.iter().enumerate() {
        if value != -1 {
            sum += value * i as i64;
        }
    }
    sum
}

fn checksum2(data: &Vec<File>) -> i64 {
    let mut sum = 0;
    let mut index = 0;
    for (_, &value) in data.iter().enumerate() {

        if value.id != -1 {
            for j in 0..value.nb_blocks {
                sum += value.id * (index + j) ;
            }
        }

        index += value.nb_blocks
    }
    sum
}

fn part02(data_: &Vec<File>) -> i64 {
    let mut data = data_.clone();
    for mut i in (0..data.len()).rev() {
        let id = data[i].id;

            if id != -1 {
                if let Some(index) = data.iter().position(|&x| x.id == -1 && x.nb_blocks >= data[i].nb_blocks) {

                    if index < i {
                        if data[index].nb_blocks > data[i].nb_blocks {
                            data.insert(index+1, File { id: -1, nb_blocks: data[index].nb_blocks - data[i].nb_blocks });
                            i += 1
                        }
                        data[index] = data[i];


                        data[i].id = -1;


                    }
                }
            }


        if id == 0 {
            break
        }
    }
    checksum2(&data)



}