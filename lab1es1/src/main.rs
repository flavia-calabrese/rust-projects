use clap::Parser;
const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";

const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

#[derive(Parser, Debug)]
struct Args {
    // input string
    #[arg(short, long)]
    slug_in: String,

    #[arg(short, long)]
    repeat: Option<i32>,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {

    let args = Args::parse();
    //println!("{}" , args.slug_in);

    println!("{}", slugfy(&args.slug_in, &args));
/*    let str1 = slugfy("ààAAAAA??%%%%$$$aaaaa??");
    println!("{}", str1);

    println!("{}", slugfy("$")); **/
}

fn conv(c: char) -> char {
    let input: Vec<char> = SUBS_I.chars().collect();
    let output: Vec<char> = SUBS_O.chars().collect();
    let mut my_ch = '-';

    let  tmp = c.to_lowercase().to_string();
    let tmp_c:Vec<char> =  tmp.chars().collect();
    if tmp_c.len() == 1 && tmp_c[0].is_alphanumeric() {
        my_ch = tmp_c[0];
        for (index, ch) in input.iter().enumerate()  {
            if ch == &tmp_c[0]  {
                my_ch = output[index];
            }
        }
    }
    if !my_ch.is_ascii(){
        my_ch = '-';
    }
    return my_ch;
}

fn slugfy(s: &str, args: &Args) -> String {
    let mut i = 0;
    let mut result: String = String::new();

    for c in s.chars() {
        let  my_ch = conv(c);
        if args.verbose {
            println!("{} -> {}", c, my_ch);
        }
        //due “-” consecutivi non sono ammessi, solo il primo viene tenuto
        if my_ch == '-' && i < 1 {
            result.push(my_ch);
            i += 1;
        }
        // un “-” finale non è ammesso a meno che non sia l’unico carattere nella stringa
         else if my_ch != '-' {
            result.push(my_ch);
            i = 0;
        }
    }
    if result.ends_with('-') && result.chars().count() > 1 {
        result.pop();
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv_accented_letter() {
        assert_eq!(conv('è'), 'e');
    }

    #[test]
    fn conv_not_accente_letter() {
        assert_eq!(conv('è'), 'e');
    }

    #[test]
    fn conv_unknown_letter() {
        assert_eq!(conv('漢'), '-');
    }

    #[test]
    fn conv_uknown_eccented_letter(){
        assert_eq!(conv('ῶ'), '-');
    }

    #[test]
    fn conv_multiple_words_string(){
        assert_eq!(slugfy("ciao come va"), "ciao-come-va");
    }

    #[test]
    fn conv_accented_string(){
        assert_eq!(slugfy("lllàlll"), "lllalll");
    }

    #[test]
    fn conv_empty_string(){
        assert_eq!(slugfy(""), "");
    }

    #[test]
    fn conv_consecutive_spaces_string(){
        assert_eq!(slugfy("ciao   bre"), "ciao-bre");
    }

    #[test]
    fn conv_more_unknown_chars_string(){
        assert_eq!(slugfy("漢漢漢 hdsbfhb èhbdì"), "-hdsbfhb-ehbdi");
    }

    #[test]
    fn conv_only_unknown_chars_string(){
        assert_eq!(slugfy("漢漢漢"), "-");
    }

    #[test]
    fn conv_end_space_string(){
        assert_eq!(slugfy("hola "), "hola");
    }

    #[test]
    fn conv_more_unknown_chars_at_the_end_string(){
        assert_eq!(slugfy("ciao漢漢漢"), "ciao");
    }

}