// This is needed because the compiler emits a bogus
// uncreachable_code warning at the StructOpt derive.
#![allow(unreachable_code)]

use structopt::StructOpt;
use structopt_derive::StructOpt;
use std::cmp::Ordering;

#[derive(StructOpt, Debug)]
#[structopt(name = "sfc", about = "React Scaffold Component.")]
struct Opt {

    #[structopt(long = "--styled")]
    styled: bool,

    #[structopt(long = "--routed")]
    routed: bool,

    #[structopt(long = "--feather")]
    feather: bool,

    #[structopt(long = "--children")]
    children: bool,

    #[structopt(long = "--name")]
    name: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", generate_template(&opt));
}

fn generate_template(opt: &Opt) -> String {
    let mut buf = String::new();
    tpl_gen_imports(&opt, &mut buf);
    tpl_gen_component(&opt, &mut buf);
    tpl_gen_export(&opt, &mut buf);
    format_jsx(&mut buf)
}

fn tpl_gen_imports(opt: &Opt, buf: &mut String) {
    let im_react = "import React from 'react';\n";
    let im_styled = "import styled from 'styled-components';\n";
    let im_feather = "import * as feather from 'react-feather';\n";
    let im_routed = "import {\n  Link,\n  Switch,\n  useRouteMatch,\n} from 'react-router-dom';\n";
    buf.push_str(&im_react);
    if opt.styled { buf.push_str(&im_styled); }
    if opt.feather { buf.push_str(&im_feather); }
    if opt.routed { buf.push_str(&im_routed); }
    buf.push('\n');
}

fn tpl_gen_component(opt: &Opt, buf: &mut String) {
    let sig = format!(
        "const {name} = ({{{params}}}) => {{
          {body}
        }};\n\n",
        name = opt.name.as_ref().unwrap_or(&"MyComponent".to_string()),
        params = tplh_gen_component_args(&opt),
        body = tplh_gen_component_body(&opt),
    );
    buf.push_str(&sig);
}

fn tpl_gen_export(opt: &Opt, buf: &mut String) {
    buf.push_str(&format!("export default {var};", var = tplh_gen_export_var(&opt)));
}

fn tplh_gen_component_args(opt: &Opt) -> String {
    let mut args: Vec<&str> = Vec::new();
    if opt.styled { args.push("className") };
    if opt.children { args.push("children") };
    args.join(", ")
}

fn tplh_gen_component_body(opt: &Opt) -> String {
    let mut lines: Vec<&str> = Vec::new();
    if opt.routed {
        lines.push("const {path, url} = useRouteMatch();");
    }
    let body = tplh_gen_component_body_jsx(&opt);
    lines.push(&body);
    lines.join("\n")
}

fn tplh_gen_component_body_jsx(opt: &Opt) -> String {
    let mut buf = String::new();
    buf.push_str(&format!(
        "return (\n<div{args}>\n{extend_children}</div>\n)",
        args = tplh_gen_component_body_jsx_element_args(&opt),
        extend_children = if opt.children { "{children}\n" } else { "" },
    ));
    buf.lines().map(|s| format!("{}", s)).collect::<Vec<String>>().join("\n")
}

fn tplh_gen_component_body_jsx_element_args(opt: &Opt) -> String {
    let mut args: Vec<&str> = Vec::new();
    if opt.styled {
        args.push("className={className}");
    }
    if args.is_empty() {
        String::with_capacity(0)
    } else {
        format!(" {}", args.join(" "))
    }
}

fn tplh_gen_export_var(opt: &Opt) -> String {
    let mut buf = String::new();

    // Opening
    if opt.styled {
        buf.push_str("styled(");
    }

    // Middle
    buf.push_str(opt.name.as_ref().unwrap_or(&"MyComponent".to_string()));
    
    // Closing
    if opt.styled {
        buf.push_str(")``");
    }
    buf
}

fn format_jsx(buf: &mut String) -> String {
    let indent_step = 2;
    let mut indent = 0;
    let mut stack: Vec::<char> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let mut jsx_tag_opening = false;
    let mut jsx_tag_closed = false;
    for line in buf.lines() {
        // println!("------ [indent={}]", indent / indent_step);
        let mut additions = 0_i32;
        let mut deletions = 0_i32;
        let line = line.trim();
        let iter = line.chars();
        for chr in iter {
            match chr {
                '(' => {
                    stack.push(')');
                    additions += 1;
                    jsx_tag_opening = false;
                }
                '{' => {
                    stack.push('}');
                    additions += 1;
                    jsx_tag_opening = false;
                }
                '<' => {
                    jsx_tag_opening = true;
                    additions += 1;
                },
                '/' => {
                    if jsx_tag_opening {
                        additions -= 1;
                        jsx_tag_opening = false;
                        jsx_tag_closed = true;
                    }
                }
                '>' => {
                    if jsx_tag_closed {
                        deletions += 1;
                    }
                    jsx_tag_opening = false;
                }
                ')' | '}' => {
                    if let Some(top_chr) = stack.last() {
                        if chr == *top_chr {
                            stack.pop().unwrap(); // safe
                            deletions += 1;
                        }
                    }
                    jsx_tag_opening = false;
                }
                _ => ()
            }
            // println!("'{}' => [add={};del={};opening={};closed={}]", chr, additions, deletions, jsx_tag_opening, jsx_tag_closed);
        }
        let sign = match additions.cmp(&deletions) {
            Ordering::Equal => 0_i32,
            Ordering::Greater => 1_i32,
            Ordering::Less => -1_i32,
        };
        let delta = indent_step * sign;
        if delta < 0 {
            indent += delta;
            lines.push(format!("{}{}", " ".repeat(indent as usize), line));
        } else {
            lines.push(format!("{}{}", " ".repeat(indent as usize), line));
            indent += delta;
        }
    }
    lines.join("\n")
}