#[macro_use] extern crate log;
use std::{rc::Rc, sync::{Mutex, Arc}, thread, cell::{Cell, RefCell}, borrow::{Borrow, BorrowMut}};

use simple_logger::SimpleLogger;

fn main() {
    println!("Hello, world!");
}

fn init_log(){
    SimpleLogger::new().init();
}

#[test]
fn test_box(){
    // Box 智能指针，在堆栈上分配内存

    init_log();
    struct Note{
        data:usize,
        next:Option<Box<Note>>,
    }
    impl Note {
        pub fn add(&mut self, value:usize) {
            let mut tmp = self;
            loop {
                if tmp.next.is_none(){
                    tmp.next = Some(Box::new(
                        Note{
                            data:value,
                            next: None,
                        }
                    ));
                    break; 
                }else{
                    tmp = tmp.next.as_mut().unwrap();
                }
            }
        }
        pub fn show(&self){
            let mut tmp = self;
            loop {
                info!("{}", tmp.data);

                if tmp.next.is_none(){ break;}
                else{
                    tmp = tmp.next.as_ref().unwrap();
                }
            }
        }
    }

    let mut note = Note{ data: 32, next: None };
    note.add(1);
    note.add(2);
    note.add(3);
    note.add(5);
    note.add(8);

    note.show();

}

#[test]
fn test_rc(){
    // Rc 智能指针，在堆中分配内存
    // 可以被多个对象共享， 共享时引用计数加一
    // 只能被引用，不能被借用(语法结构层次上)
    // 只能用于单线程
    struct Point{
        x:usize,
        y:usize,
        z:Mutex<Vec<usize>> 
    }


    let point = Rc::new(
        Point{ x: 1, y: 22 ,z:Mutex::new(vec![])}
    );

    let point2 = point.as_ref();
    let mut v = point.z.lock().unwrap();
    v.push(1);

    // ============================================================
    // 这是不行的
    // let nums = Rc::new(vec![1,2,3,4,5]);
    // let mut childs = vec![];
    // for i in 0..5{
    //     let ns = nums.clone();
    //     let c = thread::spawn(move ||{ println!("{}",ns[i])});
    //     childs.push(c);
    // }

    // for c in childs{
    //     c.join().unwrap();
    // }
}

#[test]
fn test_arc(){
    // 与Rc几乎一样
    // 不同在于Arc是多线程安全的
    let nums = Arc::new(vec![1,2,3,4,5]);
    let mut childs = vec![];
    for i in 0..5{
        let ns = nums.clone();
        let c = thread::spawn(move ||{ println!("{}",ns[i])});
        childs.push(c);
    }

    for c in childs{
        c.join().unwrap();
    }
}

#[test]
fn test_cell(){
    // Cell 是内部可变
    // 例如一个引用的值，也可以通过Cell包裹从而改变它的值
    // 可同时存在多个不可变引用，或单个可变引用
    // 可通过get方法返回一个clone
    let ref cell = Cell::new(1);

    // 这里是引用，但也可以修改数据
    cell.set(2);

    let ref cell2 = Cell::new(vec![]);
    let x1 = cell2.borrow();
    let mut x = x1.take();
    x.push(1);

}

#[test]
fn test_refcell(){
    // 大部分与Cell相同
    // 区别在于返回的是引用类型，所以其包裹的数据大部分未实现Copy特性
    // 同理没有get方法
    // 可同时存在多个不可变引用，或单个可变引用

    let refcell = RefCell::new(1);

    let x1 = refcell.borrow();
    let x2 = refcell.borrow();
    drop(x1);
    drop(x2);
    let x3 = refcell.borrow_mut();

}