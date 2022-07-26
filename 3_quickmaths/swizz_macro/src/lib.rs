extern crate proc_macro;
use proc_macro::{TokenStream};
use itertools::Itertools;
use std::format;

// Generates a swizzled accessor
#[proc_macro]
pub fn gen_swizz(input: TokenStream) -> TokenStream {
    let in_str = input.into_iter().next().unwrap().to_string();
    let mut out_src = "".to_string();
    out_src.push_str(&"fn ");
    out_src.push_str(&in_str);
    if in_str.chars().count() == 1 {
        out_src.push_str(&"(&self) -> Scalar { return ");
        match &in_str.chars().next().unwrap() {
            'x' | 'r' | 'u' => out_src.push_str(&"self.0.v[0]; }"),
            'y' | 'g' | 'v' => out_src.push_str(&"self.0.v[1]; }"),
            'z' | 'b' => out_src.push_str(&"self.0.v[2]; }"),
            'w' | 'a' => out_src.push_str(&"self.0.v[3]; }"),
            _ => panic!(),
        }
    }
    else {
        out_src.push_str(&"(&self) -> Vec");
        out_src.push_str(&in_str.chars().count().to_string());
        out_src.push_str(&"{ return ");
        out_src.push_str(&"Vec");
        out_src.push_str(&in_str.chars().count().to_string());
        out_src.push_str(&" { 0: VecN::<");
        out_src.push_str(&in_str.chars().count().to_string());
        out_src.push_str(&"> { v: [");
        for c in in_str.chars() {
            match c {
                'x' | 'r' | 'u' => out_src.push_str(&"self.0.v[0], "),
                'y' | 'g' | 'v' => out_src.push_str(&"self.0.v[1], "),
                'z' | 'b' => out_src.push_str(&"self.0.v[2], "),
                'w' | 'a' => out_src.push_str(&"self.0.v[3], "),
                _ => panic!(),
            }
        }
        out_src.push_str(&"] } }; }");
    }
    return out_src.parse().unwrap();
}

// Generates a swizzled setter
#[proc_macro]
pub fn gen_swizz_assign(input: TokenStream) -> TokenStream {
    let in_str = input.into_iter().next().unwrap().to_string();
    let mut out_src = "".to_string();
    out_src.push_str(&"fn set_");
    out_src.push_str(&in_str);
    if in_str.chars().count() == 1 {
        out_src.push_str(&"(&mut self, v: Scalar) {");
        match &in_str.chars().next().unwrap() {
            'x' | 'r' | 'u' => out_src.push_str(&"self.0.v[0] = v; }"),
            'y' | 'g' | 'v' => out_src.push_str(&"self.0.v[1] = v; }"),
            'z' | 'b' => out_src.push_str(&"self.0.v[2] = v; }"),
            'w' | 'a' => out_src.push_str(&"self.0.v[3] = v; }"),
            _ => panic!(),
        }
    }
    else {
        out_src.push_str(&"(&mut self, v: Vec");
        out_src.push_str(&in_str.chars().count().to_string());
        out_src.push_str(&") {");
        for (i, c) in in_str.chars().enumerate() {
            match c {
                'x' | 'r' | 'u' => { out_src.push_str(&"self.0.v[0] = v.0.v["); out_src.push_str(&i.to_string()); out_src.push_str(&"];") },
                'y' | 'g' | 'v' => { out_src.push_str(&"self.0.v[1] = v.0.v["); out_src.push_str(&i.to_string()); out_src.push_str(&"];")  },
                'z' | 'b' => { out_src.push_str(&"self.0.v[2] = v.0.v["); out_src.push_str(&i.to_string()); out_src.push_str(&"];")  },
                'w' | 'a' => { out_src.push_str(&"self.0.v[3] = v.0.v["); out_src.push_str(&i.to_string()); out_src.push_str(&"];")  },
                _ => panic!(),
            }
        }
        out_src.push_str(&"}");
    }
    return out_src.parse().unwrap();
}

// Generate swizzled functions for Scalar, Vec2, Vec3 and Vec4
#[proc_macro]
pub fn gen_swizz_funcs(input: TokenStream) -> TokenStream {
    let in_str = input.into_iter().next().unwrap().to_string();
    let chars_initial: Vec<&str> = in_str.split_terminator("").collect();
    let chars_initial = &chars_initial[1..];
    let mut chars: Vec<String> = chars_initial.iter().map(|x| x.to_string()).collect();
    let mut out_src = "".to_string();
    for char_arr in &chars {
        out_src.push_str(&"gen_swizz!(");
        out_src.push_str(char_arr);
        out_src.push_str(&");\n");
        out_src.push_str(&"gen_swizz_assign!(");
        out_src.push_str(char_arr);
        out_src.push_str(&");\n");
    }
    for vec_d in 0..3 {
        chars = chars.iter().cartesian_product(chars_initial.iter()).map(|x| format!("{}{}", x.0, x.1)).collect();
        for char_arr in &chars {
            out_src.push_str(&"gen_swizz!(");
            out_src.push_str(char_arr);
            out_src.push_str(&");\n");
            if vec_d <= chars_initial.len() {
                out_src.push_str(&"gen_swizz_assign!(");
                out_src.push_str(char_arr);
                out_src.push_str(&");\n");
            }
        }
    }
    return out_src.parse().unwrap();
}

#[proc_macro]
pub fn gen_constructor(input: TokenStream) -> TokenStream {
    let num_elements = input.into_iter().next().unwrap().to_string().parse::<i32>().unwrap();
    let mut out_src = "".to_string();
    out_src.push_str(&"fn new(");
    for element in 0..num_elements {
        out_src.push_str(&"v");
        out_src.push_str(&element.to_string());    
        out_src.push_str(&": Scalar,");
    }
    out_src.push_str(&") -> Self { return Self{ 0: VecN::<");
    out_src.push_str(&num_elements.to_string());
    out_src.push_str(&">{ v: [");
    for element in 0..num_elements {
        out_src.push_str(&"v");
        out_src.push_str(&element.to_string());    
        out_src.push_str(&",");
    }
    out_src.push_str(&"] } }; }");
    return out_src.parse().unwrap();
}

#[proc_macro]
pub fn gen_elementwise(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let return_type = input_iter.next().unwrap().to_string();
    let num_elements = input_iter.next().unwrap().to_string().parse::<i32>().unwrap();
    let operator = input_iter.next().unwrap().to_string();
    let mut out_src = "".to_string();
    out_src.push_str(&return_type.to_string());
    out_src.push_str(&"{ 0: VecN::<");
    out_src.push_str(&num_elements.to_string());
    out_src.push_str(&"> { v: [");
    for element_num in 0..num_elements {
        out_src.push_str(&"a.0.v[");
        out_src.push_str(&element_num.to_string());
        out_src.push_str(&"]");
        out_src.push_str(&operator);
        out_src.push_str(&"b.0.v[");
        out_src.push_str(&element_num.to_string());
        out_src.push_str(&"],");
    }
    out_src.push_str(&"] } }");
    return out_src.parse().unwrap();
}

#[proc_macro]
pub fn gen_scalar_right(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let return_type = input_iter.next().unwrap().to_string();
    let num_elements = input_iter.next().unwrap().to_string().parse::<i32>().unwrap();
    let operator = input_iter.next().unwrap().to_string();
    let mut out_src = "".to_string();
    out_src.push_str(&return_type.to_string());
    out_src.push_str(&"{ 0: VecN::<");
    out_src.push_str(&num_elements.to_string());
    out_src.push_str(&"> { v: [");
    for element_num in 0..num_elements {
        out_src.push_str(&"a.0.v[");
        out_src.push_str(&element_num.to_string());
        out_src.push_str(&"]");
        out_src.push_str(&operator);
        out_src.push_str(&"b,");
    }
    out_src.push_str(&"] } }");
    return out_src.parse().unwrap();
}

#[proc_macro]
pub fn gen_scalar_left(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let return_type = input_iter.next().unwrap().to_string();
    let num_elements = input_iter.next().unwrap().to_string().parse::<i32>().unwrap();
    let operator = input_iter.next().unwrap().to_string();
    let mut out_src = "".to_string();
    out_src.push_str(&return_type.to_string());
    out_src.push_str(&"{ 0: VecN::<");
    out_src.push_str(&num_elements.to_string());
    out_src.push_str(&"> { v: [");
    for element_num in 0..num_elements {
        out_src.push_str(&"a");
        out_src.push_str(&operator);
        out_src.push_str(&"b.0.v[");
        out_src.push_str(&element_num.to_string());
        out_src.push_str(&"],");
    }
    out_src.push_str(&"] } }");
    return out_src.parse().unwrap();
}
#[proc_macro]
pub fn gen_basic_ops(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let return_type = input_iter.next().unwrap().to_string();
    let num_elements = input_iter.next().unwrap().to_string().parse::<i32>().unwrap();
    return format!("
        impl_op!(+ |a: {1}, b: {1}| -> {1} {{ gen_elementwise!({1} {0} +) }});
        impl_op!(+= |a: &mut {1}, b: {1}| {{ *a = *a + b }});
        impl_op!(* |a: {1}, b: {1}| -> {1} {{ gen_elementwise!({1} {0} *) }});
        impl_op!(*= |a: &mut {1}, b: {1}| {{ *a = *a * b }});
        impl_op!(- |a: {1}, b: {1}| -> {1} {{ gen_elementwise!({1} {0} -) }});
        impl_op!(-= |a: &mut {1}, b: {1}| {{ *a = *a - b }});
        impl_op!(/ |a: {1}, b: {1}| -> {1} {{ gen_elementwise!({1} {0} /) }});
        impl_op!(/= |a: &mut {1}, b: {1}| {{ *a = *a / b }});
        impl_op_commutative!(+ |a: {1}, b: Scalar| -> {1} {{ gen_scalar_right!({1} {0} +) }});
        impl_op_commutative!(* |a: {1}, b: Scalar| -> {1} {{ gen_scalar_right!({1} {0} *) }});
        impl_op!(- |a: {1}, b: Scalar| -> {1} {{ gen_scalar_right!({1} {0} -) }});
        impl_op!(- |a: Scalar, b: {1}| -> {1} {{ gen_scalar_left!({1} {0} -) }});
        impl_op!(/ |a: {1}, b: Scalar| -> {1} {{ gen_scalar_right!({1} {0} /) }});
        impl_op!(/ |a: Scalar, b: {1}| -> {1} {{ gen_scalar_left!({1} {0} /) }});
    ", num_elements, return_type).parse().unwrap();
}