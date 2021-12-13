/*
 * @Author       : ian <lauer3912@gmail.com>
 * @Date         : 2021-08-25 19:19:53
 * @LastEditors  : ian <lauer3912@gmail.com>
 * @LastEditTime : 2021-12-13 21:18:42
 * @FilePath     : /iceoryx-rs/examples/publisher_simple.rs
 * @Description  : 消息发布者
 */

// SPDX-License-Identifier: Apache-2.0

use iceoryx_rs::pb::{TopicBuilder, POD};
use iceoryx_rs::Runtime;
use iceoryx_rs::print_build_info;

use std::error::Error;
use std::thread;
use std::time::Duration;

// 这告诉编译器"像C那样对类型布局"，可使用在结构体，枚举和联合类型。
#[repr(C)]
struct CounterTopic {
    counter: u32,
}

// 实现 POD 接口
unsafe impl POD for CounterTopic {}

fn main() -> Result<(), Box<dyn Error>> {
    // 输出构建信息
    print_build_info();

    // 创建一个运行时
    Runtime::init("publisher_simple");

    // 创建一个消息主题, SoA 模型
    let topic = TopicBuilder::<CounterTopic>::new(
        "service.name.Radar",        // Service Name, 服务名称
        "service.intance.FrontLeft", // service intance, 服务实例名称
        "envent.name.Counter"        // event name, 事件名称
    ).build()?;

    // 创建一个消息发布者
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
