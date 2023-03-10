use std::io::{Read, Write};
use std::net::SocketAddr;
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
    let ret = event.unwrap().token().0 as jint;
    println!("token id is {}", ret);
    Box::into_raw(event);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Event_isReadable0(
    env: JNIEnv, this: jobject, event_raw: jlong) -> jboolean {
    let event = Box::from_raw(event_raw as *mut Option<&Event>);
    let ret = event.unwrap().is_readable() as jboolean;
    Box::into_raw(event);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Event_isWritable0(
    env: JNIEnv, this: jobject, event_raw: jlong) -> jboolean {
    let event = Box::from_raw(event_raw as *mut Option<&Event>);
    let ret = event.unwrap().is_writable() as jboolean;
    Box::into_raw(event);
    ret
}

// JNI for scala class io.otavia.channel.mio.Events
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Events_next0(
    env: JNIEnv, this: jobject, iter_raw: jlong) -> jlong {
    let mut iter = Box::from_raw(iter_raw as *mut Iter);
    let next = Box::new(iter.next());
    let ret = Box::into_raw(next) as jlong;
    Box::into_raw(iter);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_Events_optIsEmpty0(
    env: JNIEnv, this: jobject, opt_raw: jlong) -> jboolean {
    let next = Box::from_raw(opt_raw as *mut Option<&Event>);

    let ret = next.is_none() as jboolean;
    Box::into_raw(next);

    ret
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
    println!("register0 id is {}", id);
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
            Box::into_raw(socket);
        }
        1 => {
            let mut socket = Box::from_raw(socket_raw as *mut TcpStream);
            poll.registry()
                .register(&mut socket, Token(id as usize), inter)
                .unwrap();
            Box::into_raw(socket);
        }
        _ => {}
    };
    Box::into_raw(poll);
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_otavia_channel_mio_Poll_select0(
    env: JNIEnv, this: jobject, poll_raw: jlong, events_raw: jlong, secs: jlong, nanos: jlong)
    -> jlong {
    let mut poll = Box::from_raw(poll_raw as *mut Poll);
    let mut events = Box::from_raw(events_raw as *mut Events);
    let timeout = if secs == 0 && nanos == 0 { None } else { Some(Duration::new(secs as u64, nanos as u32)) };
    let res = poll.poll(&mut events, timeout);
    let mut ret_raw: jlong = -1;
    match res {
        Ok(_) => {
            println!("poll success ============================================");
            println!("events: {:?} with capacity: {}", events, events.capacity());
            let iter = Box::new(events.iter());
            ret_raw = Box::into_raw(iter) as jlong
        }
        Err(e) => {
            // println!("Err: {}", e);
            panic!("Err: {}", e)
        }
    };
    Box::into_raw(poll);
    Box::into_raw(events);
    ret_raw
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
    println!("server accepted client on {:?}", client.as_ref());
    let ret = Box::into_raw(client) as jlong;

    Box::into_raw(server);

    ret
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

// JNI for scala object io.otavia.channel.mio.TcpStream
//
#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_TcpStream_read0(
    env: JNIEnv, this: jobject, client_raw: jlong, buffer_address: jlong, writable: jint) -> jint {
    let mut client = Box::from_raw(client_raw as *mut (TcpStream, SocketAddr));
    let mut buffer = std::slice::from_raw_parts_mut(buffer_address as *mut u8, writable as usize);
    let (socket, addr) = client.as_mut();

    let read = socket.read(buffer).unwrap();

    Box::into_raw(client);
    buffer as *const [u8];

    read as jint
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_otavia_channel_mio_TcpStream_write0(
    env: JNIEnv, this: jobject, client_raw: jlong, buffer_address: jlong, readable: jint) -> jint {
    let mut client = Box::from_raw(client_raw as *mut (TcpStream, SocketAddr));
    let mut buffer = std::slice::from_raw_parts(buffer_address as *const u8, readable as usize);
    let (socket, addr) = client.as_mut();

    let write = socket.write(buffer).unwrap();

    Box::into_raw(client);
    buffer as *const [u8];

    write as jint
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


    data.as_bytes().len() as jint
}