use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod binary_tree;
use binary_tree::{TreeNode, BST};

fn main() {
    let bst = init_tree();
    let mut route_vec: Vec<&TreeNode> = Vec::new();
    route_vec.push(bst.get_root());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut route_vec);
    }
}

fn init_tree() -> BST {
    let mut bst = BST::new(' ');
    let mut cur_node = bst.get_root_mut();

    for instruction in fs::read_to_string("src/init.txt").unwrap().lines() {
        if let Some((command, arg)) = instruction.split_once(" ") {
            match command {
                "init" => {
                    bst = BST::new(str_to_char(arg));
                    cur_node = bst.get_root_mut();
                },
                "left" => cur_node.add_left(str_to_char(arg)),
                "right" => cur_node.add_right(str_to_char(arg)),
                "go" => {
                    match arg {
                        "left" => cur_node = cur_node.get_left_mut(),
                        "right" => cur_node = cur_node.get_right_mut(),
                        _ => (),
                    }
                },
                "restart" => cur_node = bst.get_root_mut(),
                _ => (),
            }
        }
    }
    
    bst
}

fn handle_connection(mut stream: TcpStream, route_vec: &mut Vec<&TreeNode>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let contents;

    if request_line == "GET /favicon.ico HTTP/1.1" {
        return;
    }

    if request_line == "GET / HTTP/1.1" {
        contents = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>URI Maze</title>
            </head>
            <body>
                <h1>Hello! Try to get to the '!'</h1>
                <a href="Y"><button>Y</button></a>
            </body>
            </html>
        "#.to_string();
    } else {
        let path = request_line.split_whitespace().nth(1).unwrap();
        let last = path.chars().last().unwrap();
        let route = get_route(route_vec);

        if path.len() < route.len() {
            route_vec.pop();
        } else if path.len() > route.len() {
            match route_vec.last().unwrap().get_left() {
                Some(left) => {
                    if last == left.get_value() {
                        route_vec.push(route_vec.last().unwrap().get_left().unwrap());
                    } else {
                        route_vec.push(route_vec.last().unwrap().get_right().unwrap());
                    }                    
                },
                None => ()
            }            
        }
        let route = get_route(route_vec);
        let cur_node = route_vec.last().unwrap();

        match cur_node.get_left() {
            Some(left_node) => {
                let left_val = left_node.get_value();
                let mut left_option = route.clone();
                left_option.push(left_val);
                match cur_node.get_right() {
                    Some(node) => {
                        let right_val = node.get_value();
                        let mut right_option = route.clone();
                        right_option.push(right_val);
                        contents = format!(r#"
                            <!DOCTYPE html>
                            <html lang="en">
                            <head>
                                <meta charset="utf-8">
                                <title>URI Maze</title>
                            </head>
                            <body>
                                <h1>{message}</h1>
                                <a href="{back}"><button>Go Back!</button></a>
                                <a href="{left_option}"><button>{left_val}</button></a>
                                <a href="{right_option}"><button>{right_val}</button></a>
                            </body>
                            </html>
                        "#, message = &route[1..route.len()], back = &route[..route.len() - 1]);
                    }
                    None => {
                        contents = format!(r#"
                            <!DOCTYPE html>
                            <html lang="en">
                            <head>
                                <meta charset="utf-8">
                                <title>URI Maze</title>
                            </head>
                            <body>
                                <h1>{message}</h1>
                                <a href="{back}"><button>Go Back!</button></a>
                                <a href="{left_option}"><button>{left_val}</button></a>
                            </body>
                            </html>
                        "#, message = &route[1..route.len()], back = &route[..route.len() - 1]);
                    }
                }                
            },
            None => {
                if route.chars().last().unwrap() == '!' {
                    route_vec.truncate(1);
                    contents = format!(r#"
                    <!DOCTYPE html>
                        <html lang="en">
                        <head>
                            <meta charset="utf-8">
                            <title>URI Maze</title>
                        </head>
                        <body>
                            <h1>{message} (You Win!)</h1>
                            <a href="/"><button>Go Back!</button></a>
                        </body>
                        </html>
                    "#, message = &route[1..route.len()]);
                } else {
                    contents = format!(r#"
                    <!DOCTYPE html>
                        <html lang="en">
                        <head>
                            <meta charset="utf-8">
                            <title>URI Maze</title>
                        </head>
                        <body>
                            <h1>{message} (Dead end!)</h1>
                            <a href="{back}"><button>Go Back!</button></a>
                        </body>
                        </html>
                    "#, message = &route[1..route.len()], back = &route[..route.len() - 1]);
                }
            }
        }
    }

    let length = contents.len();
    let status_line = "HTTP/1.1 200 OK";

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_route(route_vec: &Vec<&TreeNode>) -> String {
    let mut route = String::from("/");
    for node in route_vec {
        route.push(node.get_value());
    }

    route
}

fn str_to_char(arg: &str) -> char {
    match arg.chars().next() {
        Some(str) => str,
        None => ' ',
    }
}