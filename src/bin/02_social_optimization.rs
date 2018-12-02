use std::io::{self, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;


struct Room{
    residents: HashSet<String>,
    unpopular: HashSet<String>,
}

fn add_new(name: &String, rooms: &mut Vec<Room>) -> Result<usize, bool> {
    {
        let mut c = 0usize;
        for r in rooms.iter_mut() {
            if r.residents.contains(name) {
                return Ok(c);
            }
            c += 1;
        }
    }
    {
        let mut new_room = Room {
            residents: HashSet::new(),
            unpopular: HashSet::new()
        };
        new_room.residents.insert(name.clone());
        rooms.push(new_room);
    }
    Ok(rooms.len() - 1)
}

fn add_plus(name: &str, room: usize, rooms: &mut Vec<Room>) -> Result<usize, bool> {
    
    if rooms[room].unpopular.contains(name) {
        return Err(false);
    }

    let mut target = 0usize;
    let mut b = false;
    for c in 0..rooms.len() {
        if c == room {
            continue;
        }
        let r = &rooms[c];
        if r.residents.contains(name) {
            if !r.unpopular.is_disjoint(&rooms[room].residents) {
                return Err(false);
            }
            if !r.residents.is_disjoint(&rooms[room].unpopular) {
                return Err(false);
            }
            b = true;
            target = c;
            break;
        }
    }

    rooms[room].residents.insert(name.to_string());

    if b {
        return Ok(merge(rooms, room, target));
    }

    Ok(room)
}

fn merge(rooms: &mut Vec<Room>, a: usize, b: usize) -> usize {
    assert_ne!(a, b);
    let into = std::cmp::min(a, b);
    let from = std::cmp::max(a, b);

    // ???? iter, struct, into_iter???
    //rooms[a].residents.extend(&rooms[b].residents.iter());
    //let &mut res = &rooms[a].residents;
    // ???? cannot use reference, need copy
    let s = rooms[from].residents.clone();
    rooms[into].residents.extend(s);

    let u = rooms[from].unpopular.clone();
    rooms[into].unpopular.extend(u);
    rooms.remove(from);
    into
}

fn add_minus(name: &str, room: usize, rooms: &mut Vec<Room>) -> Result<usize, bool> {

    if rooms[room].residents.contains(name) {
        return Err(false);
    }
    rooms[room].unpopular.insert(name.to_string());
    Ok(room)
}

fn print_rooms(rooms: &Vec<Room>) {
    let mut l = Vec::new();
    for r in rooms.iter() {
        let mut v = Vec::from_iter(r.residents.iter());
        v.sort();
        let mut joined = String::new();
        for v in v.iter() {
            joined.push_str(v);
            joined.push_str(" ");
        }
        joined.pop();
        l.push(joined);
//        let joined = v.join(" ");
    }

    l.sort();
    for r in l.iter() {
        println!("{}", r);
    }
//    let string_list = vec!["Foo".to_string(),"Bar".to_string()];
//    let joined = string_list.join("-");
}

fn main() {

    let stdin = io::stdin();
    let iterator = stdin.lock().lines();

    let mut rooms = Vec::new();

    let mut x: Option<usize> = None;
    for line in iterator {
        let s = line.unwrap();

        let success = match s.chars().next() {
            Some('+') => add_plus(&s[2..], x.unwrap(), &mut rooms),
            Some('-') => add_minus(&s[2..], x.unwrap(), &mut rooms),
            Some(_)   => add_new(&s, &mut rooms),
            None      => { x = None; continue; },
        };

        match success {
            Ok(v) => {x = Some(v);},
            Err(_e) => {
                println!("hallelujah");
                return;
            }
        }
    }
    print_rooms(&rooms);
}
