mod parse;
use std::fs;

// expresions
#[derive(Debug, Clone)]
pub enum Expr{
    // Literal values
    IntLit(u32),              // integers
    FloatLit(f64),            // floating point numbers
    BoolLit(bool),            // true/false
    StringLit(String),
    ArrayLit(Vec<Box<Expr>>),    // your generic array
    VecLit(Vec<Box<Expr>>),

    // Variable reference
    Variable(String),

    // Unary operations (like -x, !x)
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expr>,   // <- add generics here
    },

    // Binary operations (like a + b, x * y)
    BinaryOp {
        left: Box<Expr>,   // <- add generics here
        op: BinaryOperator,
        right: Box<Expr>,  // <- add generics here
    },

    // Function call
    Call {
        name: String,
        args: Vec<Box<Expr>>,  // <- add generics here
    },
}
// Optional: enums for operators
#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Dereference, //*x
    Address, //&x
    Negate, // -x
    Not,    // !x
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,    // &&
    Or,     // ||
    Equal,  // ==
    NotEqual, // !=
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub enum tokens {
    Expersion(Expr),
    PerethisisSet(Vec<Box<tokens>>),
    KeyWord(KeyWord),
    Type(Type),
    Word(String),
}

#[derive(Debug, Clone, Copy)]
pub enum KeyWord {
    If,
    For,
    While,
    Fn,
    Colin, 
    Const,
    Pointer, // * for types
    Alloc,
    Free,
}

#[derive(Debug, Clone)]
pub enum Type {
    I8, 
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
    Void,
    Array(Box<Type>,i32),
}




fn main() {
    let source = load_file("/home/keller-polk/Desktop/compiler/src/test.lang");
    
    println!("{:#?}",lexer(&source)); 

    //println!("{:#?}",parse_expr("a * &a",1));  
}



fn load_file(path: &str) -> String {
    fs::read_to_string(path).expect("failed to read file")
}

fn lexer(Code:&str) -> Vec<tokens>{
    let mut lines: Vec<String> = vec!["".to_string()];
    let mut curln: i32 = 0;
    for (i,x) in Code.chars().collect::<Vec<char>>().iter().enumerate(){
        if x == &';'||x == &'{'||x == &'}'{
            println!("end of line");
            lines.push("".to_string());
            curln+=1;
        }else{
            lines[curln as usize].push(*x);
        }
    }
    for line in lines.iter_mut() {
        *line = line.replace('\n', "");
        *line = line.replace('\t', "");
    }
    println!("lines: {:?}", &lines);

    let mut linesTok: Vec<Vec<tokens>> = vec![];
    for (i,line) in lines.iter().enumerate() {
        linesTok.push(lexLine(line.to_string(),(i as i32 + 1)));
    }

    vec![]
}
fn lexLine(Line: String,LineNum:i32) -> Vec<tokens>{
    // so we will split into parets then get tokens the condidon in whic we splitare 
    //like colins or like perenthsis and sometimes spaces if not folowed by a binary operator dont split at commas

    let mut result:Vec<tokens> = vec![]; 

    let mut split:Vec<String> = vec![]; 

    let mut c = 0;
    let mut s = "".to_string();
    for (i,t) in Line.clone().chars().collect::<Vec<char>>().iter().enumerate(){
        if i < c{
            break;
        }
        
        if !matches!(*t, ':'|' '){
            s.push(*t);
        }else{
            if *t == ':'{
                split.push(s);
                s = "".to_string();
                split.push(":".to_string()); 
                c = i;
           
            } else if  *t == ' '{
                // if after space ther is a binary operator we dont stop but we haave to check for if after out = thers is anouther =
                let mut condtmp = true;
                let mut u: char = Line.clone().chars().collect::<Vec<char>>()[i];
                let mut j = i as i32;
                while true {
                    
                   if u == ' '{
                        j += 1;
                        u = match Line.clone().chars().collect::<Vec<char>>().get(j as usize) {
                            Some(expr) => *expr,
                            None => break,
                        };
                        continue;
                   }else{
                        let mut condtmp2 = false;
                        if u == '=' || u == '!'{
                            let seccondchar: char  = match Line.clone().chars().collect::<Vec<char>>().get((j + 1) as usize) {
                                Some(expr) => *expr,
                                None => ' ',
                            };
                            //println!("seccondchar: {:?}", seccondchar);

                            if seccondchar == '='{
                                condtmp2 = true
                            } 
                        }
                        if matches!(u, '-'|'!'|'*'|'&'|'+'|'/'|'%'|'|'|'<'|'>') || condtmp2{
                            condtmp = false;
                            break
                        }else {
                            break;
                        }
                   } 
                }
                let mut j = i as i32 -1;
                let mut y = true;
                u = match Line.clone().chars().collect::<Vec<char>>().get(j as usize) {
                    Some(expr) => *expr,
                    None => {
                        y = false;
                        '\0'
                    },
                };
                while y {
                   if u == ' '{
                        j -= 1;
                        u = match Line.clone().chars().collect::<Vec<char>>().get(j as usize) {
                            Some(expr) => *expr,
                            None => break,
                        };
                        continue;
                   }else{
                        let mut condtmp2 = false;
                        if u == '='{
                            let seccondchar: char  = match Line.clone().chars().collect::<Vec<char>>().get((j - 1) as usize) {
                                Some(expr) => *expr,
                                None => ' ',
                            };
                            //println!("seccondchar: {:?}", seccondchar);

                            if matches!(seccondchar, '='|'!'|'<'|'>'){
                                condtmp2 = true
                            } 
                        }
                        if matches!(u, '-'|'!'|'*'|'&'|'+'|'/'|'%'|'|'|'<'|'>'|'!') || condtmp2{
                            condtmp = false;
                            break
                        }else {
                            break;
                        }
                   } 
                }




                if condtmp {
                    if s != ""{
                        split.push(s);
                        s = "".to_string();
                        c = i;
                    }    
                }
            }
        }
    }
    if s != ""{
        split.push(s.clone());
    }

    println!("s: {:?}", s);
    println!("split: {:?}", split);
    

    for (i,x) in split.iter().enumerate(){
        if x == "="{

        } else if parse::IsOpp(x,LineNum){
            result.push(tokens::Expersion(parse_expr(x,LineNum)));
        }

    }
    println!("tokens: {:#?}", result);
    return todo!()
}




fn parse_expr(exprStr:&str,Line:i32) ->  Expr{

    if parse::IsIntLit(exprStr,Line){
        return parse::parse_IntLit(exprStr,Line)
    }
    if parse::IsFloatLit(exprStr,Line){
        return parse::parse_FloatLit(exprStr,Line)
    }
    if parse::IsStringLit(exprStr,Line){
        return parse::parse_StringLit(exprStr,Line)
    }
    if parse::IsBoolLit(exprStr,Line){
        return parse::parse_BoolLit(exprStr,Line)
    }
    if parse::IsArrayLit(exprStr,Line){
        return parse::parse_ArrayLit(exprStr,Line)
    }
    if parse::IsVecLit(exprStr,Line){
        return parse::parse_VecLit(exprStr,Line)
    }
    if parse::IsVarbleName(exprStr,Line){
        return Expr::Variable(exprStr.trim().to_string())
    }
    if parse::IsOpp(exprStr,Line){
        return parse::parse_OppLit(exprStr,Line) 
    }
    if parse::IsFunctionCall(exprStr,Line){
        return parse::parse_FuncLit(exprStr,Line)
    }
    panic!("invalid expresion {:?}", exprStr)

}










