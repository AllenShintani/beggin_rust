fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // もしlet x だったら？=> 新しいxという別の変数の宣言が行われている
    println!("The value of x is: {}", x);

    //ここからシャドーイングの実験
    let shadow = "   ";
    let shadow = shadow.len();
    println!("{}", { shadow })

    //mutは新しい変数じゃないから型を変える事はできない(letは新しく宣言した型がたまたま違っただけで変更しているわけではない。)
    //エラーになるお
    /*let mut shadow_mut = "   ";
    shadow_mut = shadow_mut.len();
    println!(shadow_mut)*/

    
}
