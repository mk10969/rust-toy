#[cfg(test)]
mod tests {
    use maplit::hashmap;
    #[test]
    fn test_generate_hashmap() {
        println!("map!!!!!");
        // macroで生成する
        let map = hashmap!(
            "foo" => 10,
            "bar" => 20,
            "baz" => 30,
        );

        println!("{:?}", map);
    }
}
