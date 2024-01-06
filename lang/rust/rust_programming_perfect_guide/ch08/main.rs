fn main() {
    // 8.1
    let data = (10000000, 183.19, 'Q');
    let copy_of_data = data;
    println!("{}, {}, {}",
        data.0, copy_of_data.1, data.2);

    let _data: (i32, f64, char) = (10000000, 183.19, 'Q');

    let mut data = (10000000, 183.19, 'Q');
    data.0 = -5;
    data.2 = 'x';
    println!("{}, {}, {}", data.0, data.1, data.2);

    // let array = [12, 13, 14];
    // let tuple = (12, 13, 14);
    // let i = 0;
    // println!("{}", array[i]);
    // println!("{}", tuple.i); // compile error

    // 8.2
    let data = (10, 'x', 12, 183.19, 'Q', false, -9);
    println!("{}", data.2 + data.6);

    // let data1 = (10, 'x', 12, 183.19, 'Q', false, -9);
    // let mut data2: (u16, char, i16, f64, bool, char, i16);
    // data2 = data1; // compile error

    let data = ("first", 10, 'x', 12, 183.19, 'Q', false, -9);
    println!("{}", data.3 + data.7);

    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8; 5],
    }
    let data = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    println!("{}, {}, {}, {}",
        data.five_bytes[3], data.integer,
        data.fractional, data.character);

    struct SomeData2 {
        integer: i32,
        fractional: f32,
    }
    let mut data2 = SomeData2 {
        integer: 10,
        fractional: 183.19,
    };
    data2.fractional = 8.2;
    println!("{}, {}", data2.integer, data2.fractional);

    struct NoData {}
    let _no_data = NoData {};

    // 8.3
    struct SomeData3 (
        i32,
        f32,
        char,
        [u8; 5],
    );
    let data3 = SomeData3 (
        10_000_000,
        183.19,
        'Q',
        [9, 0, 250, 60, 200],
    );
    println!("{}, {}, {}, {}",
        data3.2, data3.0,
        data3.1, data3.3[2]);

    // 8.4
    const MAXIMUM_POWER: u16 = 600;
    #[allow(dead_code)]
    enum VehicleKind {
        Motorcycle,
        Car,
        Truck,
    }
    #[allow(dead_code)]
    struct VehicleData {
        kind: VehicleKind,
        registration_year: u16,
        registration_month: u8,
        power: u16,
    }
    let vehicle = VehicleData {
        kind: VehicleKind::Car,
        registration_year: 2003,
        registration_month: 11,
        power: 120,
    };
    if vehicle.power > MAXIMUM_POWER {
        println!("Too powerful.");
    }
}