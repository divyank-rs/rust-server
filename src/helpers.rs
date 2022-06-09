pub fn pricing_logic(frequency: i32, price: i32) -> i32 {
    if frequency < 10 {
        return (0.80 * (price as f32)) as i32;
    } else if frequency > 50 {
        return (1.1 * (price as f32)) as i32;
    }
    price
}
