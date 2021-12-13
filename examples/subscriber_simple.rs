/*
 * @Author       : sunzhifeng <ian.sun@auodigitech.com>
 * @Date         : 2021-08-25 19:19:53
 * @LastEditors  : ian <lauer3912@gmail.com>
 * @LastEditTime : 2021-12-13 21:19:07
 * @FilePath     : /iceoryx-rs/examples/subscriber_simple.rs
 * @Description  : Created by sunzhifeng, Please coding something here
 */
// SPDX-License-Identifier: Apache-2.0

use iceoryx_rs::sb::{SubscribeState, TopicBuilder};
use iceoryx_rs::Runtime;
use iceoryx_rs::print_build_info;

use std::thread;
use std::time::Duration;

#[repr(C)]
struct CounterTopic {
    counter: u32,
}

fn main() {
    // 输出构建信息
    print_build_info();

    // 创建一个运行时
    Runtime::init("subscriber_simple");

    let topic = TopicBuilder::<CounterTopic>::new(
        "service.name.Radar",        // Service Name, 服务名称
        "service.intance.FrontLeft", // service intance, 服务实例名称
        "envent.name.Counter"        // event name, 事件名称
    ).queue_capacity(5)
    .build();

    let (subscriber, sample_receive_token) = topic.subscribe();

    let mut has_printed_waiting_for_subscription = false;
    while subscriber.subscription_state() != SubscribeState::Subscribed {
        if !has_printed_waiting_for_subscription {
            println!("waiting for subscription ...");
            has_printed_waiting_for_subscription = true;
        }
        thread::sleep(Duration::from_millis(10));
    }

    if has_printed_waiting_for_subscription {
        println!("  -> subscribed");
    }

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    loop {
        if sample_receiver.has_samples() {
            while let Some(sample) = sample_receiver.get_sample() {
                println!("Receiving: {}", sample.counter);
            }
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
