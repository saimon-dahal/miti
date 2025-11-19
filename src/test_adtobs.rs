use adtobs::converter;

fn main() {
    let ad_date = "2024-05-21";
    let bs_date = converter::convert_ad_to_bs(ad_date);
    println!("AD: {} -> BS: {:?}", ad_date, bs_date);
    
    // Check if there is a reverse conversion
    // I'll try to guess the function name or check if it exists by trying to compile
    // If this fails to compile, I know I need to implement it myself or look harder.
    // converter::convert_bs_to_ad("2081-02-08"); 
}
