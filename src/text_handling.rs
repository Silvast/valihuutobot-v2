use regex::Regex;

pub struct Content {
    pub content: String,
}

impl Content {
    pub fn new(content: &str) -> Content {
        Content {
            content: content.to_string(),
        }
    }
}

fn find_text_between_brackets(input: &str) -> Vec<String> {
    let re = Regex::new(r"\[([^\[\]]+)\]").unwrap();
    let mut results = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(matched) = cap.get(1) {
            results.push(matched.as_str().to_string());
        }
    }
    results
}

pub fn get_shouts(content: &Content) -> Vec<String> {
    let mut shouts = Vec::new();
    let rows: Vec<&str> = content.content.split("\n").collect();
    for row in rows {
        let row_data: Vec<&str> = row.split("\t").collect();
        if row_data.len() > 1 {
            let shouts_in_row = find_text_between_brackets(row_data[1]);
            for shout in shouts_in_row {
                shouts.push(shout);
            }
        }
    }
    shouts
}


// take vector row_data from memo and add to memo.content
// for each 2nd element in row_data parse text between [] and return each shout as a element in vector
