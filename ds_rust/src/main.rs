//use ds_rust::mylib::liblist::list::List;
//use ds_rust::mylib::liblist::sq_list::SqList;
//use ds_rust::mylib::liblist::double_link_list::DLinkList;
//use ds_rust::mylib::app::libqueen::EQueen;
//use ds_rust::mylib::liblist::libkmp::get_next;
//use ds_rust::mylib::libtree::binarytree::BiTree;
use ds_rust::mylib::app::prime::{is_prime, EularSieve};
use std::time::Instant;
fn main(){
    let n = 100000;
    let mut eular = EularSieve::new();
    let start1 = Instant::now();
    eular.solve(n);
    let duration1 = start1.elapsed();
    println!("Eular Sieve run time:{:?}", duration1);

    let start2 = Instant::now();
    let mut ans = Vec::<usize>::new();
    for i in 1..=n{
        if is_prime(i)
        {
            ans.push(i);
        }
    }
    let duration2 = start2.elapsed();
    println!("Common method run time:{:?}", duration2);
}
/*fn main(){
    let mut bt = BiTree::<char>::new();
    let v = [Some('A'), Some('B'), None, Some('C'), None, Some('D'), None, None, Some('E'), Some('F'), None, Some('G'), None, None, None];
    let ans = bt.build_tree(v.to_vec());
    match ans{
        Ok(()) => println!("init tree ok!"),
        Err(err) => println!("init tree error: {}", err)
    }
    println!("Preorder Traverse:");
    bt.preorder();
    println!("Inorder Traverse:");
    bt.inorder();
    println!("Postorder Traverse:");
    bt.postorder();
}*/

/*fn main(){
    let mut q = EQueen::new(8);
    let num_of_solution  = q.solve();
    println!("number of solution: {}", num_of_solution);
}*/

/*fn main() { 
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

    let n = l.length();
    for i in 0..=(n+1){
        match l.get(i){
            Ok(e) =>{
                if e > 0{
                    let _ = l.set(i, e*100);
                }
            }
            Err(err) => println!("get at pos {} error:{}", i, err)
        }
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
    
    println!("len = {}", l.length());
}*/