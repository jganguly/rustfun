use std::process;

#[derive(Debug)]
struct Sentence<'a> {
	part: &'a str
}

pub fn split(sentence: &str) -> Vec<&str> {
    let sent = sentence.split(".");
    let mut vec: Vec<&str> = sent.collect();
	vec
}

