use ds_rust::mylib::liblist::list::List;
//use ds_rust::mylib::liblist::sq_list::SqList;
use ds_rust::mylib::liblist::double_link_list::DLinkList; 
fn main() { 
    let mut l = DLinkList::<i32>::new();
    let mut pos;
    let mut e;
    for i in 1..=3{
        let r = l.insert(i, i as i32);
        match r {
            Ok(()) => println!("insert {} at pos {} ok", i, i),
            Err(err) => println!("insert {} at pos {} error:{}", i, i, err)
        }
    }    
    l.traverse();

    pos = 1;
    e = -1;  
    let mut r = l.insert(1, -1);
    match r {
        Ok(()) => println!("insert {} at {} ok", e, pos),
        Err(err) => println!("insert {} at {} error:{}", e , pos, err)
    }
    l.traverse();

    pos = 7;
    e = -2;
    r = l.insert(pos, e);
    match r {
        Ok(()) => println!("insert {} at {} ok", e, pos),
        Err(err) =>println!("insert {} at {} error:{}", e , pos, err)
    }
    l.traverse();
    println!("len={}", l.length());


    pos = 3;
    e = -5;
    r = l.insert(pos, e);
    match r {
        Ok(()) => println!("insert {} at {} ok", e, pos),
        Err(err) => println!("insert {} at {} error:{}", e , pos, err)
    }
    l.traverse();

    pos = 3;
    let mut ans = l.delete(pos);
    match ans {
        Ok(e) => println!("delete {} at {}", e, pos),
        Err(err) => println!("delete at pos {} error:{}", pos, err)
    }
    l.traverse();

    pos = 1; 
    ans = l.delete(pos);
    match ans {
        Ok(e) => println!("delete {} at {}", e, pos),
        Err(err) => println!("delete at pos {} error:{}", pos, err)
    }
    l.traverse();

    pos = 4;
    ans = l.delete(pos);
    match ans {
        Ok(e) => println!("delete {} at {}", e, pos),
        Err(err) => println!("delete at pos {} error:{}", pos, err)
    }
    l.traverse();

    pos = 4;
    ans = l.delete(pos);
    match ans {
        Ok(e) => println!("delete {} at {}", pos, e),
        Err(err) => println!("delete at pos {} error:{}", pos, err)
    }
    l.traverse();

    l.clear();
    l.traverse();
    println!("{}", l.length());
}
