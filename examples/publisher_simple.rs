/*
 * @Author       : sunzhifeng <ian.sun@auodigitech.com>
 * @Date         : 2021-08-25 19:19:53
 * @LastEditors  : sunzhifeng <ian.sun@auodigitech.com>
 * @LastEditTime : 2021-12-10 21:49:02
 * @FilePath     : /iceoryx-rs/examples/publisher_simple.rs
 * @Description  : Created by sunzhifeng, Please coding something here
 */
// SPDX-License-Identifier: Apache-2.0

use iceoryx_rs::pb::{TopicBuilder, POD};
use iceoryx_rs::Runtime;
use iceoryx_rs::print_build_info;

use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

unsafe impl POD for CounterTopic {}

fn main() -> Result<(), Box<dyn Error>> {
    print_build_info();
    Runtime::init("publisher_simple");

    let topic = TopicBuilder::<CounterTopic>::new("Radar", "FrontLeft", "Counter").build()?;

    let publisher = topic.offer();

    // wait until RouDi runs the discovery loop
    while !publisher.is_offered() {
        thread::sleep(Duration::from_millis(10));
    }

    let mut counter = 0u32;
    loop {
        let mut sample = publisher.allocate_sample()?;
        sample.counter = counter;
        publisher.publish(sample);

        println!("Sending: {}", counter);
        counter += 1;

        thread::sleep(Duration::from_millis(1000));
    }
}
