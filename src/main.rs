use std::io;
use std::collections::HashMap;

fn main() {
    fn show() {
        println!("");
        println!("== ToDo ==");
        println!("1. 추가");
        println!("2. 보기");
        println!("3. 수정");
        println!("4. 삭제");
        println!("");
        println!("Enter selection:");
    }

    let mut to_do_list = ToDoList::new();
    println!("start ToDo");    
    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_menu(&mut to_do_list),
            "2" => view_menu(&to_do_list),
            "3" => update_menu(&mut to_do_list),
            "4" => remove_menu(&mut to_do_list),
            _ => break,
        }
    }
   
}

fn add_menu(to_do_list: &mut ToDoList){
    println!("To Do 작성");
    let mut _to_do = create_to_dt();
    let _id = to_do_list.next_id();
    to_do_list.add(_id, _to_do);
}
fn view_menu(to_do_list: &ToDoList) {
    println!("To Do 목록");
    to_do_list.print();
    println!("==========================");
    for (_id, _to_do) in &to_do_list.items {
        println!("번호 : {:?}", _id);
        _to_do.print();
    }
}
fn update_menu(to_do_list: &mut ToDoList){
    println!("To Do 수정");
    println!("수정할 번호를 입력해주세요.> ");
    let mut _id = match get_input() {
        Some(_id) => _id,
        None => return,
    };

    let _string_to_number: i32 = _id.parse().expect("Not a number");
    println!("{:?}", _string_to_number);

    if to_do_list.items.contains_key(&_string_to_number) {
        let mut _to_do = create_to_dt();
        to_do_list.add(_string_to_number, _to_do);
    } else {
        println!("존재하지 않는 데이터입니다.");
    }
}
fn remove_menu(to_do_list: &mut ToDoList) {
    println!("삭제할 번호를 입력해주세요.> ");
    let mut _id = match get_input() {
        Some(_id) => _id,
        None => return,
    };

    let _string_to_number: i32 = _id.parse().expect("Not a number");

    if to_do_list.items.contains_key(&_string_to_number) {
        println!("{:?}번을 삭제 하시겠습니까? (ok이면 삭제)> ", _string_to_number);
        let _command = match get_input() {
            Some(_command) => _command,
            None => return,
        };

        if _command.as_str() == "ok" {
            to_do_list.delete_item(_string_to_number);
            println!("삭제완료");
        }
    } else {
        println!("존재하지 않는 데이터입니다.");
    }
}
fn create_to_dt () -> ToDo{
    println!("제목 입력 > ");    
    let _title = match get_input() {
        Some(_title) => _title,
        None => "".to_owned(),
    };

    println!("내용 입력 > ");    
    let _comment = match get_input() {
        Some(_comment) => _comment,
        None => "".to_owned(),
    };

    println!("카테고리 입력 > ");
    let _category = match get_input() {
        Some(_category) => _category,
        None => "".to_owned(),
    };

    println!("태그 입력 (여러개 가능, end 입력 시 종료) > ");
    let mut _tags = vec![];
    loop {
        let input_tag = match get_input() {
            Some(input_tag) => input_tag,
            None => "".to_owned(),
        };
        if input_tag.as_str() == "end" {
            break;
        } else {
            _tags.push(input_tag);
        }
    }
    return ToDo::new(
        _category,
        _tags,
        _comment,
        _title,
        false
    );
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}


struct ToDo {
    category: String,
    tags: Vec<String>,
    comment: String,
    title: String,
    is_done: bool, 
}

impl ToDo {
    fn new(
        category: String, 
        tags: Vec<String>,
        comment: String,
        title: String,
        is_done: bool
    ) -> Self {
        Self {
            category,
            tags,
            comment,
            title,
            is_done
        }
    }

    fn print(&self) {
        let isDone = if self.is_done {"O"} else {"X"};
        println!("타이틀 : {:?} 완료 여부 : {:?}", self.title, isDone);
        println!("내용 : {:?}", self.comment);
        println!("카테고리:{:?}", self.category);
        println!("태그");
        for tag in self.tags.iter() {
            print!("{:?} ", tag);
        }
        println!("");
        println!("----------------------------");
    }
}

struct ToDoList {
    items: HashMap<i32, ToDo>,
}

impl ToDoList {
    fn new() -> Self{
        Self {
            items: HashMap::new(),
        }
    }

    fn next_id(&self) -> i32 {
        let last_id = self.items.len() as i32;
        last_id + 1
    }

    fn print (&self) {
        println!(" 총 todo 수 : {:?}", self.items.keys().len());
    }

    fn add(&mut self, _id: i32 ,_to_do: ToDo) {
        self.items.insert(_id, _to_do);
    }

    fn delete_item(&mut self, _id: i32) {
        self.items.remove(&_id);
    }
}