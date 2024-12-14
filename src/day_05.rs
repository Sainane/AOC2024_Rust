use std::cell::{RefCell};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

pub fn day_05() {
    println!("Day 05: ");
    let content = fs::read_to_string("src/input05").expect("Error reading the file");

    let rule_parser = regex::Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rule_parser.captures_iter(&content) {
        let key = rule[1].parse::<i32>().unwrap();
        let value = rule[2].parse::<i32>().unwrap();
        rules_map.entry(key).or_insert_with(Vec::new).push(value);
    }

    let mut prints: Vec<Print> = Vec::new();
    let mut second_part = false;

    for line in content.lines() {
        if line.trim().is_empty() {
            second_part = true;
            continue;
        }
        if second_part {
            let pages: Vec<Rc<RefCell<Page>>> = line.trim()
                .split(',')
                .map(|x| {
                    Rc::new(RefCell::new(Page {
                        pred_nb: 0,
                        preds: Vec::new(),
                        nb: x.parse::<i32>().unwrap()
                    }))
                })
                .collect();
            prints.push(Print { page: pages });
        }
    }


    println!("	- Part 1 : {}", part01(&prints, &rules_map));
    println!("	- Part 2 : {}", part02(prints, rules_map));

}

fn part01(prints : &Vec<Print>, rules_map : &HashMap<i32, Vec<i32>>) -> i32 {

    let mut result = 0;

    for (i,  p) in prints.iter().enumerate() {
        clear_print(&p);
        init_prints(&p, rules_map.clone());
        let mut validity = true;

        for i in 0..p.page.len() {
            if p.page[i].borrow().pred_nb != 0 {
                validity = false;
                break;
            } else {
                update_pred_nb(p, i);
            }
        }

        if validity {
            result += p.page[p.page.len()/2].borrow().nb;
        }
    }
    result

}

fn init_prints(mut p: &Print, rules_map: HashMap<i32, Vec<i32>>) {
    for i in 0..p.page.len() {
        for j in 0..p.page.len() {
            if i == j {
                continue;
            }
            if rules_map.contains_key(&p.page[i].borrow().nb) {
                if rules_map.get(&p.page[i].borrow().nb).unwrap().contains(&p.page[j].borrow().nb) {
                    p.page[j].borrow_mut().pred_nb += 1;
                    p.page[i].borrow_mut().preds.push(Rc::clone(&p.page[j]));
                }
            }
        }
    }
}
fn clear_print(mut p: &Print) {
    for page in &p.page {
        let mut page_ref = page.borrow_mut();
        page_ref.pred_nb = 0;
        page_ref.preds.clear();
    }
}
fn part02(prints: Vec<Print>, rules_map: HashMap<i32, Vec<i32>>) -> i32 {
    let mut result = 0;
    for print in prints {
        if !is_valid(&print, rules_map.clone()) {
            result += order_print(print, rules_map.clone());
        }
    }
    result
}


fn is_valid(p: &Print, rules_map: HashMap<i32, Vec<i32>>) -> bool {

    for page in &p.page {
        let mut page_ref = page.borrow_mut();
        page_ref.pred_nb = 0;
        page_ref.preds.clear();
    }

        build_dependencies(p, rules_map.clone());

        for i in 0..p.page.len() {
            if p.page[i].borrow().pred_nb != 0 {
                return false;
            } else {
              update_pred_nb(p, i);
            }
        }


    true
}

fn update_pred_nb(p: &Print, i: usize) {
    for j in 0..p.page[i].borrow().preds.len() {
        p.page[i].borrow().preds[j].borrow_mut().pred_nb -= 1;
    }
}
fn order_print(p: Print, rules_map: HashMap<i32, Vec<i32>>) -> i32 {

    clear_print(&p);
    build_dependencies(&p, rules_map.clone());

    let mut order: Vec<i32> = Vec::new();
    let mut visited = vec![false; p.page.len()];

    while order.len() < p.page.len() {
        for i in 0..p.page.len() {
            if !visited[i] && p.page[i].borrow().pred_nb == 0 {
                visited[i] = true;
                order.push(p.page[i].borrow().nb);

                for pred in &p.page[i].borrow().preds {
                    pred.borrow_mut().pred_nb -= 1;
                }
                break;
            }
        }
    }

    order[order.len() / 2]
}

fn build_dependencies(p : &Print, rules_map : HashMap<i32, Vec<i32>>) {
    for i in 0..p.page.len() {
        for j in 0..p.page.len() {
            if i == j {
                continue;
            }
            let nb_i = p.page[i].borrow().nb;
            let nb_j = p.page[j].borrow().nb;

            if let Some(preds) = rules_map.get(&nb_i) {
                if preds.contains(&nb_j) {
                    p.page[j].borrow_mut().pred_nb += 1;
                    p.page[i].borrow_mut().preds.push(Rc::clone(&p.page[j]));
                }
            }
        }
    }
}
#[derive(Debug)]
struct Print {
    page: Vec<Rc<RefCell<Page>>>
}

#[derive(Debug)]
struct Page {
    pred_nb: i32,
    nb: i32,
    preds: Vec<Rc<RefCell<Page>>>
}
