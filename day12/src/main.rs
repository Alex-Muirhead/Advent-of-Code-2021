use std::collections::HashMap;

fn search<'a>(path: &mut Vec<&'a str>, graph: &HashMap<&str, Vec<&'a str>>, small: bool) -> i32{
    let last = path.last().unwrap();
    let neighbours: Vec<&str> = graph.get(last).unwrap().clone();

    let mut count = 0;

    // Call recursive depth search
    for neighbour in neighbours {
        let mut is_small = small;
        if neighbour == "end" {
            // println!("Found complete path: {},end", path.join(","));
            count += 1;
            continue;
        }

        if neighbour == "start" {
            // No point checking back to start
            continue;
        }

        if neighbour.chars().nth(0).unwrap().is_lowercase() & path.contains(&neighbour) {
            // Can't go through small caves twice
            if is_small {
                continue;
            }
            is_small = true;
        }

        path.push(neighbour);
        count += search(path, graph, is_small);
        path.pop();
    }

    count
}

fn main() {
    let lines = include_str!("../caves.txt").lines();

    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    // Build graph connections with hashmap (bi-directional graph)
    for line in lines {
        let (left, right) = line.split_once('-').unwrap();

        if !connections.contains_key(left) {
            connections.insert(left, Vec::new());
        }
        if !connections.contains_key(right) {
            connections.insert(right, Vec::new());
        }

        connections.get_mut(left).unwrap().push(right);
        connections.get_mut(right).unwrap().push(left);
    }

    let mut path = vec!["start"];
    let count = search(&mut path, &connections, false);
    println!("Found {} paths", count);
}
