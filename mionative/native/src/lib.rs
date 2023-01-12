use std::io::Write;
use std::ptr;
use std::ptr::slice_from_raw_parts_mut;
#[warn(unused_imports)]
#[warn(unused_variables)]
use std::time::Duration;

use jni::JNIEnv;
use jni::objects::*;
use jni::sys::{jboolean, jclass, jint, jlong, jobject};
use mio::{Events, Interest, Poll, Token};
use mio::event::{Event, Iter};
use mio::net::{TcpListener, TcpStream};

// JNI for scala class io.otavia.channel.mio.Event
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Event_token0(
    env: JNIEnv, this: jobject, event_raw: jlong) -> jint {
    let event = Box::from_raw(event_raw as *mut Option<&Event>);
    event.unwrap().token().0 as jint
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Event_isReadable0(
    env: JNIEnv, this: jobject, event_raw: jlong) -> jboolean {
    let event = Box::from_raw(event_raw as *mut Option<&Event>);
    event.unwrap().is_readable() as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Event_isWritable0(
    env: JNIEnv, this: jobject, event_raw: jlong) -> jboolean {
    let event = Box::from_raw(event_raw as *mut Option<&Event>);
    event.unwrap().is_writable() as jboolean
}

// JNI for scala class io.otavia.channel.mio.Events
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Events_next0(
    env: JNIEnv, this: jobject, iter_raw: jlong) -> jlong {
    let mut iter = Box::from_raw(iter_raw as *mut Iter);
    let next = Box::new(iter.next());
    Box::into_raw(next) as jlong
}

// JNI for scala object io.otavia.channel.mio.Events$
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Events_00024_openEvents(
    env: JNIEnv, this: jobject, capacity: jint) -> jlong {
    let events = Box::new(Events::with_capacity(capacity as usize));
    Box::into_raw(events) as jlong
}

// JNI for scala class io.otavia.channel.mio.Poll
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Poll_register0(
    env: JNIEnv, this: jobject, poll_raw: jlong, socket_raw: jlong,
    id: jint, interest: jint, tp: jint) {
    let inter = match interest {
        1 => Interest::READABLE,
        2 => Interest::WRITABLE,
        _ => panic!("not support")
    };
    let poll = Box::from_raw(poll_raw as *mut Poll);
    match tp {
        0 => {
            let mut socket = Box::from_raw(socket_raw as *mut TcpListener);
            poll.registry()
                .register(&mut socket, Token(id as usize), inter)
                .unwrap();
        }
        1 => {
            let mut socket = Box::from_raw(socket_raw as *mut TcpStream);
            poll.registry()
                .register(&mut socket, Token(id as usize), inter)
                .unwrap();
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_otavia_channel_mio_Poll_select0(
    env: JNIEnv, this: jobject, poll_raw: jlong, events_raw: jlong, secs: jlong, nanos: jlong)
    -> jlong {
    let mut poll = Box::from_raw(poll_raw as *mut Poll);
    let mut events = Box::from_raw(events_raw as *mut Events);
    println!("events: {:?} with capacity: {}", events, events.capacity());
    let timeout = if secs == 0 && nanos == 0 { None } else { Some(Duration::new(secs as u64, nanos as u32)) };
    println!("timeout: {:?}", timeout);
    let res = poll.poll(&mut events, timeout);
    match res {
        Ok(_) => {
            println!("poll success!");
            let iter = Box::new(events.iter());
            Box::into_raw(iter) as jlong
        }
        Err(e) => {
            // println!("Err: {}", e);
            panic!("Err: {}", e)
        }
    }
}


// JNI for scala object io.otavia.channel.mio.Poll$
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Poll_00024_openPoll(
    env: JNIEnv, this: jobject) -> jlong {
    let poll = Box::new(Poll::new().unwrap());
    Box::into_raw(poll) as jlong
}

// JNI for scala class io.otavia.channel.mio.TcpListener
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_TcpListener_accept0(
    env: JNIEnv, this: jobject, server_raw: jlong) -> jlong {
    let server = Box::from_raw(server_raw as *mut TcpListener);
    let client = Box::new(server.accept().unwrap());
    Box::into_raw(client) as jlong
}


// JNI for scala object io.otavia.channel.mio.TcpListener$
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_TcpListener_00024_bind0(
    env: JNIEnv, this: jobject, addr: JString) -> jlong {
    let address = env.get_string(addr).unwrap().to_string_lossy().to_string();
    let server = Box::new(TcpListener::bind(address.parse().unwrap()).unwrap());
    Box::into_raw(server) as jlong
}


// JNI for java class io.otavia.channel.mio.JavaJNI

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_JavaJNI_add(
    env: JNIEnv, clz: jclass, a: jint, b: jint) -> jint {
    println!("at Java_io_otavia_channel_mio_JavaJNI_add");
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_JavaJNI_plus(
    env: JNIEnv, this: jobject, term: jint) {
    println!("at Java_io_otavia_channel_mio_JavaJNI_plus");
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_JavaJNI_write0(
    env: JNIEnv, clz: jclass, buffer_address: jlong, writable: jint) -> jint {
    let mut buffer = std::slice::from_raw_parts_mut(buffer_address as *mut u8, writable as usize);
    let data = "hello world!";

    buffer.write(data.as_bytes()).unwrap();

    println!("{:?}", buffer);

    data.as_bytes().len() as jint
}