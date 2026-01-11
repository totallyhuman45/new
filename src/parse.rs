use crate::parse_expr;
use crate::BinaryOperator;
use crate::UnaryOperator;
use crate::Expr;

// ok little note to i dont think this should be reccursive in my past prjects where i did not have any element of 
//precedence and you declare order based of perenthisis i did it recersively but with this one with the kinda pratt like parser.
// i think this works but it is probobly just much slower but it is not enough to be noticeablez.
// it probobly shouldnt use
// mabey get back to this latter but for now i will move on 



// extractors
pub fn parse_IntLit(exprStr:&str,Line:i32) -> Expr {
    let intNum: u32 = exprStr.trim().parse().expect(&format!("Failed to parse string: {:?} on line {:?}",exprStr,Line));
    return Expr::IntLit(intNum);
}
pub fn parse_FloatLit(exprStr:&str,Line:i32) -> Expr {
    let floatNum: f64 = exprStr.trim().parse().expect(&format!("Failed to parse string: {:?} on line {:?}",exprStr,Line));
    return Expr::FloatLit(floatNum);
}
pub fn parse_StringLit(exprStr:&str,Line:i32) -> Expr {
    let remove_pre = exprStr.strip_prefix("\"").unwrap_or_else(|| {panic!("Failed to parse string: {:?} on line {:?}", exprStr, Line)});
    return Expr::StringLit(remove_pre.strip_suffix("\"").unwrap_or_else(|| {panic!("Failed to parse string: {:?} on line {:?}", exprStr, Line)}).to_string());
}
pub fn parse_BoolLit(exprStr:&str,Line:i32) -> Expr {
    return Expr::BoolLit(exprStr.trim() == "True");
}
pub fn parse_ArrayLit(exprStr:&str,Line:i32) -> Expr {
    let mut removed = exprStr.strip_prefix("[").unwrap_or_else(|| {panic!("Failed to parse string: {:?} on line {:?}", exprStr, Line)});
    removed = removed.strip_suffix("]").unwrap_or_else(|| {panic!("Failed to parse string: {:?} on line {:?}", exprStr, Line)});
    if removed.trim() == ""{
        return Expr::ArrayLit(vec![]);
    }
    let mut result :Vec<Box<Expr>> = vec![];
    for x in removed.split(','){
        result.push(Box::new(parse_expr(x,Line)));
    }
    return Expr::ArrayLit(result)
}
pub fn parse_VecLit(exprStr:&str,Line:i32) -> Expr {
    let mut removed = exprStr.strip_prefix("Vec[").unwrap_or_else(|| {panic!("Failed to parse string: {:?} on line {:?}", exprStr, Line)});
    removed = removed.strip_suffix("]").unwrap_or_else(|| {panic!("Failed to parse string: {:?} on line {:?}", exprStr, Line)});
    if removed.trim() == ""{
        return Expr::ArrayLit(vec![]);
    }
    let mut result :Vec<Box<Expr>> = vec![];
    for x in removed.split(','){
        result.push(Box::new(parse_expr(x,Line)));
    }
    return Expr::VecLit(result)
}



pub fn parse_OppLit(exprStr:&str,Line:i32) -> Expr{
    let mut tokens: Vec<char>= vec![];
    let mut tokensInd: Vec<usize>= vec![];
    let mut nonTokens: Vec<char>= vec![];
    let mut nonTokensInd: Vec<usize>= vec![];
    let chars: Vec<char> = exprStr.chars().collect();
    for (i,x) in exprStr.trim().chars().collect::<Vec<char>>().iter().enumerate(){
        if matches!(x, '-'|'!'|'*'|'&'|'+'|'/'|'%'|'|'|'^'|'<'|'>'|'='|'.'|'('|')'|'0'..='9'|'.'|'\"'|','){
            tokens.push(*x); 
            tokensInd.push(i); 
        }else {
            nonTokens.push(*x); 
            nonTokensInd.push(i);
        }
    }
    
    let mut tokenStrings: Vec<String>= vec![];
    for x in tokens.clone(){
        tokenStrings.push(x.to_string());
    }

    let mut toRemove:Vec<usize> = vec![];

    println!("tokens: {:?}",tokenStrings);
    println!("non: {:?}",nonTokens);
    println!("noni: {:?}",nonTokensInd); 



    let mut j:Vec<(usize,String)> = vec![];
    for (i,x) in tokenStrings.clone().iter().enumerate(){
        j.push((tokensInd[i],x.to_string()));
    }

    let mut toSkip:Vec<usize> = vec![];
    for (i,x) in nonTokens.clone().iter().enumerate() {
        if x.is_ascii_alphabetic() && !toSkip.contains(&i){
            let mut t: usize = 0;
            let mut name: String = "".to_string();

            while nonTokensInd[i] + t < chars.len() &&
                  (chars[nonTokensInd[i] + t].is_ascii_alphanumeric() ||
                   chars[nonTokensInd[i] + t] == '_')
            {
                name.push(chars[nonTokensInd[i] + t]);
                toSkip.push(i+t as usize);
                t += 1;
            }
            let idx = nonTokensInd[i] + t;

            if idx >= chars.len()
                || (chars[idx] != '"' && chars[idx] != '(')
            {

                j.push((nonTokensInd[i],name));

            }
        }
    }
    j.sort_by_key(|k| k.0);

    tokenStrings = j.into_iter().map(|(_, s)| s).collect();

    println!("tokens: {:?}",tokenStrings);


    let mut exc:Vec<usize> = vec![]; 
    for (i,x) in tokens.clone().iter().enumerate() {
        if matches!(x, '0'..='9'|'.'){
            if !exc.contains(&i){
                let mut t:usize = i + 1 as usize;
                while t < tokens.len() && matches!(tokens[t], '0'..='9'|'.') {

                    tokenStrings[i].push(tokens[t]);
                    toRemove.push(t);
                    t += 1;
                }
            }
        }
    }





    



    let mut amountOfStrings: i32 = 0;
    for (i,x) in tokenStrings.clone().iter().enumerate() {
        if *x == '"'.to_string(){
            amountOfStrings += 1;
            if (amountOfStrings %2) == 1{
                let mut curString:String = "".to_string();
                let u: Vec<char> = exprStr.chars().collect::<Vec<char>>();
                let mut y = 0;
                let mut openCount = 0;
                while true{
                    //println!("{:?}", curString);
                    if u[y]=='"'{
                        openCount += 1;
                    }
                    //println!("{:?},{:?}",openCount,u[y]);
                    if openCount == 2*amountOfStrings{
                        curString.push(u[y]);
                        tokenStrings[i] = curString;
                        break;
                    }
                    if openCount == 1*amountOfStrings{
                        curString.push(u[y]);
                    }  
                    y+=1;
                    if y == u.len(){
                        panic!("need a closing \" for every opening \" on line {:?}",Line);
                        break;
                    } 
                }
                let mut t:usize = ((i.clone() as i32+1) as usize).try_into().unwrap();
                while true{
                    toRemove.push(t);
                    if tokenStrings[t] == "\""{
                        break;
                    }
                    t+=1;
                    if y == tokenStrings.len(){
                        panic!("need a closing \" for every opening \" on line {:?}",Line);
                        break;
                    } 
                }
            }
        }
    }
 


    for (i,x) in tokenStrings.clone().iter().enumerate() {
        if i != tokenStrings.len()-1{
            if *x == '('.to_string(){
                let mut t: i32 = 1;
                let mut name: String = "".to_string();
                if let Some(index) = tokensInd[i].checked_sub(t as usize) {
                    if index < chars.len() && (chars[index].is_ascii_alphabetic() || matches!(chars[index], '0'..='9'|'_')){
                        name = name + &chars[tokensInd[i]- t as usize].to_string();
                        t+=1;
                    }
                }
                tokenStrings[i] = name.chars().rev().collect::<String>() + &tokenStrings[i];

            }
        }
    }



    for (i,x) in tokenStrings.clone().iter().enumerate() {
        if i != tokenStrings.len()-1{
            if *x == '('.to_string(){
                let mut y = i;
                let mut perenthisCount = 0;
                let mut inPeren: String = "".to_string();
                while  true{
                    if tokenStrings[y] == '('.to_string(){
                        perenthisCount += 1;
                    }
                    if tokenStrings[y] == ')'.to_string(){
                        if perenthisCount == 1{
                            inPeren = inPeren+&tokenStrings[y];
                            tokenStrings[y] = inPeren;
                            break;
                        }else{
                            perenthisCount -= 1;
                        }
                    }
                    //println!("{:?}",inPeren);
                    inPeren = inPeren+&tokenStrings[y];

                    toRemove.push(y);

                    if y == tokenStrings.len()-1{
                        panic!("need a closing ) for every opening ( on line {:?}",Line);
                        break;
                    }
                    y+=1;
                }

            }
        }
    }

    for (i,x) in tokenStrings.clone().iter().enumerate() {
        if *x =="&".to_string() || *x == "|".to_string(){
            if i < tokenStrings.len(){
                if *x == tokenStrings[i+ 1 as usize].clone(){
                    tokenStrings[i] = tokenStrings[i].clone() + &tokenStrings[i];
                    toRemove.push(i+1 as usize);
                }
            }
        }
        if *x == "<".to_string()|| *x == ">".to_string()|| *x == "=".to_string()|| *x == "!".to_string(){
            if i < tokenStrings.len(){
                if tokenStrings[i+ 1 as usize].clone() == "=".to_string(){
                    tokenStrings[i] = tokenStrings[i].clone() + "=";
                    toRemove.push(i+1 as usize);
                }
            }
        }
        

    }


    println!("{:?}",tokens);
    println!("{:?}",toRemove);
    toRemove.sort_unstable();
    toRemove.dedup();
    for &idx in toRemove.iter().rev() {
        tokenStrings.remove(idx);
    }
    println!("tokens: {:?}",tokenStrings);

    let mut tokensPower: Vec<(String,i32)>= vec![];
    for (i,x) in tokenStrings.iter().enumerate(){
        let left: Vec<String> = tokenStrings.clone()[0..i].to_vec();

        if left.is_empty() && (*x == "!".to_string() || *x == "-".to_string() || *x == "*".to_string()|| *x == "&".to_string()){
            tokensPower.push((x.clone(),10));
        }else{
            tokensPower.push((x.clone(),power_of_operators(&x)));
        }
    }

    println!("tokens power: {:?}",tokensPower);


    let idx = tokensPower
    .iter()
    .enumerate()
    .min_by_key(|(_, (_, power))| *power)
    .unwrap_or_else(|| panic!("empty operation"))
    .0;

    println!("{:?}",idx);

    // first split int binary and uranary operators by geting left and right


    let left: Vec<String> = tokenStrings.clone()[0..idx].to_vec();
    let right: Vec<String> = tokenStrings.clone()[idx+1..tokenStrings.len()].to_vec();

    println!("{:?}",left);
    println!("{:?}",right);

    let mut lefttext: String = "".to_string();
    for x in left{
        lefttext.push_str(&x);
        lefttext.push(' ');
    }
    let mut righttext: String = "".to_string();
    for x in right{
        righttext.push_str(&x);
        righttext.push(' ');
    }
    println!("{:?}",righttext);
    println!("{:?}",lefttext);


    if lefttext == "".to_string(){ 
        // uranary
        if !(tokenStrings.len() > idx+1)  {
            println!("sdaf:{:?}", )            
            panic!("canot use a uranary operator on nothing")
        }
        let mut righttext2: String = tokenStrings[idx+1].clone();
        
        match tokenStrings[idx].as_str(){

            "!" => {
                return Expr::UnaryOp { 
                    op: UnaryOperator::Not,
                    expr: Box::new(parse_expr(&righttext2,Line)),
                }
            },  
            "-" => {
                return Expr::UnaryOp { 
                    op: UnaryOperator::Negate,
                    expr: Box::new(parse_expr(&righttext2,Line)),
                }
            },  
            "*" => {
                return Expr::UnaryOp { 
                    op: UnaryOperator::Dereference,
                    expr: Box::new(parse_expr(&righttext2,Line)),
                }
            },  
            "&" => {
                return Expr::UnaryOp { 
                    op: UnaryOperator::Address,
                    expr: Box::new(parse_expr(&righttext2,Line)),
                }
            },  
            _ => panic!("not a valid operation"), 
        }
    }else{
        // binary
        /*Add,
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
        GreaterEqual,*/
        println!("adsfas {:?}",tokenStrings[idx]);
        match tokenStrings[idx].as_str(){
            "+" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Add,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "-" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Subtract,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "*" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Multiply,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "/" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Divide,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "%" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Modulo,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "||" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Or,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "&&" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::And,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "==" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Equal,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "<" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Less,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            "<=" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::LessEqual,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            ">" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::Greater,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },
            ">=" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::GreaterEqual,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },   
            "!=" => {
                return Expr::BinaryOp {
                    left: Box::new(parse_expr(&lefttext,Line)),   
                    op: BinaryOperator::NotEqual,
                    right: Box::new(parse_expr(&righttext,Line)),
                }
            },                        
            _ => panic!("not a valid operation"),   
        }
    }


    panic!("not a valid operation");


}
pub 
fn power_of_operators(ch: &str) -> i32 {
    match ch {         
        "!" => 8,               
        "*" | "/" | "%" => 7,    
        "+" | "-" => 6,           
        ">" | "<"| ">=" | "<=" => 5,           
        "==" | "!=" => 4,                 
        "&&" => 3,                 
        "||" => 2,                 
        "^" => 1,                 
        _ => 11,                  
    }
}



// classifiers 
pub fn IsIntLit( exprStr:&str,Line:i32) -> bool{
    for x in exprStr.trim().chars(){
        if !matches!(x, '0'..='9'){
            return false
        }
    }
    return true
}

 
pub fn IsFloatLit(exprStr:&str,Line:i32) -> bool{
    for x in exprStr.trim().chars(){
        if !matches!(x, '0'..='9'|'.'){
            return false
        }
    }
    return true
}

 
pub fn IsOpp(exprStr:&str,Line:i32) -> bool{
    for x in exprStr.trim().chars(){
        if matches!(x, '-'|'!'|'*'|'&'|'+'|'/'|'%'|'|'|'^'|'<'|'>'|'='|'.'|'*'){
            return true 
        }
    }
    return false
}


 
pub fn IsBoolLit(exprStr:&str,Line:i32) -> bool{
    return exprStr.trim() == "True" || exprStr.trim() == "False"
}

 
pub fn IsStringLit(exprStr:&str,Line:i32) -> bool{
    let trimmed = exprStr.trim();
    trimmed.starts_with('"') && trimmed.ends_with('"')
}

 
pub fn IsArrayLit(exprStr:&str,Line:i32) -> bool{
    let trimmed = exprStr.trim();
    trimmed.starts_with("[") && trimmed.ends_with(']')
}
pub fn IsVecLit(exprStr:&str,Line:i32) -> bool{
    let trimmed = exprStr.trim();
    trimmed.starts_with("Vec[") && trimmed.ends_with(']')
}

 
pub fn IsFunctionCall(exprStr:&str,Line:i32) -> bool{
    return exprStr.contains("(") && exprStr.trim().ends_with(')');
}

pub fn IsVarbleName(exprStr: &str, _Line: i32) -> bool {
    let exprStr: &str = exprStr.trim();
    if exprStr.is_empty() {
        return false;
    }

    let mut chars = exprStr.chars();

    // Check first character
    match chars.next() {
        Some(c) if c.is_ascii_alphanumeric() || c == '_' => {}
        _ => return false,
    }

    // Check the rest
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
