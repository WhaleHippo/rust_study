#[derive(Debug)]
enum Route {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn route_destination(route: Route) -> String {
    match route {
        Route::V4(address) => format!("IPv4 {address}"),
        Route::V6(address) => format!("IPv6 {address}"),
    }
}

fn selected_route_destination(route: Option<Route>) -> String {
    let Some(route) = route else {
        return String::from("no route selected");
    };
    route_destination(route)
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    value.map(|number| number + 1)
}

fn main() {
    println!("Chapter 06: Enums and Pattern Matching");

    let home = Route::V4(String::from("127.0.0.1"));
    let loopback = Route::V6(String::from("::1"));
    println!(
        "routes: {}, {}",
        route_destination(home),
        route_destination(loopback)
    );

    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("if let found a state quarter: {state:?}");
    }
    println!(
        "coin values: penny={}, nickel={}, dime={}",
        coin_value(Coin::Penny),
        coin_value(Coin::Nickel),
        coin_value(Coin::Dime)
    );

    let next = plus_one(Some(5));
    match next {
        Some(value) => println!("Option matched: {value}"),
        None => println!("Option matched: no value"),
    }
    println!("let else: {}", selected_route_destination(None));
}

#[cfg(test)]
mod tests {
    use super::{Coin, Route, UsState, coin_value, plus_one, selected_route_destination};

    #[test]
    fn values_state_quarter_when_coin_carries_state_data() {
        // Given: a quarter carrying its issuing state.
        let coin = Coin::Quarter(UsState::Alaska);
        // When: the owned enum is matched for its value.
        let value = coin_value(coin);
        // Then: the quarter is worth twenty-five cents.
        assert_eq!(value, 25);
    }

    #[test]
    fn returns_none_when_option_has_no_number() {
        // Given: an absent optional number.
        let value = None;
        // When: Option::map applies addition only to a present value.
        let next = plus_one(value);
        // Then: it remains absent.
        assert_eq!(next, None);
    }

    #[test]
    fn describes_v6_when_let_else_receives_selected_route() {
        // Given: a selected IPv6 route.
        let route = Some(Route::V6(String::from("::1")));
        // When: let-else extracts the route.
        let destination = selected_route_destination(route);
        // Then: the variant data identifies the IPv6 route.
        assert_eq!(destination, "IPv6 ::1");
    }
}
