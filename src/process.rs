use crate::filter;
use crate::log;
use crate::log_settings::LogSettings;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde_json::{Map, Value};
use std::io::Write;
use std::io::{self, BufRead};
use yansi::{Color, Paint};

lazy_static! {
	static ref ORANGE: Color = Color::Rgb(255, 135, 22);
}

pub fn process_input(
	log_settings: &LogSettings,
	input: &mut dyn io::BufRead,
	maybe_filter: Option<&String>,
	implicit_return: bool,
	handlebars: &Handlebars<'static>,
) {
	for line in input.lines() {
		match line {
			Ok(raw_line) => {
				let result = process_input_line(log_settings, &raw_line, None, maybe_filter, implicit_return, handlebars);
				if result.is_err() {
					emit_raw_line(&raw_line, &ORANGE);
				}
			}
			Err(err) => {
				emit_raw_line(&format!("Could not read line: {err}"), &Color::Red);
			}
		}
	}
}

fn emit_raw_line(line: &str, color: &Color) {
	let write_result = writeln!(&mut io::stdout(), "{} {}", "??? >".fg(*color).bold(), line);
	if write_result.is_err() {
		std::process::exit(14);
	}
}

fn process_input_line(
	log_settings: &LogSettings,
	raw_line: &str,
	prefix: Option<&str>,
	maybe_filter: Option<&String>,
	implicit_return: bool,
	handlebars: &Handlebars<'static>,
) -> Result<(), ()> {
	match serde_json::from_str::<Value>(raw_line) {
		Ok(Value::Object(log_entry)) => {
			handle_json_entry(log_settings, prefix, &log_entry, maybe_filter, implicit_return, handlebars);
			Ok(())
		}
		_ => {
			if log_settings.with_prefix && prefix.is_none() {
				if let Some(brace_pos) = raw_line.find('{') {
					let line_prefix = &raw_line[..brace_pos];
					let json_part = &raw_line[brace_pos..];
					process_input_line(log_settings, json_part, Some(line_prefix), maybe_filter, implicit_return, handlebars)
				} else {
					Err(())
				}
			} else {
				Err(())
			}
		}
	}
}

fn handle_json_entry(
	log_settings: &LogSettings,
	prefix: Option<&str>,
	log_entry: &Map<String, Value>,
	maybe_filter: Option<&String>,
	implicit_return: bool,
	handlebars: &Handlebars<'static>,
) {
	if let Some(filter_expr) = maybe_filter {
		match filter::show_log_entry(log_entry, filter_expr, implicit_return, log_settings) {
			Ok(true) => render_entry(log_settings, prefix, log_entry, handlebars),
			Ok(false) => (),
			Err(e) => {
				writeln!(io::stderr(), "{}: '{:?}'", "Failed to apply filter expression".red(), e).expect("Should be able to write to stderr");
			}
		}
	} else {
		render_entry(log_settings, prefix, log_entry, handlebars)
	}
}

fn render_entry(log_settings: &LogSettings, prefix: Option<&str>, log_entry: &Map<String, Value>, handlebars: &Handlebars<'static>) {
	log::print_log_line(&mut io::stdout(), prefix, log_entry, log_settings, handlebars)
}
