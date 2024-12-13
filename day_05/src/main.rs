use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_FIX_ATTEMPTS: u16 = 1000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let (rules, mut updates) = read_input(file_path).expect("Failed to read and parse file");
    let rule_set = RuleSet::new(rules);

    println!(
        "Loaded {} rules and {} updates",
        rule_set.len(),
        updates.len()
    );

    println!("===================== Part 1 =====================");
    let sum_of_medians: u16 = updates
        .iter()
        .map(|update| {
            if rule_set.evaluate_all(&update).is_none() {
                update.median as u16
            } else {
                0
            }
        })
        .sum();

    println!("Sum of medians: {}", sum_of_medians);

    println!("===================== Part 2 =====================");
    let mut incorrect_updates: Vec<&mut UpdateBatch> = updates
        .iter_mut()
        .filter(|update| rule_set.evaluate_all(update).is_some())
        .collect();
    println!("Incorrect updates: {}", incorrect_updates.len());
    incorrect_updates.iter_mut().for_each(|update| {
        if !fix_update_batch(update, &rule_set) {
            panic!("Failed to fix update: {:?}", update.to_vec());
        }
    });

    let fixed_median = incorrect_updates
        .iter()
        .map(|update| update.median as u16)
        .sum::<u16>();
    println!("Fixed median: {}", fixed_median);
}

enum ParseMode {
    Rule = 1,
    UpdateBatch = 2,
}

fn read_input(file_path: &str) -> io::Result<(Vec<PrintingRule>, Vec<UpdateBatch>)> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut parser_mode = ParseMode::Rule;

    let mut rules = vec![];
    let mut update_batches = vec![];

    for line in reader.lines() {
        let line = line?;
        match parser_mode {
            ParseMode::Rule => {
                if line.is_empty() {
                    parser_mode = ParseMode::UpdateBatch;
                } else {
                    rules.push(parse_printing_order_rule(&line));
                }
            }
            ParseMode::UpdateBatch => {
                update_batches.push(parse_update_batch(&line));
            }
        }
    }

    Ok((rules, update_batches))
}

fn parse_update_batch(update_batch_str: &str) -> UpdateBatch {
    let update_batch = update_batch_str
        .split(",")
        .map(|page| page.parse::<u8>().unwrap())
        .collect();
    UpdateBatch::new(update_batch)
}

fn parse_printing_order_rule(rule_str: &str) -> PrintingRule {
    let rule: Vec<u8> = rule_str
        .split("|")
        .map(|page| page.parse::<u8>().unwrap())
        .collect();
    if rule.len() != 2 {
        panic!("Invalid rule format: {}", rule_str);
    }
    PrintingRule::new(rule[0], rule[1])
}

#[derive(Debug)]
struct PrintingRule {
    pub page: u8,
    pub after_page: u8,
}

#[derive(Debug)]
struct RuleSet {
    rules: Vec<PrintingRule>,
}

fn fix_update_batch(update: &mut UpdateBatch, rule_set: &RuleSet) -> bool {
    let mut attempts = 0;
    while let Some(failed_rule_idx) = rule_set.evaluate_all(update) {
        if attempts >= MAX_FIX_ATTEMPTS {
            return false;
        }
        attempts += 1;
        let rule_to_fix = rule_set.get_rule_by_id(failed_rule_idx[0]).unwrap();

        let incorrect_page_idx = update.get_page_index(rule_to_fix.after_page).unwrap();
        let correct_page_idx = update.get_page_index(rule_to_fix.page).unwrap();

        update.set_page_index(rule_to_fix.after_page, correct_page_idx);
        update.set_page_index(rule_to_fix.page, incorrect_page_idx);
    }

    true
}

impl RuleSet {
    fn new(rules: Vec<PrintingRule>) -> RuleSet {
        RuleSet { rules }
    }

    fn evaluate_all(&self, update: &UpdateBatch) -> Option<Vec<usize>> {
        let violated_rules: Vec<usize> = self
            .rules
            .iter()
            .enumerate()
            .map(|(rule_id, rule)| (rule_id, rule.evaluate(update)))
            .filter(|(_rule_id, success)| !success)
            .map(|(rule_id, _)| rule_id)
            .collect();
        if violated_rules.is_empty() {
            None
        } else {
            Some(violated_rules)
        }
    }

    fn len(&self) -> usize {
        self.rules.len()
    }

    fn get_rule_by_id(&self, rule_id: usize) -> Option<&PrintingRule> {
        self.rules.get(rule_id)
    }
}

#[derive(Debug, Clone)]
struct UpdateBatch {
    pub order: HashMap<u8, usize>,
    pub median: u8,
    median_idx: usize,
}

impl UpdateBatch {
    fn new(printing_order: Vec<u8>) -> UpdateBatch {
        let median_idx = printing_order.len() / 2 as usize;
        let median = printing_order[median_idx];
        let order = printing_order
            .iter()
            .enumerate()
            .map(|(idx, &page)| (page, idx))
            .collect();
        UpdateBatch {
            order: order,
            median: median,
            median_idx: median_idx,
        }
    }

    fn get_page_index(&self, page: u8) -> Option<usize> {
        self.order.get(&page).copied()
    }

    fn set_page_index(&mut self, page: u8, idx: usize) {
        self.order.insert(page, idx);
        self.median = self.to_vec()[self.median_idx];
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut res = vec![0; self.order.len()];
        for (page, idx) in self.order.iter() {
            res[*idx] = *page;
        }
        res
    }
}

impl PrintingRule {
    fn new(page: u8, after_page: u8) -> PrintingRule {
        PrintingRule { page, after_page }
    }

    fn evaluate(&self, printing_order: &UpdateBatch) -> bool {
        match (
            printing_order.get_page_index(self.page),
            printing_order.get_page_index(self.after_page),
        ) {
            (Some(page_idx), Some(after_page_idx)) => page_idx < after_page_idx,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_printing_order_rule() {
        let rule = parse_printing_order_rule("1|2");
        assert_eq!(rule.page, 1);
        assert_eq!(rule.after_page, 2);
    }

    #[test]
    fn test_parse_update_batch() {
        let update_batch = parse_update_batch("1,2,3,4");
        assert_eq!(update_batch.order.len(), 4);
        assert_eq!(update_batch.get_page_index(1), Some(0));
        assert_eq!(update_batch.get_page_index(4), Some(3));
    }

    #[test]
    fn test_evaluate_rule() {
        let rule = PrintingRule::new(1, 2);
        let update_batch = UpdateBatch::new(vec![1, 2, 3, 4]);
        assert_eq!(rule.evaluate(&update_batch), true);
    }

    #[test]
    fn test_get_median() {
        let update_batch = UpdateBatch::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(update_batch.median, 3);
    }

    #[test]
    fn test_fix_incorrect_updates() {
        let rule_set = RuleSet::new(vec![
            PrintingRule::new(47, 53),
            PrintingRule::new(97, 13),
            PrintingRule::new(97, 61),
            PrintingRule::new(97, 47),
            PrintingRule::new(75, 29),
            PrintingRule::new(61, 13),
            PrintingRule::new(75, 53),
            PrintingRule::new(29, 13),
            PrintingRule::new(97, 29),
            PrintingRule::new(53, 29),
            PrintingRule::new(61, 53),
            PrintingRule::new(97, 53),
            PrintingRule::new(61, 29),
            PrintingRule::new(47, 13),
            PrintingRule::new(75, 47),
            PrintingRule::new(97, 75),
            PrintingRule::new(47, 61),
            PrintingRule::new(75, 61),
            PrintingRule::new(47, 29),
            PrintingRule::new(75, 13),
            PrintingRule::new(53, 13),
        ]);
        let mut incorrect_update = UpdateBatch::new(vec![75, 97, 47, 61, 53]);
        let expected_fixed_update = UpdateBatch::new(vec![97, 75, 47, 61, 53]);

        assert!(fix_update_batch(&mut incorrect_update, &rule_set));

        assert_eq!(incorrect_update.order, expected_fixed_update.order);
    }
}
