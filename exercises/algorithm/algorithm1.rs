/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;
use std::cmp::PartialOrd;


#[derive(Debug)]
#[derive(Clone)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
 
    pub fn add(&mut self, obj: T) 
    {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }
    //获取从头结点开始的第index（由0开始）个节点的值
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    // 插入方法
    fn insert(&mut self, mut list:  LinkedList<T>) -> LinkedList<T>
    where 
        T: PartialOrd + Copy,
    {
        //创建一个新的链表
        let mut new_list:LinkedList<T> = LinkedList::new();
        // 该怎么插入呢？一个一个比较大小？                         
                                                                 // b:2、4、6、10
                                                                 // a:1、3、5、7、9
                                                                 // 1 2 
        // 先从第一个元素开始比较大小
        // 先对 b 开始循环
        let mut add_num_a = 0;
        let mut add_num_b = 0;
        let mut start = 0;
        for index in 0..list.length{
            let  b_value = list.get(index as i32).expect("啊？");
            //开始对 a 循环
            if start == self.length {
                break;
            }
            for indexs in start..self.length{
                let a_value = self.get(indexs as i32).expect("啊啊？");
                if *b_value > *a_value{
                    new_list.add(*a_value);
                    start +=1;
                    add_num_a +=1;
                    
                }else{
                    new_list.add(*b_value);
                    add_num_b +=1;
                    break;
                }
            }
        }
        if add_num_a < self.length{
            for index in add_num_a..self.length{
                let a_value = self.get(index as i32).expect("啊啊啊？");
                new_list.add(*a_value);
            }
        }else if add_num_b < list.length{
            for index in add_num_b..list.length{
                let b_value = list.get(index as i32).expect("啊啊啊？");
                new_list.add(*b_value);
            }
        }
        new_list
    }
	pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self
    where T: std::cmp::PartialOrd + Copy,
	{
		//TODO
        
        // 1 2 
		
        // 最终返回的新链表
        let new_list:LinkedList<T>;
       
        new_list = list_a.insert(list_b);
        new_list
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display + Clone + PartialOrd,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display + Clone + PartialOrd,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}