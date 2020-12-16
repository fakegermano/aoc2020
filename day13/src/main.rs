/*
--- Day 13: Shuttle Search ---

Your ferry can make it safely to a nearby port, but it won't get much further. When you call to book another ship, you discover that no ships embark from that port to your vacation island. You'll need to get from the port to the nearest airport.

Fortunately, a shuttle bus service is available to bring you from the sea port to the airport! Each bus has an ID number that also indicates how often the bus leaves for the airport.

Bus schedules are defined based on a timestamp that measures the number of minutes since some fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the sea port. After that, each bus travels to the airport, then various other locations, and finally returns to the sea port to repeat its journey forever.

The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33, and so on. If you are there when the bus departs, you can ride that bus to the airport!

Your notes (your puzzle input) consist of two lines. The first line is your estimate of the earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in service according to the shuttle company; entries that show x must be out of service, so you decide to ignore them.

To save time once you arrive, your goal is to figure out the earliest bus you can take to the airport. (There will be exactly one such bus.)

For example, suppose you have the following notes:

939
7,13,x,x,59,x,31,19

Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59, 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:

time   bus 7   bus 13  bus 59  bus 31  bus 19
929      .       .       .       .       .
930      .       .       .       D       .
931      D       .       .       .       D
932      .       .       .       .       .
933      .       .       .       .       .
934      .       .       .       .       .
935      .       .       .       .       .
936      .       D       .       .       .
937      .       .       .       .       .
938      D       .       .       .       .
939      .       .       .       .       .
940      .       .       .       .       .
941      .       .       .       .       .
942      .       .       .       .       .
943      .       .       .       .       .
944      .       .       D       .       .
945      D       .       .       .       .
946      .       .       .       .       .
947      .       .       .       .       .
948      .       .       .       .       .
949      .       D       .       .       .

The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number of minutes you'd need to wait gives 295.

What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?

--- Part Two ---

The shuttle company is running a contest: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)

For example, suppose you have the same list of bus IDs as above:

7,13,x,x,59,x,31,19

An x in the schedule means there are no constraints on what bus IDs must depart at that time.

This means you are looking for the earliest timestamp (called t) such that:

    Bus ID 7 departs at timestamp t.
    Bus ID 13 departs one minute after timestamp t.
    There are no requirements or restrictions on departures at two or three minutes after timestamp t.
    Bus ID 59 departs four minutes after timestamp t.
    There are no requirements or restrictions on departures at five minutes after timestamp t.
    Bus ID 31 departs six minutes after timestamp t.
    Bus ID 19 departs seven minutes after timestamp t.

The only bus departures that matter are the listed bus IDs at their specific offsets from t. Those bus IDs can depart at other times, and other bus IDs can depart at those times. For example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes after timestamp t.

In this example, the earliest timestamp at which this occurs is 1068781:

time     bus 7   bus 13  bus 59  bus 31  bus 19
1068773    .       .       .       .       .
1068774    D       .       .       .       .
1068775    .       .       .       .       .
1068776    .       .       .       .       .
1068777    .       .       .       .       .
1068778    .       .       .       .       .
1068779    .       .       .       .       .
1068780    .       .       .       .       .
1068781    D       .       .       .       .
1068782    .       D       .       .       .
1068783    .       .       .       .       .
1068784    .       .       .       .       .
1068785    .       .       D       .       .
1068786    .       .       .       .       .
1068787    .       .       .       D       .
1068788    D       .       .       .       D
1068789    .       .       .       .       .
1068790    .       .       .       .       .
1068791    .       .       .       .       .
1068792    .       .       .       .       .
1068793    .       .       .       .       .
1068794    .       .       .       .       .
1068795    D       D       .       .       .
1068796    .       .       .       .       .
1068797    .       .       .       .       .

In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is fine; the only requirement on that minute is that bus ID 19 departs then, and it does.

Here are some other examples:

    The earliest timestamp that matches the list 17,x,13,19 is 3417.
    67,7,59,61 first occurs at timestamp 754018.
    67,x,7,59,61 first occurs at timestamp 779210.
    67,7,x,59,61 first occurs at timestamp 1261476.
    1789,37,47,1889 first occurs at timestamp 1202161486.

However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!

What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?

*/
use std::io;
use std::io::prelude::*;

// extended euclidian algo for computign bezout coeff
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r): (i128, i128) = (a, b);
    let (mut old_s, mut s) = (1i128, 0i128);
    let (mut old_t, mut t) = (0i128, 1i128);

    while r != 0 {
        let q = old_r / r;
        let (n_old_r, n_r) = (r, old_r - q*r);
        let (n_old_s, n_s) = (s, old_s - q*s);
        let (n_old_t, n_t) = (t, old_t - q*t);
        old_r = n_old_r;
        r = n_r;
        old_s = n_old_s;
        s = n_s;
        old_t = n_old_t;
        t = n_t;
    }

    return (old_r, old_s, old_t);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let v: Vec<&str> = buffer.split("\n").collect();
    let departure = v[0].parse::<i128>().unwrap();
    let mut buses: Vec<i128> = v[1].split(",").filter_map(|s| s.parse::<i128>().ok()).collect();
    buses.sort();
    //println!("{} {} {:?}", n, departure, buses);

    let (mut min, mut min_bus) = (departure, -1);
    for bus in buses {
        let distance = bus - (departure % bus);
        if distance <= min {
            min = distance;
            min_bus = bus;
        }
        //println!("{}: {}", bus, distance);
    }

    //println!("{} {}", min_bus, min);
    println!("{}", min*min_bus);
    
    let aux: Vec<i128> =  v[1].split(",").map(|s| s.parse::<i128>().unwrap_or(-1)).collect();
    //println!("{:?}", aux);

    let mut new_buses = Vec::new();
    for i in 0..aux.len() {
        if aux[i] != -1 {
            let mut ai = (aux[i] - i as i128) % aux[i];
            if ai < 0 {
                ai = aux[i] + ai;
            }
            //println!("t = {} (mod {})", ai, aux[i]);
            new_buses.push((ai, aux[i]));
        }
    }
    //println!("{:?}", new_buses);

    //to compute second part, we need to use chinese remainder theorem and
    //run the construction factor using the bezout coeff pair by pair
    //we could make it even faster by paralelizing in multiple threads
    //but its not needed for this problem
    while new_buses.len() > 1 {
        let len = new_buses.len();
        for _ in 0..(len/2) {
            let (a1, n1) = new_buses[0];
            let (a2, n2) = new_buses[1];
            let (_, m1, m2) = extended_gcd(n1, n2);
            //println!("{} x {} + {} x {} = 1", m1, n1, m2, n2);
            let xi = a2*m1*n1 + a1*m2*n2;
            let mut si = xi % (n1*n2);
            if si < 0 {
                si = n1*n2 + si;
            }
            //println!("{} x {} x {} + {} x {} x {} = {}", a2, m1, n1, a1, m2, n2, xi);
            //println!("{} (mod {})", si, n1*n2);
            new_buses.push((si, n1*n2));
            // remove the first two elements
            new_buses.remove(0);
            new_buses.remove(0);
        }
        //println!("{:?}", new_buses);
    }
    let (k, _) = new_buses[0];
    println!("{}", k);
}
