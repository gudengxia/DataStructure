use ds_rust::mylib::liblist::list::List;
//use ds_rust::mylib::liblist::sq_list::SqList;
use ds_rust::mylib::liblist::link_list::LinkList; 
fn main() { 
    let mut l = LinkList::<i32>::new();
        for i in 1..=3{
            let r = l.insert(i, i as i32);
            match r {
                Ok(()) => println!("insert {} ok", i),
                Err(e) => println!("insert {} error:{}", i, e)
            }
        }    
        l.traverse();
        
        let mut r = l.insert(1, -1);
        match r {
            Ok(()) => println!("insert {} ok", -1),
            Err(e) => println!("insert {} error:{}", -1, e)
        }
        l.traverse();

        r = l.insert(7, -2);
        match r {
            Ok(()) => println!("insert {} ok", -2),
            Err(e) => println!("insert {} error:{}", -2, e)
        }   
        l.traverse();

        r = l.insert(3, -5);
        match r {
            Ok(()) => println!("insert {} ok", -5),
            Err(e) => println!("insert {} error:{}", -5, e)
        } 
        l.traverse();

        let mut ans = l.delete(3);
        match ans {
            Ok(e) => println!("delete {} ok, e = {}", 3, e),
            Err(e) => println!("delete {} error:{}", 3, e)
        }
        l.traverse();

        ans = l.delete(1);
        match ans {
            Ok(e) => println!("delete {} ok, e = {}", 1, e),
            Err(e) => println!("delete {} error:{}", 1, e)
        }
        l.traverse();

        ans = l.delete(4);
        match ans{
            Ok(e) => println!("delete {} ok", e),
            Err(e) => println!("delete error:{}", e)
        }
        l.traverse();

        ans = l.delete(4);
        match ans {
            Ok(e) => println!("delete {} ok", e),
            Err(e) => println!("delete error:{}", e)
        }
        l.traverse();

        l.clear();
        l.traverse();
        println!("{}", l.length());
}
