mod antlr;

trait Data<T> {
    fn accept(&mut self, visitor: &mut Visitor<T>) -> T;
}

trait Visitor<T> {
    fn visit_a(&mut self, a: &DataA) -> T;
    fn visit_b(&mut self, b: &DataB) -> T;
}

struct DataA;
struct DataB;

impl Data<String> for DataA {
    fn accept(&mut self, visitor: &mut Visitor<String>) -> String {
        visitor.visit_a(&self)
    }
}

impl Data<String> for DataB {
    fn accept(&mut self, visitor: &mut Visitor<String>) -> String {
        visitor.visit_b(&self)
    }
}

struct StringVisitor;

impl Visitor<String> for StringVisitor {
    fn visit_a(&mut self, _a: &DataA) -> String { "got an A".to_owned() }
    fn visit_b(&mut self, _b: &DataB) -> String { "got a B".to_owned() }
}

fn data_to_string(data: &mut Data<String>) -> String {
    data.accept(&mut StringVisitor)
}

fn main() {
    let mut data = DataA;
    let res = data_to_string(&mut data);
    println!("Res is {}", res);
}