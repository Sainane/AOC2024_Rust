use std::collections::HashMap;
use std::ffi::c_float;
use std::os::raw::c_double;
use log::log;

pub fn day_11() {
    let  cache : &mut HashMap<(i128, i128), i128> = &mut std::collections::HashMap::new();
   let mut values:  Vec<i128> = vec![3279, 998884, 1832781, 517, 8, 18864, 28, 0];
    println!("Day 11:");
    let mut part1 = 0;
      for i in 0..values.len() {
            part1 += nb_rock(25, values[i], cache);
    }
    println!("\t- Part 1: {}", part1);
    let mut part2 = 0;
    for i in 0..values.len() {
        part2 += nb_rock(75, values[i], cache);
    }
    println!("\t- Part 2: {}", part2);
    //println!("Part 1 : {}", values.len());
    //println!("Part 2 : {}", values.len());
}


fn nb_rock(nb_iter : i128, rock_nb : i128, cache:  &mut HashMap<(i128, i128), i128>) -> i128 {
    if cache.contains_key(&(nb_iter, rock_nb)) {
        return *cache.get(&(nb_iter, rock_nb)).unwrap();
    }
   if nb_iter == 0 {
       return 1;
   }
    if rock_nb == 0 {
        let result = nb_rock(nb_iter - 1, 1, cache);
             cache.insert((nb_iter, rock_nb), result);
            result
    } else if (rock_nb.to_string().len() % 2 == 0) {
        let right_val = rock_nb.to_string().split_at(rock_nb.to_string().len()/2).0.parse::<i128>().unwrap();
        let left_val = rock_nb.to_string().split_at(rock_nb.to_string().len()/2).1.parse::<i128>().unwrap();
        let result = nb_rock(nb_iter - 1, right_val, cache) + nb_rock(nb_iter - 1, left_val, cache);
        cache.insert((nb_iter, rock_nb), result);
        return result;
    } else {
        let result = nb_rock(nb_iter - 1, rock_nb * 2024, cache);
        cache.insert((nb_iter, rock_nb), result);
        return result
}

}
