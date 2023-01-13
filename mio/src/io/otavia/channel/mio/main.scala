package io.otavia.channel.mio

import io.netty5.buffer.{Buffer, BufferAllocator}

import java.nio.charset.Charset

@main def main(): Unit = {
  val allocator = BufferAllocator.offHeapPooled()

  val buffer = allocator.allocate(1024 * 8)

  val javaJNI = new JavaJNI()

  val len = JavaJNI.writeBuffer(buffer)
  val str = buffer.readCharSequence(len, Charset.forName("utf-8"))


  val poll = Poll.createPoll()
  val server = TcpListener.bind("127.0.0.1:13265")

  poll.register(server, Interest.READABLE)


  while (true) {
    poll.select()
    val events = poll.selectedKeys()

    while (events.hasNext) {
      val event = events.next()
      val socket = event.socket
      socket match
        case server: TcpListener => if (event.isReadable) {
          val client = server.accept()
          poll.register(client, Interest.READABLE)
          println("main: server accept")
        }
        case client: TcpStream => if (event.isReadable) {
          println("main: client event")
        }

    }


  }
}